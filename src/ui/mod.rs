//! The GTK side. Everything here runs on the main thread; work that blocks
//! (running rvn, scanning desktop files) goes through [`spawn`] and comes
//! back through a closure on the main loop.

pub mod detail;
pub mod pages;
pub mod theme;
pub mod transaction;
pub mod widgets;
pub mod window;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use gtk4 as gtk;
use libadwaita as adw;
use libadwaita::prelude::*;

use crate::backend::apps::{AppIndex, LaunchableApp};
use crate::backend::rvn::{self, Package, Transaction, Updates};
use crate::config::{Desktop, StoreConfig};

pub const APP_ID: &str = "com.ravenstore.Raven";

type Listener = Box<dyn Fn(&Rc<App>)>;

/// What the store currently knows about the system. Refreshed after every
/// transaction and on demand.
#[derive(Default)]
pub struct State {
    pub installed: HashMap<String, Package>,
    pub updates: Updates,
    /// Whether an update check has completed since launch.
    pub checked: bool,
    pub loading: bool,
    pub apps: Option<AppIndex>,
    /// Set when the last reload failed, e.g. rvn missing.
    pub error: Option<String>,
}

/// What a card should offer for a package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    NotInstalled,
    Installed { version: String },
    Updatable { from: String, to: String },
}

pub struct App {
    pub desktop: Desktop,
    pub config: RefCell<StoreConfig>,
    pub state: RefCell<State>,
    pub toasts: adw::ToastOverlay,
    window: RefCell<Option<adw::ApplicationWindow>>,
    listeners: RefCell<Vec<Listener>>,
    nav: RefCell<Option<(gtk::ListBox, gtk::Stack)>>,
    /// A transaction is running; a second one must wait.
    pub busy: std::cell::Cell<bool>,
}

impl App {
    pub fn window(&self) -> adw::ApplicationWindow {
        self.window.borrow().clone().expect("window not built yet")
    }

    pub fn toast(&self, text: &str) {
        self.toasts.add_toast(adw::Toast::new(text));
    }

    pub fn error(&self, context: &str, err: &anyhow::Error) {
        tracing::warn!("{context}: {err:#}");
        let t = adw::Toast::new(&format!("{context}: {err}"));
        t.set_timeout(6);
        self.toasts.add_toast(t);
    }

    pub fn on_change(&self, f: impl Fn(&Rc<App>) + 'static) {
        self.listeners.borrow_mut().push(Box::new(f));
    }

    pub fn notify(self: &Rc<Self>) {
        for l in self.listeners.borrow().iter() {
            l(self);
        }
    }

    pub fn status(&self, name: &str) -> Status {
        let st = self.state.borrow();
        if let Some(c) = st.updates.candidates.iter().find(|c| c.name == name) {
            return Status::Updatable {
                from: c.installed_version.clone(),
                to: c.new_version.clone(),
            };
        }
        match st.installed.get(name) {
            Some(p) => Status::Installed {
                version: p.version.clone(),
            },
            None => Status::NotInstalled,
        }
    }

    pub fn is_installed(&self, name: &str) -> bool {
        self.state.borrow().installed.contains_key(name)
    }

    pub fn launchable(&self, name: &str) -> Option<LaunchableApp> {
        if !self.is_installed(name) {
            return None;
        }
        self.state
            .borrow()
            .apps
            .as_ref()
            .and_then(|i| i.for_package(name).cloned())
    }

    pub fn update_count(&self) -> Option<usize> {
        let st = self.state.borrow();
        st.checked.then_some(st.updates.candidates.len())
    }

    pub fn repo_only(&self) -> bool {
        self.config.borrow().repo_only
    }

    /// Re-read what is installed and what is out of date.
    pub fn reload(self: &Rc<Self>) {
        {
            let mut st = self.state.borrow_mut();
            if st.loading {
                return;
            }
            st.loading = true;
        }
        self.notify();
        let repo_only = self.repo_only();
        let app = self.clone();
        spawn(
            move || {
                let installed = rvn::installed(true);
                let updates = rvn::check_updates(repo_only);
                let apps = AppIndex::scan();
                (installed, updates, apps)
            },
            move |(installed, updates, apps)| {
                {
                    let mut st = app.state.borrow_mut();
                    st.loading = false;
                    st.apps = Some(apps);
                    st.error = None;
                    match installed {
                        Ok(list) => {
                            st.installed = list.into_iter().map(|p| (p.name.clone(), p)).collect();
                        }
                        Err(e) => st.error = Some(e.to_string()),
                    }
                    match updates {
                        Ok(u) => {
                            st.updates = u;
                            st.checked = true;
                        }
                        Err(e) => {
                            if st.error.is_none() {
                                st.error = Some(e.to_string());
                            }
                        }
                    }
                }
                app.notify();
            },
        );
    }

