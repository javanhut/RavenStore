//! The package detail view: everything rvn knows, and the buttons to act.

use std::rc::Rc;

use gtk4 as gtk;
use libadwaita as adw;
use libadwaita::prelude::*;

use super::{spawn, widgets, App, Status};
use crate::backend::human_bytes;
use crate::backend::rvn::{self, Package};

pub fn show(app: &Rc<App>, package: &str, title: &str) {
    let dialog = adw::Dialog::new();
    dialog.set_title(title);
    dialog.set_content_width(640);
    dialog.set_content_height(620);

    let toolbar = adw::ToolbarView::new();
    toolbar.add_top_bar(&adw::HeaderBar::new());
    let body = gtk::Box::new(gtk::Orientation::Vertical, 14);
    body.set_margin_start(22);
    body.set_margin_end(22);
    body.set_margin_top(6);
    body.set_margin_bottom(22);
    let scroller = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vexpand(true)
        .child(&body)
        .build();
    toolbar.set_content(Some(&scroller));
    dialog.set_child(Some(&toolbar));

    let spinner = gtk::Spinner::new();
    spinner.set_size_request(32, 32);
    spinner.set_halign(gtk::Align::Center);
    spinner.set_margin_top(60);
    spinner.start();
    body.append(&spinner);

    dialog.present(Some(&app.window()));

    let name = package.to_string();
    let repo_only = app.repo_only();
    let app2 = app.clone();
    let dialog2 = dialog.clone();
    spawn(
        move || rvn::info(&name, repo_only),
        move |result| {
            widgets::clear_box(&body);
            match result {
                Ok(Some(pkg)) => fill(&app2, &dialog2, &body, &pkg),
                Ok(None) => body.append(&widgets::empty_state(
                    "dialog-question-symbolic",
                    "Not found",
                    "No repository or the AUR carries a package by this name.",
                )),
                Err(e) => body.append(&widgets::empty_state(
                    "dialog-warning-symbolic",
                    "Could not load package",
                    &e.to_string(),
                )),
            }
        },
    );
}

fn fill(app: &Rc<App>, dialog: &adw::Dialog, body: &gtk::Box, pkg: &Package) {
    let curated = crate::catalog::entry(&pkg.name);

    // ---- head: icon, name, badges, action -------------------------------
    let head = gtk::Box::new(gtk::Orientation::Horizontal, 16);
    let icon = widgets::icon_for(app, &pkg.name, curated.map(|e| e.icon), 72);
    icon.set_valign(gtk::Align::Start);
    head.append(&icon);

    let text = gtk::Box::new(gtk::Orientation::Vertical, 4);
    text.set_hexpand(true);
    text.set_valign(gtk::Align::Center);
    let title = gtk::Label::new(Some(curated.map(|e| e.title).unwrap_or(&pkg.name)));
    title.add_css_class("page-title");
    title.set_xalign(0.0);
    title.set_wrap(true);
    text.append(&title);
    if curated.is_some() {
        let pn = gtk::Label::new(Some(&pkg.name));
        pn.add_css_class("dim");
        pn.set_xalign(0.0);
        text.append(&pn);
    }
    let badges = gtk::Box::new(gtk::Orientation::Horizontal, 6);
    badges.append(&widgets::origin_badge(&pkg.origin, pkg.aur));
    let version = gtk::Label::new(Some(&pkg.version));
    version.add_css_class("dim");
    badges.append(&version);
    match app.status(&pkg.name) {
        Status::Installed { version } => badges.append(&widgets::badge(
            &format!("installed {version}"),
            "installed",
        )),
        Status::Updatable { from, to } => {
            badges.append(&widgets::badge(&format!("update {from} → {to}"), "update"))
        }
        Status::NotInstalled => {}
    }
    if pkg.out_of_date {
        badges.append(&widgets::badge("flagged out of date", "update"));
    }
    text.append(&badges);
    head.append(&text);
    body.append(&head);

    // ---- actions --------------------------------------------------------
    let actions = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    let primary = widgets::action_button(app, &pkg.name);
    primary.add_css_class("pill");
    actions.append(&primary);
    if app.launchable(&pkg.name).is_some() && app.status(&pkg.name) != Status::NotInstalled {
        if let Status::Updatable { .. } = app.status(&pkg.name) {
            let open = gtk::Button::with_label("Open");
            open.add_css_class("pill");
            let app2 = app.clone();
            let n = pkg.name.clone();
            open.connect_clicked(move |_| app2.open(&n));
            actions.append(&open);
        }
    }
    if app.is_installed(&pkg.name) {
        let remove = gtk::Button::with_label("Remove");
        remove.add_css_class("pill");
        remove.add_css_class("destructive-action");
        let app2 = app.clone();
        let n = pkg.name.clone();
        let d = dialog.clone();
        remove.connect_clicked(move |_| {
            d.close();
            app2.remove(&n);
        });
        actions.append(&remove);
    }
    let heart = widgets::heart_button(app, &pkg.name);
    heart.set_valign(gtk::Align::Center);
    actions.append(&heart);
    if let Some(url) = &pkg.url {
        let link = gtk::LinkButton::with_label(url, "Website");
        link.set_halign(gtk::Align::End);
        link.set_hexpand(true);
        actions.append(&link);
    }
    body.append(&actions);
    // Close the dialog once an install/update starts so the progress view
    // is what the user sees.
    {
        let d = dialog.clone();
        let app2 = app.clone();
        primary.connect_clicked(move |_| {
            if app2.busy.get() {
                d.close();
            }
        });
    }

    // ---- description ----------------------------------------------------
    let desc = widgets::dim_label(curated.map(|e| e.tagline).unwrap_or(""));
    desc.remove_css_class("dim");
    if curated.is_some() {
        body.append(&desc);
    }
    body.append(&widgets::dim_label(&pkg.description));

    // ---- facts ----------------------------------------------------------
    let list = widgets::list();
    let fact = |label: &str, value: &str| {
        if value.is_empty() {
            return;
        }
        let row = adw::ActionRow::builder()
            .title(label)
            .subtitle(glib::markup_escape_text(value))
            .subtitle_selectable(true)
            .build();
        list.append(&row);
    };
    if pkg.download_size > 0 {
        fact("Download", &human_bytes(pkg.download_size));
    }
    if pkg.installed_size > 0 {
        fact("Installed size", &human_bytes(pkg.installed_size));
    }
    fact("Licence", &pkg.licenses.join(", "));
    if pkg.aur {
        fact(
            "Source",
            "Arch User Repository — built from source on this machine",
        );
        if pkg.popularity > 0.0 {
            fact("Popularity", &format!("{:.2}", pkg.popularity));
        }
    } else {
        fact("Source", &format!("Official repository ({})", pkg.origin));
    }
    fact("Depends on", &pkg.depends.join("  "));
    if !pkg.optdepends.is_empty() {
        fact("Optional", &pkg.optdepends.join("\n"));
    }
    if !pkg.required_by.is_empty() {
        fact("Required by", &pkg.required_by.join("  "));
    }
    body.append(&list);
}
