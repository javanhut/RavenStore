//! Running one rvn transaction with a live progress dialog.
//!
//! Flow: for a privileged transaction, check whether sudo needs a password
//! and ask if so, then start rvn under sudo; an unprivileged one (a
//! refresh-for-check) starts straight away as the user. Either way, drain
//! its event stream on the main loop and reload the store's view of the
//! system when it exits.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::{Receiver, TryRecvError};

use gtk4 as gtk;
use libadwaita as adw;
use libadwaita::prelude::*;

use super::{ask_password, spawn, App};
use crate::backend::human_bytes;
use crate::backend::rvn::{self, Event, Transaction};

struct View {
    dialog: adw::Dialog,
    stage: gtk::Label,
    spinner: gtk::Spinner,
    status_icon: gtk::Image,
    progress: gtk::ProgressBar,
    summary: gtk::Label,
    log: gtk::TextBuffer,
    log_view: gtk::TextView,
    close: gtk::Button,
    /// Names rvn resolved, for the toast at the end.
    touched: RefCell<Vec<String>>,
    /// Failure text, if rvn reported one.
    failure: RefCell<Option<String>>,
}

pub fn run(app: &Rc<App>, tx: Transaction) {
    if !tx.privileged {
        launch(app, tx, None, 0);
        return;
    }
    let app = app.clone();
    spawn(rvn::sudo_cached, move |cached| {
        if cached {
            launch(&app, tx, None, 0);
        } else {
            ask(&app, tx, 0, None);
        }
    });
}

fn ask(app: &Rc<App>, tx: Transaction, attempt: u32, complaint: Option<&str>) {
    let heading = "Authentication required";
    let body = match complaint {
        Some(c) => format!("{c}\n\n{} needs your password.", tx.title),
        None => format!("{} needs your password. Raven Store runs rvn with sudo, the same way the terminal does.", tx.title),
    };
    let app2 = app.clone();
    ask_password(&app.window(), heading, &body, move |answer| match answer {
        Some(pw) if !pw.is_empty() => launch(&app2, tx.clone(), Some(pw), attempt),
        _ => app2.toast("Cancelled"),
    });
}

fn launch(app: &Rc<App>, tx: Transaction, password: Option<String>, attempt: u32) {
    let rx = match rvn::start(&tx, password) {
        Ok(rx) => rx,
        Err(e) => {
            app.error("Could not start rvn", &e);
            return;
        }
    };
    app.busy.set(true);
    let view = Rc::new(build_view(app, &tx));
    view.dialog.present(Some(&app.window()));
    poll(app.clone(), tx, view, rx, attempt);
}

fn build_view(app: &Rc<App>, tx: &Transaction) -> View {
    let dialog = adw::Dialog::new();
    dialog.set_title(&tx.title);
    dialog.set_content_width(600);
    dialog.set_can_close(false);

    let toolbar = adw::ToolbarView::new();
    toolbar.add_top_bar(&adw::HeaderBar::new());
    let body = gtk::Box::new(gtk::Orientation::Vertical, 12);
    body.set_margin_start(20);
    body.set_margin_end(20);
    body.set_margin_top(4);
    body.set_margin_bottom(20);
    toolbar.set_content(Some(&body));
    dialog.set_child(Some(&toolbar));

    let row = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    let spinner = gtk::Spinner::new();
    spinner.start();
    spinner.set_size_request(22, 22);
    row.append(&spinner);
    let status_icon = gtk::Image::new();
    status_icon.set_pixel_size(22);
    status_icon.set_visible(false);
    row.append(&status_icon);
    let stage = gtk::Label::new(Some("Starting…"));
    stage.add_css_class("tx-stage");
    stage.set_xalign(0.0);
    stage.set_hexpand(true);
    stage.set_ellipsize(gtk::pango::EllipsizeMode::End);
    row.append(&stage);
    body.append(&row);

    let progress = gtk::ProgressBar::new();
    progress.set_show_text(true);
    progress.set_visible(false);
    body.append(&progress);

    let summary = gtk::Label::new(None);
    summary.add_css_class("dim");
    summary.set_xalign(0.0);
    summary.set_wrap(true);
    summary.set_visible(false);
    body.append(&summary);

    let log = gtk::TextBuffer::new(None);
    let log_view = gtk::TextView::with_buffer(&log);
    log_view.set_editable(false);
    log_view.set_cursor_visible(false);
    log_view.set_monospace(true);
    log_view.set_wrap_mode(gtk::WrapMode::WordChar);
    log_view.add_css_class("tx-log");
    log_view.set_left_margin(6);
    log_view.set_right_margin(6);
    let log_scroller = gtk::ScrolledWindow::builder()
        .child(&log_view)
        .min_content_height(220)
        .vexpand(true)
        .build();
    let expander = gtk::Expander::builder()
        .label("Details")
        .child(&log_scroller)
        .build();
    body.append(&expander);

    let buttons = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    buttons.set_halign(gtk::Align::End);
    let close = gtk::Button::with_label("Close");
    close.add_css_class("pill");
    close.set_sensitive(false);
    let d2 = dialog.clone();
    close.connect_clicked(move |_| {
        d2.set_can_close(true);
        d2.close();
    });
    buttons.append(&close);
    body.append(&buttons);

    let _ = app;
    View {
        dialog,
        stage,
        spinner,
        status_icon,
        progress,
        summary,
        log,
        log_view,
        close,
        touched: RefCell::new(Vec::new()),
        failure: RefCell::new(None),
    }
}

impl View {
    fn append_log(&self, line: &str) {
        let mut end = self.log.end_iter();
        self.log.insert(&mut end, line);
        self.log.insert(&mut end, "\n");
        let mark = self.log.create_mark(None, &self.log.end_iter(), false);
        self.log_view.scroll_mark_onscreen(&mark);
    }

