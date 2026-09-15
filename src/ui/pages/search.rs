//! Search results, fed by the header's search field. Applications come
//! first and packages — libraries, plugins, language packs — after, each
//! best first (see `crate::ranking`), so the app people meant is the one
//! they install.

use std::cell::RefCell;
use std::rc::Rc;

use gtk4 as gtk;
use libadwaita as adw;
use libadwaita::prelude::*;

use crate::backend::rvn::{self, Package};
use crate::ranking;
use crate::ui::widgets::{self, CardInfo};
use crate::ui::{spawn, App};

/// How many results to ask rvn for. Generous, because rvn's own order can
/// put an app's add-ons ahead of it and the ranking needs the app in hand.
const LIMIT: usize = 96;

struct Page {
    title: gtk::Label,
    subtitle: gtk::Label,
    spinner: gtk::Spinner,
    apps_head: gtk::Box,
    apps_count: gtk::Label,
    apps: gtk::FlowBox,
    packages_head: gtk::Box,
    packages_count: gtk::Label,
    packages: gtk::ListBox,
    empty: gtk::Widget,
    /// The query whose results are showing (or loading).
    query: RefCell<String>,
    results: RefCell<Vec<Package>>,
}

thread_local! {
    static PAGE: RefCell<Option<Rc<Page>>> = const { RefCell::new(None) };
}

pub fn build(app: &Rc<App>) -> gtk::Widget {
    let (root, content) = widgets::page("", "");
    let head = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    let text = gtk::Box::new(gtk::Orientation::Vertical, 2);
    text.set_hexpand(true);
    let title = gtk::Label::new(Some("Search"));
    title.add_css_class("page-title");
    title.set_xalign(0.0);
    text.append(&title);
    let subtitle = gtk::Label::new(None);
    subtitle.add_css_class("page-subtitle");
    subtitle.set_xalign(0.0);
    text.append(&subtitle);
    head.append(&text);
    let spinner = gtk::Spinner::new();
    head.append(&spinner);
    content.append(&head);

    let (apps_head, apps_count) = section("Applications", None);
    content.append(&apps_head);
    let apps = widgets::flow(3);
    content.append(&apps);

    let (packages_head, packages_count) = section(
        "Packages",
        Some("Libraries, plugins, language packs and other parts of apps. Most are installed for you as dependencies, so check before installing one by hand."),
    );
    packages_head.set_margin_top(12);
    content.append(&packages_head);
    let packages = widgets::list();
    packages.set_valign(gtk::Align::Start);
    content.append(&packages);

    let empty = widgets::empty_state("system-search-symbolic", "No matches", "Nothing in the repositories or the AUR matches. Check the spelling, or try a shorter term.");
    empty.set_visible(false);
    content.append(&empty);

    let page = Rc::new(Page {
        title,
        subtitle,
        spinner,
        apps_head,
        apps_count,
        apps,
        packages_head,
        packages_count,
        packages,
        empty: empty.upcast(),
        query: RefCell::new(String::new()),
        results: RefCell::new(Vec::new()),
    });
    PAGE.with(|p| *p.borrow_mut() = Some(page.clone()));
    app.on_change(move |app| {
        if let Some(page) = PAGE.with(|p| p.borrow().clone()) {
            render(app, &page);
        }
    });
    root.upcast()
}

/// A section heading with a count beside it and an optional note below.
fn section(title: &str, note: Option<&str>) -> (gtk::Box, gtk::Label) {
    let bx = gtk::Box::new(gtk::Orientation::Vertical, 4);
    bx.set_margin_top(6);
    let row = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let t = gtk::Label::new(Some(title));
    t.add_css_class("section-heading");
    t.set_xalign(0.0);
    row.append(&t);
    let count = widgets::badge("0", "count");
    row.append(&count);
    bx.append(&row);
    if let Some(note) = note {
        let n = widgets::dim_label(note);
        n.add_css_class("results-note");
        n.set_max_width_chars(100);
        bx.append(&n);
    }
    (bx, count)
}

/// Run a search and show the results page.
pub fn show(app: &Rc<App>, query: &str) {
    let Some(page) = PAGE.with(|p| p.borrow().clone()) else {
        return;
    };
    *page.query.borrow_mut() = query.to_string();
    page.title.set_text(&format!("Results for “{query}”"));
    page.subtitle
        .set_text("Searching the repositories and the AUR…");
    page.spinner.start();
    app.navigate("search");

    let q = query.to_string();
    let repo_only = app.repo_only();
    let app2 = app.clone();
    spawn(
        move || rvn::search(&q, repo_only, LIMIT).map(|r| (q, r)),
        move |result| match result {
            Ok((q, results)) => {
                // A slower, older search must not overwrite a newer one.
                if *page.query.borrow() != q {
                    return;
                }
                page.spinner.stop();
                *page.results.borrow_mut() = results;
                render(&app2, &page);
            }
            Err(e) => {
                page.spinner.stop();
                page.subtitle.set_text(&format!("Search failed: {e}"));
            }
        },
    );
}

fn render(app: &Rc<App>, page: &Page) {
    let query = page.query.borrow().clone();
    if query.is_empty() {
        return;
    }
    let ranked = ranking::arrange(&query, &page.results.borrow());
    let total = ranked.apps.len() + ranked.packages.len();
    page.subtitle.set_text(&format!(
        "{total} result{}{}",
        if total == 1 { "" } else { "s" },
        if app.repo_only() {
            " · official repositories only"
        } else {
            ""
        }
    ));
    page.empty.set_visible(total == 0);

    let has_apps = !ranked.apps.is_empty();
    page.apps_head.set_visible(has_apps);
    page.apps.set_visible(has_apps);
    page.apps_count.set_text(&ranked.apps.len().to_string());
    widgets::clear_flow(&page.apps);
    for p in &ranked.apps {
        page.apps
            .insert(&widgets::app_card(app, &CardInfo::from_package(p)), -1);
    }

    let has_packages = !ranked.packages.is_empty();
    page.packages_head.set_visible(has_packages);
    page.packages.set_visible(has_packages);
    page.packages_count
        .set_text(&ranked.packages.len().to_string());
    widgets::clear(&page.packages);
    for p in &ranked.packages {
        page.packages.append(&package_row(app, p));
    }
}

/// Packages are rows, not tiles, and carry no install button: installing
/// one takes the extra step through its detail view.
fn package_row(app: &Rc<App>, p: &Package) -> adw::ActionRow {
    let row = widgets::package_row(app, &CardInfo::from_package(p), &p.description);
    row.set_subtitle_lines(1);
    let version = gtk::Label::new(Some(&p.version));
    version.add_css_class("dim");
    version.set_valign(gtk::Align::Center);
    row.add_suffix(&version);
    if !p.origin.is_empty() {
        row.add_suffix(&widgets::origin_badge(&p.origin, p.aur));
    }
    if app.is_installed(&p.name) {
        row.add_suffix(&widgets::badge("installed", "installed"));
    }
    let chevron = gtk::Image::from_icon_name("go-next-symbolic");
    chevron.add_css_class("chevron");
    row.add_suffix(&chevron);
    row
}
