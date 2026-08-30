//! Store preferences, and the escape hatch to a terminal.

use std::rc::Rc;

use gtk4 as gtk;
use libadwaita as adw;
use libadwaita::prelude::*;

use crate::backend::{rvn, system};
use crate::ui::{widgets, App};

pub fn build(app: &Rc<App>) -> gtk::Widget {
    let (root, content) = widgets::page("Settings", "How the store finds and installs software.");

    let (sources, sources_body) = widgets::card("Sources", "");
    let list = widgets::list();
    let repo_only = adw::SwitchRow::builder()
        .title("Official repositories only")
        .subtitle("Skip the Arch User Repository. AUR packages are built from source on this machine, so they take longer and are not signed by Arch.")
        .active(app.config.borrow().repo_only)
        .build();
    {
        let app2 = app.clone();
        repo_only.connect_active_notify(move |r| {
            app2.config.borrow_mut().repo_only = r.is_active();
            app2.save_config();
            app2.reload();
        });
    }
    list.append(&repo_only);
    let refresh = adw::SwitchRow::builder()
        .title("Refresh repositories when the store opens")
        .subtitle("Asks for your password at launch. Off, the store uses the databases already on disk and refreshes before any install.")
        .active(app.config.borrow().refresh_on_start)
        .build();
    {
        let app2 = app.clone();
        refresh.connect_active_notify(move |r| {
            app2.config.borrow_mut().refresh_on_start = r.is_active();
            app2.save_config();
        });
    }
    list.append(&refresh);
    let deps = adw::SwitchRow::builder()
        .title("Show dependencies on the Installed page")
        .subtitle("Include packages that were pulled in by something else.")
        .active(app.config.borrow().show_dependencies)
        .build();
    {
        let app2 = app.clone();
        deps.connect_active_notify(move |r| {
            app2.config.borrow_mut().show_dependencies = r.is_active();
            app2.save_config();
            app2.notify();
        });
    }
    list.append(&deps);
    sources_body.append(&list);
    content.append(&sources);

    let (term, term_body) = widgets::card("Prefer a terminal?", "The store is optional. Everything it does is `rvn` underneath, and you can run that yourself.");
    let row = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    let terminal = app.desktop.terminal();
    for (label, args) in [
        ("Update in a terminal", vec!["update"]),
        ("Refresh in a terminal", vec!["sync"]),
    ] {
        let b = gtk::Button::with_label(label);
        let app2 = app.clone();
        let terminal = terminal.clone();
        b.connect_clicked(
            move |_| match system::launch_in_terminal(&terminal, &args) {
                Ok(()) => app2.toast("Running in a terminal window"),
                Err(e) => app2.error("Could not open a terminal", &e),
            },
        );
        row.append(&b);
    }
    term_body.append(&row);
    term_body.append(&widgets::dim_label(&format!(
        "Terminal: {terminal} (change it in Settings → General)"
    )));
    content.append(&term);

    let (about, about_body) = widgets::card("About", "");
    let alist = widgets::list();
    let os = system::os_release();
    alist.append(
        &adw::ActionRow::builder()
            .title("Raven Store")
            .subtitle(env!("CARGO_PKG_VERSION"))
            .build(),
    );
    alist.append(
        &adw::ActionRow::builder()
            .title("rvn")
            .subtitle(rvn::version().unwrap_or_else(|| "not found".into()))
            .build(),
    );
    alist.append(
        &adw::ActionRow::builder()
            .title("System")
            .subtitle(format!("{} {}", os.name, os.version_id))
            .build(),
    );
    alist.append(
        &adw::ActionRow::builder()
            .title("Preferences file")
            .subtitle(crate::config::StoreConfig::path().display().to_string())
            .subtitle_selectable(true)
            .build(),
    );
    about_body.append(&alist);
    content.append(&about);

    root.upcast()
}