    fn handle(&self, event: Event) {
        match event {
            Event::Stage(m) => {
                self.stage.set_text(&m);
                self.append_log(&format!("▸ {m}"));
            }
            Event::StageDone { message, ok } => {
                self.stage.set_text(&message);
                self.append_log(&format!("{} {message}", if ok { "✔" } else { "✖" }));
            }
            Event::Progress {
                label,
                done,
                total,
                unit,
                detail,
            } => {
                self.progress.set_visible(true);
                let frac = if total == 0 {
                    0.0
                } else {
                    (done as f64 / total as f64).clamp(0.0, 1.0)
                };
                self.progress.set_fraction(frac);
                let amount = if unit == "bytes" {
                    format!("{} / {}", human_bytes(done), human_bytes(total))
                } else {
                    format!("{done} / {total} {unit}")
                };
                self.progress
                    .set_text(Some(&format!("{label}  ·  {amount}")));
                if !detail.is_empty() {
                    self.stage.set_text(&format!("{label}: {detail}"));
                } else {
                    self.stage.set_text(&label);
                }
            }
            Event::ProgressDone { message } => {
                self.progress.set_fraction(1.0);
                self.stage.set_text(&message);
                self.append_log(&format!("✔ {message}"));
            }
            Event::Message { kind, text } => {
                let glyph = match kind.as_str() {
                    "ok" => "✔",
                    "warn" => "▲",
                    "err" => "✖",
                    "step" => "▸",
                    "detail" => " ",
                    _ => "•",
                };
                self.append_log(&format!("{glyph} {text}"));
                if kind == "err" || kind == "warn" {
                    self.summary.set_visible(true);
                    self.summary.set_text(&text);
                }
                if kind == "step" {
                    self.stage.set_text(&text);
                }
            }
            Event::Tree(items) => {
                for (i, item) in items.iter().enumerate() {
                    let branch = if i + 1 == items.len() {
                        "└─"
                    } else {
                        "├─"
                    };
                    self.append_log(&format!("   {branch} {item}"));
                }
            }
            Event::Plan(plan) => {
                let names: Vec<String> = plan["install"]
                    .as_array()
                    .map(|a| {
                        a.iter()
                            .filter_map(|p| p["name"].as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();
                let size = plan["download_size"].as_u64().unwrap_or(0);
                let build = plan["build_from_source"].as_u64().unwrap_or(0);
                let mut s = format!(
                    "{} package{} · download {}",
                    names.len(),
                    if names.len() == 1 { "" } else { "s" },
                    human_bytes(size)
                );
                if build > 0 {
                    s.push_str(&format!(" · {build} built from source"));
                }
                self.summary.set_text(&s);
                self.summary.set_visible(true);
                *self.touched.borrow_mut() = names;
            }
            Event::Updates(u) => {
                let n = u.candidates.len();
                self.summary.set_text(&format!(
                    "{n} update{} · download {}",
                    if n == 1 { "" } else { "s" },
                    human_bytes(u.download_size)
                ));
                self.summary.set_visible(true);
                *self.touched.borrow_mut() = u.candidates.iter().map(|c| c.name.clone()).collect();
            }
            Event::Log(line) => self.append_log(&line),
            Event::Failed(m) => {
                *self.failure.borrow_mut() = Some(m.clone());
                self.append_log(&format!("✖ {m}"));
            }
            Event::Done => {}
            Event::Exited { .. } => {}
        }
    }

    fn finish(&self, success: bool, message: &str) {
        self.spinner.stop();
        self.spinner.set_visible(false);
        self.status_icon.set_visible(true);
        self.status_icon.set_icon_name(Some(if success {
            "object-select-symbolic"
        } else {
            "dialog-error-symbolic"
        }));
        self.status_icon
            .add_css_class(if success { "success" } else { "error" });
        self.stage.set_text(message);
        self.progress.set_visible(false);
        self.close.set_sensitive(true);
        self.close.add_css_class("suggested-action");
        self.dialog.set_can_close(true);
    }
}

fn poll(app: Rc<App>, tx: Transaction, view: Rc<View>, rx: Receiver<Event>, attempt: u32) {
    glib::timeout_add_local(std::time::Duration::from_millis(40), move || loop {
        match rx.try_recv() {
            Ok(Event::Exited {
                success,
                auth_failed,
            }) => {
                app.busy.set(false);
                if auth_failed {
                    view.dialog.set_can_close(true);
                    view.dialog.close();
                    if attempt + 1 >= 3 {
                        app.toast("Too many failed password attempts");
                    } else {
                        ask(
                            &app,
                            tx.clone(),
                            attempt + 1,
                            Some("That password was not accepted."),
                        );
                    }
                    return glib::ControlFlow::Break;
                }
                let failure = view.failure.borrow().clone();
                if success && failure.is_none() {
                    let done = match tx.args.first().map(String::as_str) {
                        Some("install") => "Installed",
                        Some("uninstall") => "Removed",
                        _ if tx.args.iter().any(|a| a == "--dry-run") => "Checked for updates",
                        _ => "Updated",
                    };
                    view.finish(true, &format!("{done} successfully"));
                    app.toast(&format!("{}: done", tx.title));
                } else {
                    let why =
                        failure.unwrap_or_else(|| "rvn exited with an error — see Details".into());
                    view.finish(false, &why);
                }
                app.reload();
                return glib::ControlFlow::Break;
            }
            Ok(event) => view.handle(event),
            Err(TryRecvError::Empty) => return glib::ControlFlow::Continue,
            Err(TryRecvError::Disconnected) => {
                app.busy.set(false);
                view.finish(false, "rvn stopped without reporting");
                app.reload();
                return glib::ControlFlow::Break;
            }
        }
    });
}