    pub fn navigate(&self, id: &str) {
        if let Some((nav, stack)) = self.nav.borrow().as_ref() {
            let ids = pages::ids();
            if let Some(i) = ids.iter().position(|p| *p == id) {
                nav.select_row(nav.row_at_index(i as i32).as_ref());
            } else {
                nav.unselect_all();
            }
            if stack.child_by_name(id).is_some() {
                stack.set_visible_child_name(id);
            }
        }
    }

    pub fn current_page(&self) -> Option<String> {
        self.nav
            .borrow()
            .as_ref()
            .and_then(|(_, s)| s.visible_child_name())
            .map(|n| n.to_string())
    }

    pub fn save_config(&self) {
        if let Err(e) = self.config.borrow().save() {
            self.error("Could not save preferences", &e);
        }
    }

    pub fn toggle_wish(self: &Rc<Self>, name: &str) -> bool {
        let now = {
            let mut cfg = self.config.borrow_mut();
            if let Some(i) = cfg.wishlist.iter().position(|w| w == name) {
                cfg.wishlist.remove(i);
                false
            } else {
                cfg.wishlist.push(name.to_string());
                true
            }
        };
        self.save_config();
        self.notify();
        now
    }

    // ---- actions --------------------------------------------------------

    pub fn run(self: &Rc<Self>, tx: Transaction) {
        if self.busy.get() {
            self.toast("Another operation is still running");
            return;
        }
        transaction::run(self, tx);
    }

    pub fn install(self: &Rc<Self>, name: &str) {
        self.run(Transaction::install(&[name.to_string()], self.repo_only()));
    }

    pub fn remove(self: &Rc<Self>, name: &str) {
        let app = self.clone();
        let name = name.to_string();
        confirm(
            &self.window(),
            &format!("Remove {name}?"),
            "Dependencies nothing else needs are removed too. Configuration files you edited are kept.",
            "Remove",
            true,
            move |yes| {
                if yes {
                    app.run(Transaction::remove(std::slice::from_ref(&name)));
                }
            },
        );
    }

    pub fn update(self: &Rc<Self>, names: &[String]) {
        self.run(Transaction::update(names, self.repo_only()));
    }

    pub fn update_all(self: &Rc<Self>) {
        self.update(&[]);
    }

    /// Refresh the databases (needs your password) and re-check.
    pub fn refresh(self: &Rc<Self>) {
        self.run(Transaction::refresh(self.repo_only()));
    }

    pub fn open(self: &Rc<Self>, name: &str) {
        match self.launchable(name) {
            Some(app) => {
                if let Err(e) = crate::backend::apps::launch(&app.desktop_id) {
                    self.error("Could not open", &e);
                }
            }
            None => self.toast("This package has no app to open"),
        }
    }
}

/// Run `work` off the main thread, then `done` with its result on it.
pub fn spawn<T: Send + 'static>(
    work: impl FnOnce() -> T + Send + 'static,
    done: impl FnOnce(T) + 'static,
) {
    glib::spawn_future_local(async move {
        match gio::spawn_blocking(work).await {
            Ok(v) => done(v),
            Err(_) => tracing::error!("background task panicked"),
        }
    });
}

pub fn run(start_page: &'static str) -> glib::ExitCode {
    adw::init().expect("could not initialise GTK: is a Wayland display available?");
    let gtk_app = adw::Application::builder().application_id(APP_ID).build();
    let app: Rc<App> = Rc::new(App {
        desktop: Desktop::load(),
        config: RefCell::new(StoreConfig::load()),
        state: RefCell::new(State::default()),
        toasts: adw::ToastOverlay::new(),
        window: RefCell::new(None),
        listeners: RefCell::new(Vec::new()),
        nav: RefCell::new(None),
        busy: std::cell::Cell::new(false),
    });

    gtk_app.connect_activate(move |gtk_app| {
        if let Some(w) = app.window.borrow().as_ref() {
            w.present();
            return;
        }
        theme::load_base();
        let a = &app.desktop.appearance;
        theme::apply(None, a.theme_mode, &a.accent, a.transparency);
        let (window, nav, stack) = window::build(gtk_app, &app);
        *app.window.borrow_mut() = Some(window.clone());
        *app.nav.borrow_mut() = Some((nav, stack));
        theme::apply(Some(&window), a.theme_mode, &a.accent, a.transparency);
        window.present();
        app.navigate(start_page);
        if !rvn::available() {
            let t = adw::Toast::new(
                "rvn was not found on this system — the store cannot install anything",
            );
            t.set_timeout(0);
            app.toasts.add_toast(t);
        }
        app.reload();
        if let Ok(dir) = std::env::var("RAVEN_STORE_SNAPSHOT") {
            snapshot_pages(&app, std::path::PathBuf::from(dir));
        }
        if app.config.borrow().refresh_on_start {
            let app = app.clone();
            glib::timeout_add_local_once(std::time::Duration::from_millis(800), move || {
                app.refresh()
            });
        }
    });

    gtk_app.run_with_args::<&str>(&[])
}

