//! Installed: what is on the machine, with a filter and a Remove button.

use std::rc::Rc;

use gtk4 as gtk;
use libadwaita::prelude::*;

use crate::backend::human_bytes;
use crate::ui::widgets::{self, CardInfo};
use crate::ui::{App, Status};

pub fn build(app: &Rc<App>) -> gtk::Widget {
    let (root, content) = widgets::page("Installed", "");

    let bar = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let filter = gtk::SearchEntry::builder()
        .placeholder_text("Filter installed packages…")
        .hexpand(true)
        .build();
    bar.append(&filter);
    let count = gtk::Label::new(None);
    count.add_css_class("dim");
    bar.append(&count);
    content.append(&bar);

    let list = widgets::list();
    content.append(&list);
    let empty = widgets::empty_state(
        "folder-download-symbolic",
        "Nothing yet",
        "Packages you install appear here.",
    );
    empty.set_visible(false);
    content.append(&empty);

    let refill = {
        let list = list.clone();
        let count = count.clone();
        let filter = filter.clone();
        let empty = empty.clone();
        move |app: &Rc<App>| {
            widgets::clear(&list);
            let q = filter.text().to_lowercase();
            let show_deps = app.config.borrow().show_dependencies;
            let mut pkgs: Vec<_> = app
                .state
                .borrow()
                .installed
                .values()
                .filter(|p| show_deps || p.explicit)
                .filter(|p| {
                    q.is_empty()
                        || p.name.to_lowercase().contains(&q)
                        || p.description.to_lowercase().contains(&q)
                })
                .cloned()
                .collect();
            pkgs.sort_by(|a, b| a.name.cmp(&b.name));
            count.set_text(&format!(
                "{} package{}",
                pkgs.len(),
                if pkgs.len() == 1 { "" } else { "s" }
            ));
            empty.set_visible(pkgs.is_empty());
            list.set_visible(!pkgs.is_empty());
            // A few hundred rows is fine for a ListBox; cap the very long
            // dependency view so the page stays responsive.
            for p in pkgs.iter().take(600) {
                let info = CardInfo::from_package(p);
                let mut subtitle = format!("{}  ·  {}", p.version, human_bytes(p.installed_size));
                if !p.description.is_empty() {
                    subtitle = format!("{subtitle}  ·  {}", p.description);
                }
                let row = widgets::package_row(app, &info, &subtitle);
                if let Status::Updatable { .. } = app.status(&p.name) {
                    row.add_suffix(&widgets::badge("update", "update"));
                }
                if p.aur {
                    row.add_suffix(&widgets::badge("aur", "aur"));
                }
                if app.launchable(&p.name).is_some() {
                    let open = gtk::Button::from_icon_name("media-playback-start-symbolic");
                    open.add_css_class("flat");
                    open.set_tooltip_text(Some("Open"));
                    open.set_valign(gtk::Align::Center);
                    let app2 = app.clone();
                    let n = p.name.clone();
                    open.connect_clicked(move |_| app2.open(&n));
                    row.add_suffix(&open);
                }
                let remove = gtk::Button::from_icon_name("user-trash-symbolic");
                remove.add_css_class("flat");
                remove.set_tooltip_text(Some("Remove"));
                remove.set_valign(gtk::Align::Center);
                let app2 = app.clone();
                let n = p.name.clone();
                remove.connect_clicked(move |_| app2.remove(&n));
                row.add_suffix(&remove);
                list.append(&row);
            }
        }
    };
    let refill = Rc::new(refill);
    {
        let refill = refill.clone();
        let app2 = app.clone();
        filter.connect_search_changed(move |_| refill(&app2));
    }
    {
        let refill = refill.clone();
        app.on_change(move |app| refill(app));
    }
    refill(app);
    root.upcast()
}
