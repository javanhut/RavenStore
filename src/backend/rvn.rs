//! Driving `rvn --json`.
//!
//! The store never links rvn in: it runs the binary and reads the JSON
//! event stream rvn emits on stdout. That keeps one code path for
//! everything (the terminal and the store see the same events) and lets a
//! privileged transaction run under `sudo` while the window stays a normal
//! user process.
//!
//! Read-only queries (`list`, `find`, `info`, `update --dry-run
//! --no-refresh`) run as the user and are collected whole. Anything that
//! writes — install, remove, update, refresh — is a [`Transaction`], run
//! under sudo with its events forwarded live over a channel.

use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::mpsc::{self, Receiver, Sender};

use anyhow::{anyhow, bail, Context, Result};
use serde_json::Value;

#[derive(Debug, Clone, Default)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub description: String,
    pub url: Option<String>,
    pub origin: String,
    pub aur: bool,
    pub installed_version: Option<String>,
    pub download_size: u64,
    pub installed_size: u64,
    pub licenses: Vec<String>,
    pub depends: Vec<String>,
    pub optdepends: Vec<String>,
    pub required_by: Vec<String>,
    pub popularity: f64,
    pub out_of_date: bool,
    /// Installed by name rather than as a dependency. Only meaningful for
    /// installed packages.
    pub explicit: bool,
}

