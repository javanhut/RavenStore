//! Small facts about the machine for the sidebar.

pub struct OsRelease {
    pub name: String,
    pub version_id: String,
}

pub fn os_release() -> OsRelease {
    let text = std::fs::read_to_string("/etc/os-release").unwrap_or_default();
    let get = |key: &str| {
        text.lines()
            .find_map(|l| l.strip_prefix(key).and_then(|r| r.strip_prefix('=')))
            .map(|v| v.trim_matches('"').to_string())
    };
    OsRelease {
        name: get("PRETTY_NAME")
            .or_else(|| get("NAME"))
            .unwrap_or_else(|| "Raven Linux".into()),
        version_id: get("VERSION_ID").unwrap_or_default(),
    }
}

/// `sudo rvn ...` in the user's terminal, for people who want to watch.
pub fn launch_in_terminal(terminal: &str, args: &[&str]) -> anyhow::Result<()> {
    let mut cmd: Vec<String> = vec!["sudo".into(), "rvn".into()];
    cmd.extend(args.iter().map(|a| a.to_string()));
    let script = format!(
        "{}; echo; echo 'Done. Press Enter to close.'; read _",
        cmd.iter()
            .map(|c| format!("'{}'", c.replace('\'', "'\\''")))
            .collect::<Vec<_>>()
            .join(" ")
    );
    if super::have(terminal) {
        let spawned = std::process::Command::new(terminal)
            .args(["-e", "sh", "-c", &script])
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn();
        if spawned.is_ok() {
            return Ok(());
        }
    }
    anyhow::bail!("{terminal} could not be started")
}
