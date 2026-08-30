//! Categories: a grid of tiles, each opening the curated apps behind it.

use std::cell::RefCell;
use std::rc::Rc;

use gtk4 as gtk;
use libadwaita::prelude::*;

use crate::catalog;
use crate::ui::widgets::{self, CardInfo};
use crate::ui::App;

thread_local! {
    static PAGE: RefCell<Option<(gtk::Stack, gtk::Label, gtk::Label, gtk::FlowBox)>> = const { RefCell::new(None) };
    static CURRENT: RefCell<Option<&'static str>> = const { RefCell::new(None) };
}

pub fn build(app: &Rc<App>) -> gtk::Widget {
    let stack = gtk::Stack::builder()
        .transition_type(gtk::StackTransitionType::SlideLeftRight)
        .build();

    // ---- grid -----------------------------------------------------------
    let (grid_root, grid_content) = widgets::page(
        "Categories",
        "Browse the curated catalogue, or search for anything in the repositories.",
    );
    let tiles = widgets::flow(4);
    for cat in catalog::CATEGORIES {
        let tile = gtk::Box::new(gtk::Orientation::Vertical, 8);
        tile.add_css_class("category-tile");
        tile.append(&gtk::Image::from_icon_name(cat.icon));
        let t = gtk::Label::new(Some(cat.title));
        t.add_css_class("cat-title");
        t.set_xalign(0.0);
        tile.append(&t);
        let s = gtk::Label::new(Some(cat.blurb));
        s.add_css_class("app-kind");
        s.set_xalign(0.0);
        s.set_wrap(true);
        tile.append(&s);
        let n = gtk::Label::new(Some(&format!(
            "{} apps",
            catalog::in_category(cat.id).len()
        )));
        n.add_css_class("note");
        n.set_xalign(0.0);
        tile.append(&n);
        let b = gtk::Button::builder().child(&tile).build();
        b.add_css_class("flat");
        let app2 = app.clone();
        let id = cat.id;
        b.connect_clicked(move |_| open(&app2, id));
        tiles.insert(&b, -1);
    }
    grid_content.append(&tiles);
    stack.add_named(&grid_root, Some("grid"));

    // ---- one category ---------------------------------------------------
    let (list_root, list_content) = widgets::page("", "");
    let head = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    let back = gtk::Button::from_icon_name("go-previous-symbolic");
    back.add_css_class("flat");
    back.set_tooltip_text(Some("All categories"));
    {
        let stack = stack.clone();
        back.connect_clicked(move |_| stack.set_visible_child_name("grid"));
    }
    head.append(&back);
    let text = gtk::Box::new(gtk::Orientation::Vertical, 2);
    let title = gtk::Label::new(None);
    title.add_css_class("page-title");
    title.set_xalign(0.0);
    text.append(&title);
    let blurb = gtk::Label::new(None);
    blurb.add_css_class("page-subtitle");
    blurb.set_xalign(0.0);
    text.append(&blurb);
    head.append(&text);
    list_content.append(&head);
    let cards = widgets::flow(4);
    list_content.append(&cards);
    stack.add_named(&list_root, Some("list"));

    PAGE.with(|p| *p.borrow_mut() = Some((stack.clone(), title, blurb, cards)));
    app.on_change(refill);

    stack.upcast()
}

/// Jump to one category, from anywhere.
pub fn open(app: &Rc<App>, id: &'static str) {
    CURRENT.with(|c| *c.borrow_mut() = Some(id));
    refill(app);
    PAGE.with(|p| {
        if let Some((stack, _, _, _)) = p.borrow().as_ref() {
            stack.set_visible_child_name("list");
        }
    });
    app.navigate("categories");
}

fn refill(app: &Rc<App>) {
    let Some(id) = CURRENT.with(|c| *c.borrow()) else {
        return;
    };
    let Some(cat) = catalog::category(id) else {
        return;
    };
    PAGE.with(|p| {
        if let Some((_, title, blurb, cards)) = p.borrow().as_ref() {
            title.set_text(cat.title);
            blurb.set_text(cat.blurb);
            widgets::clear_flow(cards);
            for e in catalog::in_category(id) {
                cards.insert(&widgets::app_card(app, &CardInfo::from_entry(e)), -1);
            }
        }
    });
}
