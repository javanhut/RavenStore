//! Search results, fed by the header's search field.

use std::cell::RefCell;
use std::rc::Rc;

use gtk4 as gtk;
use libadwaita::prelude::*;

use crate::backend::rvn::{self, Package};
use crate::catalog;
use crate::ui::widgets::{self, CardInfo};
use crate::ui::{spawn, App};

struct Page {
    title: gtk::Label,
    subtitle: gtk::Label,
    spinner: gtk::Spinner,
    cards: gtk::FlowBox,
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
    let cards = widgets::flow(4);
    content.append(&cards);
    let empty = widgets::empty_state("system-search-symbolic", "No matches", "Nothing in the repositories or the AUR matches. Check the spelling, or try a shorter term.");
    empty.set_visible(false);
    content.append(&empty);

    let page = Rc::new(Page {
        title,
        subtitle,
        spinner,
        cards,
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
        move || rvn::search(&q, repo_only, 48).map(|r| (q, r)),
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
    let results = page.results.borrow();
    widgets::clear_flow(&page.cards);
    // Curated matches first, so "code" shows VS Code before code-minimap.
    let curated: Vec<&catalog::Entry> = catalog::matching(&query)
        .into_iter()
        .filter(|e| !results.iter().any(|p| p.name == e.package))
        .collect();
    let total = results.len() + curated.len();
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
    for e in curated {
        page.cards
            .insert(&widgets::app_card(app, &CardInfo::from_entry(e)), -1);
    }
    for p in results.iter() {
        page.cards
            .insert(&widgets::app_card(app, &CardInfo::from_package(p)), -1);
    }
}