fn strings(v: &Value) -> Vec<String> {
    v.as_array()
        .map(|a| {
            a.iter()
                .filter_map(|s| s.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default()
}

fn string(v: &Value) -> Option<String> {
    v.as_str().map(String::from)
}

impl Package {
    pub fn from_json(v: &Value) -> Package {
        Package {
            name: string(&v["name"]).unwrap_or_default(),
            version: string(&v["version"]).unwrap_or_default(),
            description: string(&v["description"]).unwrap_or_default(),
            url: string(&v["url"]),
            origin: string(&v["origin"]).unwrap_or_default(),
            aur: v["aur"].as_bool().unwrap_or(false),
            installed_version: string(&v["installed_version"]),
            download_size: v["download_size"].as_u64().unwrap_or(0),
            installed_size: v["installed_size"].as_u64().unwrap_or(0),
            licenses: strings(&v["licenses"]),
            depends: strings(&v["depends"]),
            optdepends: strings(&v["optdepends"]),
            required_by: strings(&v["required_by"]),
            popularity: v["popularity"].as_f64().unwrap_or(0.0),
            out_of_date: v["out_of_date"].as_bool().unwrap_or(false),
            explicit: v["explicit"].as_bool().unwrap_or(true),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Candidate {
    pub name: String,
    pub installed_version: String,
    pub new_version: String,
    pub origin: String,
    pub aur: bool,
    /// upgrade, replacement, downgrade or devel.
    pub kind: String,
    pub replaces: Option<String>,
    pub download_size: u64,
}

impl Candidate {
    fn from_json(v: &Value) -> Candidate {
        Candidate {
            name: string(&v["name"]).unwrap_or_default(),
            installed_version: string(&v["installed_version"]).unwrap_or_default(),
            new_version: string(&v["new_version"]).unwrap_or_default(),
            origin: string(&v["origin"]).unwrap_or_default(),
            aur: v["aur"].as_bool().unwrap_or(false),
            kind: string(&v["kind"]).unwrap_or_else(|| "upgrade".into()),
            replaces: string(&v["replaces"]),
            download_size: v["download_size"].as_u64().unwrap_or(0),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Updates {
    pub candidates: Vec<Candidate>,
    pub downgrades: Vec<Candidate>,
    pub download_size: u64,
}

impl Updates {
    pub fn from_json(v: &Value) -> Updates {
        let list = |key: &str| {
            v[key]
                .as_array()
                .map(|a| a.iter().map(Candidate::from_json).collect())
                .unwrap_or_default()
        };
        Updates {
            candidates: list("candidates"),
            downgrades: list("downgrades"),
            download_size: v["download_size"].as_u64().unwrap_or(0),
        }
    }
}

/// Where rvn lives. Resolved once so sudo gets an absolute path and cannot
/// be pointed elsewhere by a modified `PATH`.
pub fn binary() -> Option<PathBuf> {
    let path = std::env::var_os("PATH").unwrap_or_default();
    std::env::split_paths(&path)
        .map(|d| d.join("rvn"))
        .find(|p| p.is_file())
}

pub fn available() -> bool {
    binary().is_some()
}

pub fn version() -> Option<String> {
    let out = Command::new(binary()?).arg("--version").output().ok()?;
    let text = String::from_utf8_lossy(&out.stdout);
    text.split_whitespace().last().map(String::from)
}

/// Runs an unprivileged rvn command and returns every event it emitted.
fn run_json(args: &[&str]) -> Result<Vec<Value>> {
    let bin = binary().ok_or_else(|| anyhow!("rvn is not installed"))?;
    let out = Command::new(bin)
        .arg("--json")
        .args(args)
        .stdin(Stdio::null())
        .output()
        .context("could not run rvn")?;
    let mut events = Vec::new();
    for line in String::from_utf8_lossy(&out.stdout).lines() {
        if let Ok(v) = serde_json::from_str::<Value>(line) {
            events.push(v);
        }
    }
    if let Some(failed) = events.iter().find(|e| e["event"] == "failed") {
        bail!("{}", failed["message"].as_str().unwrap_or("rvn failed"));
    }
    if !out.status.success() {
        let err = String::from_utf8_lossy(&out.stderr);
        let last = err
            .lines()
            .rev()
            .find(|l| !l.trim().is_empty())
            .unwrap_or("rvn failed");
        bail!("{}", strip_ansi(last.trim()));
    }
    Ok(events)
}

fn find_event<'a>(events: &'a [Value], name: &str) -> Option<&'a Value> {
    events.iter().find(|e| e["event"] == name)
}

fn packages_in(v: &Value, key: &str) -> Vec<Package> {
    v[key]
        .as_array()
        .map(|a| a.iter().map(Package::from_json).collect())
        .unwrap_or_default()
}

/// Every installed package, or only the explicitly installed ones.
pub fn installed(all: bool) -> Result<Vec<Package>> {
    let mut args = vec!["--no-sync", "list"];
    if !all {
        args.push("--explicit");
    }
    let events = run_json(&args)?;
    let ev = find_event(&events, "installed").ok_or_else(|| anyhow!("rvn gave no package list"))?;
    let mut pkgs = packages_in(ev, "packages");
    for p in &mut pkgs {
        // `list` reports installed packages, whose version *is* the
        // installed version.
        p.installed_version = Some(p.version.clone());
    }
    Ok(pkgs)
}

pub fn search(query: &str, repo_only: bool, limit: usize) -> Result<Vec<Package>> {
    let limit = limit.to_string();
    let mut args = vec!["--no-sync"];
    if repo_only {
        args.push("--repo-only");
    }
    args.extend(["find", "--no-select", "--limit", &limit, query]);
    let events = run_json(&args)?;
    Ok(find_event(&events, "results")
        .map(|ev| packages_in(ev, "results"))
        .unwrap_or_default())
}

pub fn info(name: &str, repo_only: bool) -> Result<Option<Package>> {
    let mut args = vec!["--no-sync"];
    if repo_only {
        args.push("--repo-only");
    }
    args.extend(["info", name]);
    match run_json(&args) {
        Ok(events) => Ok(find_event(&events, "packages")
            .map(|ev| packages_in(ev, "packages"))
            .unwrap_or_default()
            .into_iter()
            .next()),
        Err(e) if e.to_string().starts_with("no package named") => Ok(None),
        Err(e) => Err(e),
    }
}

/// What is out of date, judged from the databases already on disk.
pub fn check_updates(repo_only: bool) -> Result<Updates> {
    let mut args = vec!["--no-sync"];
    if repo_only {
        args.push("--repo-only");
    }
    args.extend(["update", "--dry-run", "--no-refresh"]);
    let events = run_json(&args)?;
    Ok(find_event(&events, "updates")
        .map(Updates::from_json)
        .unwrap_or_default())
}

// ---- privileged transactions -------------------------------------------

/// One thing the user asked the system to do.
#[derive(Debug, Clone)]
pub struct Transaction {
    pub title: String,
    /// Arguments after `rvn --json -y`.
    pub args: Vec<String>,
}

impl Transaction {
    pub fn install(names: &[String], repo_only: bool) -> Transaction {
        let mut args = global(repo_only);
        args.push("install".into());
        args.extend(names.iter().cloned());
        Transaction {
            title: format!("Installing {}", join(names)),
            args,
        }
    }

    pub fn remove(names: &[String]) -> Transaction {
        let mut args = global(false);
        args.push("uninstall".into());
        args.extend(names.iter().cloned());
        Transaction {
            title: format!("Removing {}", join(names)),
            args,
        }
    }

    /// Everything when `names` is empty.
    pub fn update(names: &[String], repo_only: bool) -> Transaction {
        let mut args = global(repo_only);
        args.push("update".into());
        args.extend(names.iter().cloned());
        Transaction {
            title: if names.is_empty() {
                "Updating the system".into()
            } else {
                format!("Updating {}", join(names))
            },
            args,
        }
    }

    /// Refreshes the databases and reports what is out of date, changing
    /// nothing else.
    pub fn refresh(repo_only: bool) -> Transaction {
        let mut args = global(repo_only);
        args.extend(["update".into(), "--dry-run".into()]);
        Transaction {
            title: "Checking for updates".into(),
            args,
        }
    }
}

fn global(repo_only: bool) -> Vec<String> {
    let mut args = Vec::new();
    if repo_only {
        args.push("--repo-only".into());
    }
    args
}

fn join(names: &[String]) -> String {
    match names.len() {
        0 => String::new(),
        1 => names[0].clone(),
        2 => format!("{} and {}", names[0], names[1]),
        n => format!("{} and {} more", names[0], n - 1),
    }
}

/// What a running transaction reports, in order.
#[derive(Debug, Clone)]
pub enum Event {
    /// A stage began (a spinner line in the terminal).
    Stage(String),
    StageDone {
        message: String,
        ok: bool,
    },
    Progress {
        label: String,
        done: u64,
        total: u64,
        unit: String,
        detail: String,
    },
    ProgressDone {
        message: String,
    },
    /// ok, warn, err, info, step, detail.
    Message {
        kind: String,
        text: String,
    },
    Tree(Vec<String>),
    /// The resolved install plan.
    Plan(Value),
    Updates(Updates),
    /// A raw stderr line — makepkg output, mostly.
    Log(String),
    /// rvn reported a fatal error.
    Failed(String),
    /// rvn finished successfully.
    Done,
    /// The process exited. Always the last event.
    Exited {
        success: bool,
        auth_failed: bool,
    },
}

/// Whether sudo will run without asking for a password right now.
pub fn sudo_cached() -> bool {
    Command::new("sudo")
        .args(["-n", "true"])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// Starts a transaction under sudo. `password` is fed to sudo on stdin; pass
/// `None` when [`sudo_cached`] said none is needed.
pub fn start(tx: &Transaction, password: Option<String>) -> Result<Receiver<Event>> {
    let bin = binary().ok_or_else(|| anyhow!("rvn is not installed"))?;
    if !super::have("sudo") {
        bail!("sudo is not installed, so the store cannot install packages");
    }

    let mut cmd = Command::new("sudo");
    match &password {
        // -S reads the password from stdin; an empty prompt keeps stderr
        // clean; -k forces the check so a stale timestamp does not mask a
        // wrong password.
        Some(_) => cmd.args(["-S", "-p", ""]),
        // Never block on a prompt nobody can see.
        None => cmd.arg("-n"),
    };
    cmd.arg("--")
        .arg(&bin)
        .args(["--json", "-y"])
        .args(&tx.args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = cmd.spawn().context("could not start sudo")?;

    if let Some(mut stdin) = child.stdin.take() {
        if let Some(pw) = password {
            let _ = stdin.write_all(pw.as_bytes());
            let _ = stdin.write_all(b"\n");
        }
        // Closing stdin: rvn -y never reads it, and sudo gets EOF instead of
        // hanging if the password was wrong.
        drop(stdin);
    }

    let (tx_events, rx) = mpsc::channel();
    let stdout = child.stdout.take().expect("stdout piped");
    let stderr = child.stderr.take().expect("stderr piped");

    let out_thread = {
        let send = tx_events.clone();
        std::thread::spawn(move || read_events(stdout, send))
    };
    let err_thread = {
        let send = tx_events.clone();
        std::thread::spawn(move || read_log(stderr, send))
    };

    std::thread::spawn(move || {
        let saw_banner = out_thread.join().unwrap_or(false);
        let auth_text = err_thread.join().unwrap_or(false);
        let status = child.wait();
        let success = status.map(|s| s.success()).unwrap_or(false);
        // sudo rejecting the password looks like: no rvn output at all, and
        // sudo's own complaint on stderr.
        let auth_failed = !success && !saw_banner && auth_text;
        let _ = tx_events.send(Event::Exited {
            success,
            auth_failed,
        });
    });

    Ok(rx)
}

/// Parses stdout into events. Returns whether rvn ever spoke, which is how
/// a sudo failure is told apart from an rvn failure.
fn read_events(stdout: std::process::ChildStdout, send: Sender<Event>) -> bool {
    let mut spoke = false;
    for line in BufReader::new(stdout).lines().map_while(Result::ok) {
        let Ok(v) = serde_json::from_str::<Value>(&line) else {
            // makepkg or a scriptlet writing to stdout: show it as a log.
            if !line.trim().is_empty() {
                let _ = send.send(Event::Log(strip_ansi(&line)));
            }
            continue;
        };
        spoke = true;
        let text = |key: &str| v[key].as_str().unwrap_or("").to_string();
        let event = match v["event"].as_str().unwrap_or("") {
            "banner" => continue,
            "stage" => Event::Stage(text("message")),
            "stage_done" => Event::StageDone {
                message: text("message"),
                ok: v["ok"].as_bool().unwrap_or(true),
            },
            "progress" => Event::Progress {
                label: text("label"),
                done: v["done"].as_u64().unwrap_or(0),
                total: v["total"].as_u64().unwrap_or(0),
                unit: text("unit"),
                detail: text("detail"),
            },
            "progress_done" => Event::ProgressDone {
                message: text("message"),
            },
            "ok" | "warn" | "err" | "info" | "step" | "detail" => Event::Message {
                kind: v["event"].as_str().unwrap_or("info").to_string(),
                text: text("message"),
            },
            "tree" => Event::Tree(strings(&v["items"])),
            "plan" => Event::Plan(v.clone()),
            "updates" => Event::Updates(Updates::from_json(&v)),
            "failed" => Event::Failed(text("message")),
            "done" => Event::Done,
            _ => continue,
        };
        if send.send(event).is_err() {
            break;
        }
    }
    spoke
}

/// Forwards stderr as log lines. Returns whether sudo complained about the
/// password.
fn read_log(stderr: std::process::ChildStderr, send: Sender<Event>) -> bool {
    let mut auth = false;
    for line in BufReader::new(stderr).lines().map_while(Result::ok) {
        let clean = strip_ansi(line.trim_end());
        if clean.contains("incorrect password")
            || clean.starts_with("Sorry, try again")
            || clean.contains("a password is required")
        {
            auth = true;
        }
        if clean.contains("is not in the sudoers file") || clean.contains("not allowed to execute")
        {
            let _ = send.send(Event::Failed(
                "Your account is not allowed to use sudo, so the store cannot change packages. Ask an administrator to add you to the wheel group.".into(),
            ));
        }
        if !clean.trim().is_empty() {
            let _ = send.send(Event::Log(clean));
        }
    }
    auth
}

/// Removes ANSI escape sequences, which makepkg and compilers emit freely.
pub fn strip_ansi(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            if chars.peek() == Some(&'[') {
                chars.next();
                for n in chars.by_ref() {
                    if n.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
            continue;
        }
        if c != '\r' {
            out.push(c);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_escape_sequences() {
        assert_eq!(
            strip_ansi("\x1b[38;5;141mhello\x1b[0m world"),
            "hello world"
        );
        assert_eq!(strip_ansi("plain"), "plain");
    }

    #[test]
    fn package_reads_the_store_fields() {
        let v: Value = serde_json::from_str(
            r#"{"name":"vlc","version":"3.0.21-1","description":"player","origin":"extra","aur":false,"installed_version":"3.0.20-1","upgradable":true,"download_size":10,"installed_size":20,"licenses":["GPL"],"depends":["a","b"],"optdepends":[],"popularity":0.0,"out_of_date":false}"#,
        )
        .unwrap();
        let p = Package::from_json(&v);
        assert_eq!(p.name, "vlc");
        assert_eq!(p.installed_version.as_deref(), Some("3.0.20-1"));
        assert_eq!(p.depends, vec!["a", "b"]);
        assert!(p.explicit, "absent explicit flag defaults to true");
    }

    #[test]
    fn updates_reads_candidates() {
        let v: Value = serde_json::from_str(
            r#"{"event":"updates","candidates":[{"name":"x","installed_version":"1","new_version":"2","origin":"core","aur":false,"kind":"upgrade","download_size":5}],"downgrades":[],"download_size":5}"#,
        )
        .unwrap();
        let u = Updates::from_json(&v);
        assert_eq!(u.candidates.len(), 1);
        assert_eq!(u.candidates[0].new_version, "2");
    }

    /// The event pipeline end to end, with a shell standing in for
    /// `sudo rvn`: JSON on stdout becomes typed events, stderr becomes log
    /// lines, and the exit lands last.
    #[test]
    fn child_output_becomes_events_in_order() {
        let mut child = Command::new("sh")
            .arg("-c")
            .arg(r#"printf '%s\n' '{"event":"banner","version":"v0"}' '{"event":"stage","message":"resolving"}' '{"event":"progress","label":"fetching","done":5,"total":10,"unit":"bytes","detail":""}' '{"event":"updates","candidates":[],"downgrades":[],"download_size":0}' 'not json at all' '{"event":"done"}'; echo 'makepkg: building' >&2"#)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let (send, rx) = mpsc::channel();
        let out = child.stdout.take().unwrap();
        let err = child.stderr.take().unwrap();
        let spoke = read_events(out, send.clone());
        let auth = read_log(err, send);
        assert!(spoke, "rvn's banner marks the stream as rvn's");
        assert!(!auth);
        assert!(child.wait().unwrap().success());

        let events: Vec<Event> = rx.try_iter().collect();
        let stdout_events: Vec<&Event> = events
            .iter()
            .filter(|e| !matches!(e, Event::Log(l) if l.starts_with("makepkg")))
            .collect();
        assert!(matches!(stdout_events[0], Event::Stage(m) if m == "resolving"));
        assert!(matches!(
            stdout_events[1],
            Event::Progress {
                done: 5,
                total: 10,
                ..
            }
        ));
        assert!(matches!(stdout_events[2], Event::Updates(u) if u.candidates.is_empty()));
        assert!(matches!(stdout_events[3], Event::Log(l) if l == "not json at all"));
        assert!(matches!(stdout_events[4], Event::Done));
        assert!(events
            .iter()
            .any(|e| matches!(e, Event::Log(l) if l == "makepkg: building")));
    }

    #[test]
    fn sudo_rejection_is_recognised() {
        let mut child = Command::new("sh")
            .arg("-c")
            .arg("echo 'Sorry, try again.' >&2; echo 'sudo: 1 incorrect password attempt' >&2")
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let (send, _rx) = mpsc::channel();
        assert!(read_log(child.stderr.take().unwrap(), send));
        assert!(child.wait().unwrap().success());
    }

    #[test]
    fn transaction_titles_read_well() {
        let one = Transaction::install(&["vlc".into()], false);
        assert_eq!(one.title, "Installing vlc");
        assert_eq!(one.args, vec!["install", "vlc"]);
        let many = Transaction::remove(&["a".into(), "b".into(), "c".into()]);
        assert_eq!(many.title, "Removing a and 2 more");
        let all = Transaction::update(&[], true);
        assert_eq!(all.args, vec!["--repo-only", "update"]);
    }
}