/// Ask a yes/no question. `on_answer(true)` when confirmed.
pub fn confirm(
    parent: &impl IsA<gtk::Widget>,
    heading: &str,
    body: &str,
    action: &str,
    destructive: bool,
    on_answer: impl Fn(bool) + 'static,
) {
    let d = adw::AlertDialog::new(Some(heading), Some(body));
    d.add_response("cancel", "Cancel");
    d.add_response("ok", action);
    d.set_response_appearance(
        "ok",
        if destructive {
            adw::ResponseAppearance::Destructive
        } else {
            adw::ResponseAppearance::Suggested
        },
    );
    d.set_default_response(Some("ok"));
    d.set_close_response("cancel");
    d.connect_response(None, move |_, r| on_answer(r == "ok"));
    d.present(Some(parent));
}

/// Ask for the user's password. `on_answer(None)` when cancelled.
pub fn ask_password(
    parent: &impl IsA<gtk::Widget>,
    heading: &str,
    body: &str,
    on_answer: impl Fn(Option<String>) + 'static,
) {
    let d = adw::AlertDialog::new(Some(heading), Some(body));
    let entry = gtk::PasswordEntry::builder()
        .placeholder_text("Password")
        .show_peek_icon(true)
        .activates_default(true)
        .build();
    d.set_extra_child(Some(&entry));
    d.add_response("cancel", "Cancel");
    d.add_response("ok", "Authenticate");
    d.set_response_appearance("ok", adw::ResponseAppearance::Suggested);
    d.set_default_response(Some("ok"));
    d.set_close_response("cancel");
    let e2 = entry.clone();
    d.connect_response(None, move |_, r| {
        if r == "ok" {
            on_answer(Some(e2.text().to_string()));
        } else {
            on_answer(None);
        }
    });
    d.present(Some(parent));
    entry.grab_focus();
}

/// Development aid: with `RAVEN_STORE_SNAPSHOT=<dir>`, render every page
/// to a PNG in that directory and quit. Lets the UI be checked from a shell.
fn snapshot_pages(app: &Rc<App>, dir: std::path::PathBuf) {
    let app = app.clone();
    glib::timeout_add_local_once(std::time::Duration::from_millis(3500), move || {
        let window = app.window();
        let mut ids: Vec<&'static str> = pages::ids();
        ids.push("search");
        if let Some(q) = std::env::var("RAVEN_STORE_SNAPSHOT_QUERY")
            .ok()
            .filter(|q| !q.is_empty())
        {
            pages::search::show(&app, Box::leak(q.into_boxed_str()));
        }
        // `RAVEN_STORE_SNAPSHOT_TX=1` also starts a refresh so the progress
        // dialog is captured on the first page.
        if std::env::var_os("RAVEN_STORE_SNAPSHOT_TX").is_some() {
            app.refresh();
        }
        let mut i = 0usize;
        glib::timeout_add_local(std::time::Duration::from_millis(1500), move || {
            if i > 0 {
                let id = ids[i - 1];
                let paintable = gtk::WidgetPaintable::new(Some(&window));
                let snap = gtk::Snapshot::new();
                paintable.snapshot(&snap, window.width() as f64, window.height() as f64);
                if let (Some(node), Some(renderer)) = (snap.to_node(), window.renderer()) {
                    let tex = renderer.render_texture(node, None);
                    let _ = std::fs::create_dir_all(&dir);
                    tex.save_to_png(dir.join(format!("{id}.png"))).ok();
                }
            }
            if i >= ids.len() {
                // A running transaction keeps its dialog (and so the window)
                // from closing; this is a dev hook, so just leave.
                std::process::exit(0);
            }
            app.navigate(ids[i]);
            i += 1;
            glib::ControlFlow::Continue
        });
    });
}
