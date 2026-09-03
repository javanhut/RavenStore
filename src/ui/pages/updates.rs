//! Updates: what is out of date, one button to apply it all.

use std::rc::Rc;

use gtk4 as gtk;
use libadwaita as adw;
use libadwaita::prelude::*;

use crate::backend::human_bytes;
use crate::catalog;
use crate::ui::{widgets, App};

pub fn build(app: &Rc<App>) -> gtk::Widget {
    let (root, content) = widgets::page(
        "Updates",
        "Keep Raven current. Updates are downloaded, verified and applied by rvn.",
    );

    let (status_card, status_body) = widgets::card("", "");
    let row = gtk::Box::new(gtk::Orientation::Horizontal, 14);
    let icon = gtk::Image::from_icon_name("software-update-available-symbolic");
    icon.set_pixel_size(36);
    row.append(&icon);
    let text = gtk::Box::new(gtk::Orientation::Vertical, 2);
    text.set_hexpand(true);
    text.set_valign(gtk::Align::Center);
    let headline = gtk::Label::new(Some("Checking…"));
    headline.add_css_class("card-title");
    headline.set_xalign(0.0);
    let detail = widgets::dim_label("");
    text.append(&headline);
    text.append(&detail);
    row.append(&text);
    let spinner = gtk::Spinner::new();
    row.append(&spinner);
    let check = gtk::Button::with_label("Check for updates");
    check.set_valign(gtk::Align::Center);
    check.set_tooltip_text(Some("Refresh the repository databases"));
    {
        let app2 = app.clone();
        check.connect_clicked(move |_| app2.refresh());
    }
    row.append(&check);
    let update_all = gtk::Button::with_label("Update All");
    update_all.add_css_class("suggested-action");
    update_all.set_valign(gtk::Align::Center);
    update_all.set_visible(false);
    {
        let app2 = app.clone();
        update_all.connect_clicked(move |_| app2.update_all());
    }
    row.append(&update_all);
    status_body.append(&row);
    content.append(&status_card);

    let (list_card, list_body) = widgets::card("Available updates", "");
    let list = widgets::list();
    list_body.append(&list);
    list_card.set_visible(false);
    content.append(&list_card);

    let (down_card, down_body) = widgets::card("Newer than the repositories", "These installed packages are ahead of what the repositories carry, so they are left alone.");
    let down_list = widgets::list();
    down_body.append(&down_list);
    down_card.set_visible(false);
    content.append(&down_card);

    let note = widgets::dim_label("Packages from the AUR are rebuilt from source on this machine, which can take a while. Prefer a terminal? Settings has a button for that.");
    content.append(&note);

    let refresh = move |app: &Rc<App>| {
        let st = app.state.borrow();
        if st.loading && !st.checked {
            spinner.start();
            headline.set_text("Checking…");
            detail.set_text("");
            return;
        }
        spinner.stop();
        if let Some(e) = &st.error {
            headline.set_text("Could not check for updates");
            detail.set_text(e);
            update_all.set_visible(false);
            list_card.set_visible(false);
            return;
        }
        let u = &st.updates;
        let n = u.candidates.len();
        if n == 0 {
            headline.set_text("System is up to date");
            detail.set_text("Everything installed is at its latest known version. Checking here or in Settings refreshes the repositories for both.");
            icon.set_icon_name(Some("object-select-symbolic"));
        } else {
            headline.set_text(&format!(
                "{n} update{} available",
                if n == 1 { "" } else { "s" }
            ));
            let aur = u.candidates.iter().filter(|c| c.aur).count();
            let mut d = format!("Download {}", human_bytes(u.download_size));
            if aur > 0 {
                d.push_str(&format!("  ·  {aur} to rebuild from source"));
            }
            detail.set_text(&d);
            icon.set_icon_name(Some("software-update-available-symbolic"));
        }
        update_all.set_visible(n > 0);
        list_card.set_visible(n > 0);
        widgets::clear(&list);
        for c in &u.candidates {
            let curated = catalog::entry(&c.name);
            let mut sub = format!("{}  →  {}", c.installed_version, c.new_version);
            match c.kind.as_str() {
                "replacement" => sub.push_str(&format!(
                    "   (replaces {})",
                    c.replaces.clone().unwrap_or_default()
                )),
                "devel" => sub.push_str("   (upstream moved — rebuild)"),
                _ => {}
            }
            if c.aur {
                sub.push_str("   ·  built from source");
            } else if c.download_size > 0 {
                sub.push_str(&format!("   ·  {}", human_bytes(c.download_size)));
            }
            let row = adw::ActionRow::builder()
                .title(glib::markup_escape_text(
                    curated.map(|e| e.title).unwrap_or(&c.name),
                ))
                .subtitle(glib::markup_escape_text(&sub))
                .activatable(true)
                .build();
            row.add_prefix(&widgets::icon_for(
                app,
                &c.name,
                curated.map(|e| e.icon),
                28,
            ));
            row.add_suffix(&widgets::origin_badge(&c.origin, c.aur));
            let go = gtk::Button::from_icon_name("go-up-symbolic");
            go.add_css_class("circular");
            go.add_css_class("accent");
            go.set_valign(gtk::Align::Center);
            go.set_tooltip_text(Some("Update just this package"));
            {
                let app2 = app.clone();
                let name = c.name.clone();
                go.connect_clicked(move |_| app2.update(std::slice::from_ref(&name)));
            }
            row.add_suffix(&go);
            {
                let app2 = app.clone();
                let name = c.name.clone();
                let title = curated
                    .map(|e| e.title.to_string())
                    .unwrap_or_else(|| c.name.clone());
                row.connect_activated(move |_| crate::ui::detail::show(&app2, &name, &title));
            }
            list.append(&row);
        }
        down_card.set_visible(!u.downgrades.is_empty());
        widgets::clear(&down_list);
        for c in &u.downgrades {
            let row = adw::ActionRow::builder()
                .title(glib::markup_escape_text(&c.name))
                .subtitle(format!(
                    "installed {}  ·  repository {}",
                    c.installed_version, c.new_version
                ))
                .build();
            down_list.append(&row);
        }
    };
    refresh(app);
    app.on_change(refresh);
    root.upcast()
}
