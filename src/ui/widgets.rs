//! Building blocks shared by the pages: cards, page headers, app tiles.

use std::rc::Rc;

use gtk4 as gtk;
use libadwaita as adw;
use libadwaita::prelude::*;

use super::{App, Status};
use crate::backend::rvn::Package;
use crate::catalog::Entry;

/// A page: title, subtitle, and a vertical content box inside a scroller.
pub fn page(title: &str, subtitle: &str) -> (gtk::ScrolledWindow, gtk::Box) {
    let content = gtk::Box::new(gtk::Orientation::Vertical, 14);
    content.set_margin_start(26);
    content.set_margin_end(26);
    content.set_margin_top(18);
    content.set_margin_bottom(26);

    if !title.is_empty() {
        let head = gtk::Box::new(gtk::Orientation::Vertical, 4);
        let t = gtk::Label::new(Some(title));
        t.add_css_class("page-title");
        t.set_xalign(0.0);
        head.append(&t);
        if !subtitle.is_empty() {
            let s = gtk::Label::new(Some(subtitle));
            s.add_css_class("page-subtitle");
            s.set_xalign(0.0);
            s.set_wrap(true);
            s.set_margin_bottom(6);
            head.append(&s);
        }
        content.append(&head);
    }

    let scroller = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vexpand(true)
        .child(&content)
        .build();
    (scroller, content)
}

/// A card with a title/subtitle header. Returns (card, body).
pub fn card(title: &str, subtitle: &str) -> (gtk::Box, gtk::Box) {
    let outer = gtk::Box::new(gtk::Orientation::Vertical, 12);
    outer.add_css_class("raven-card");
    if !title.is_empty() {
        let head = gtk::Box::new(gtk::Orientation::Vertical, 2);
        let t = gtk::Label::new(Some(title));
        t.add_css_class("card-title");
        t.set_xalign(0.0);
        head.append(&t);
        if !subtitle.is_empty() {
            let s = gtk::Label::new(Some(subtitle));
            s.add_css_class("card-subtitle");
            s.set_xalign(0.0);
            s.set_wrap(true);
            head.append(&s);
        }
        outer.append(&head);
    }
    let body = gtk::Box::new(gtk::Orientation::Vertical, 8);
    outer.append(&body);
    (outer, body)
}

/// "Popular Apps ............ View all"
pub fn section_header(title: &str, view_all: Option<Box<dyn Fn()>>) -> gtk::Box {
    let row = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    row.set_margin_top(6);
    let t = gtk::Label::new(Some(title));
    t.add_css_class("section-title");
    t.set_xalign(0.0);
    t.set_hexpand(true);
    row.append(&t);
    if let Some(f) = view_all {
        let b = gtk::Button::with_label("View all");
        b.add_css_class("flat");
        b.add_css_class("accent");
        b.connect_clicked(move |_| f());
        row.append(&b);
    }
    row
}

pub fn dim_label(text: &str) -> gtk::Label {
    let l = gtk::Label::new(Some(text));
    l.add_css_class("dim");
    l.set_xalign(0.0);
    l.set_wrap(true);
    l.set_wrap_mode(gtk::pango::WrapMode::WordChar);
    l.set_natural_wrap_mode(gtk::NaturalWrapMode::None);
    l.set_max_width_chars(60);
    l.set_hexpand(true);
    l
}

pub fn badge(text: &str, class: &str) -> gtk::Label {
    let l = gtk::Label::new(Some(text));
    l.add_css_class("badge");
    if !class.is_empty() {
        l.add_css_class(class);
    }
    l.set_valign(gtk::Align::Center);
    l
}

pub fn origin_badge(origin: &str, aur: bool) -> gtk::Label {
    badge(origin, if aur { "aur" } else { "origin" })
}

pub fn list() -> gtk::ListBox {
    let l = gtk::ListBox::new();
    l.add_css_class("boxed-list");
    l.set_selection_mode(gtk::SelectionMode::None);
    l
}

pub fn clear(bx: &gtk::ListBox) {
    while let Some(child) = bx.first_child() {
        bx.remove(&child);
    }
}

pub fn clear_box(bx: &gtk::Box) {
    while let Some(child) = bx.first_child() {
        bx.remove(&child);
    }
}

pub fn clear_flow(fb: &gtk::FlowBox) {
    while let Some(child) = fb.first_child() {
        fb.remove(&child);
    }
}

/// A grid that wraps, for app tiles.
pub fn flow(max_per_line: u32) -> gtk::FlowBox {
    let fb = gtk::FlowBox::new();
    fb.set_selection_mode(gtk::SelectionMode::None);
    fb.set_homogeneous(true);
    fb.set_min_children_per_line(1);
    fb.set_max_children_per_line(max_per_line);
    fb.set_column_spacing(12);
    fb.set_row_spacing(12);
    fb.set_valign(gtk::Align::Start);
    fb
}

pub fn empty_state(icon: &str, title: &str, description: &str) -> adw::StatusPage {
    adw::StatusPage::builder()
        .icon_name(icon)
        .title(title)
        .description(description)
        .vexpand(true)
        .build()
}

/// The best icon for a package: the app it installed, then the catalogue's
/// hint, then a generic package glyph.
pub fn icon_for(app: &App, package: &str, hint: Option<&str>, size: i32) -> gtk::Image {
    let image = if let Some(launchable) = app.launchable(package) {
        match launchable.icon.as_deref() {
            Some(path) if path.starts_with('/') => gtk::Image::from_file(path),
            Some(name) if theme_has(name) => gtk::Image::from_icon_name(name),
            _ => gtk::Image::from_icon_name("application-x-executable-symbolic"),
        }
    } else if let Some(name) = hint.filter(|n| theme_has(n)) {
        gtk::Image::from_icon_name(name)
    } else {
        gtk::Image::from_icon_name("package-x-generic-symbolic")
    };
    image.set_pixel_size(size);
    image
}

fn theme_has(name: &str) -> bool {
    gtk::gdk::Display::default()
        .map(|d| gtk::IconTheme::for_display(&d).has_icon(name))
        .unwrap_or(false)
}

/// The tinted square behind a tile's icon. Real app icons get no tint.
pub fn icon_well(app: &App, package: &str) -> gtk::Box {
    let well = gtk::Box::new(gtk::Orientation::Vertical, 0);
    well.add_css_class("icon-well");
    well.set_valign(gtk::Align::Start);
    if app.launchable(package).is_some() {
        well.add_css_class("real");
    } else if let Some(e) = crate::catalog::entry(package) {
        well.add_css_class(e.category);
    }
    well
}

/// What a tile shows, whether it came from the catalogue or a search.
#[derive(Debug, Clone)]
pub struct CardInfo {
    pub package: String,
    pub title: String,
    pub kind: String,
    pub icon: Option<String>,
    pub description: String,
    pub origin: String,
    pub aur: bool,
    pub popularity: f64,
}

impl CardInfo {
    pub fn from_entry(e: &Entry) -> CardInfo {
        CardInfo {
            package: e.package.into(),
            title: e.title.into(),
            kind: e.kind.into(),
            icon: Some(e.icon.into()),
            description: e.tagline.into(),
            origin: String::new(),
            aur: false,
            popularity: 0.0,
        }
    }

    pub fn from_package(p: &Package) -> CardInfo {
        let curated = crate::catalog::entry(&p.name);
        CardInfo {
            package: p.name.clone(),
            title: curated
                .map(|e| e.title.to_string())
                .unwrap_or_else(|| p.name.clone()),
            kind: curated
                .map(|e| e.kind.to_string())
                .unwrap_or_else(|| p.version.clone()),
            icon: curated.map(|e| e.icon.to_string()),
            description: p.description.clone(),
            origin: p.origin.clone(),
            aur: p.aur,
            popularity: p.popularity,
        }
    }
}

/// Install / Open / Update / Installed, whichever fits right now.
pub fn action_button(app: &Rc<App>, package: &str) -> gtk::Button {
    let button = gtk::Button::new();
    button.add_css_class("action");
    let content = gtk::Box::new(gtk::Orientation::Horizontal, 6);
    content.set_halign(gtk::Align::Center);
    let label = gtk::Label::new(None);
    let icon = gtk::Image::new();
    icon.set_pixel_size(14);
    content.append(&label);
    content.append(&icon);
    button.set_child(Some(&content));

    let name = package.to_string();
    let app2 = app.clone();
    match app.status(package) {
        Status::NotInstalled => {
            label.set_text("Install");
            icon.set_icon_name(Some("folder-download-symbolic"));
            button.add_css_class("suggested-action");
            button.connect_clicked(move |_| app2.install(&name));
        }
        Status::Updatable { .. } => {
            label.set_text("Update");
            icon.set_icon_name(Some("software-update-available-symbolic"));
            button.add_css_class("suggested-action");
            button.connect_clicked(move |_| app2.update(std::slice::from_ref(&name)));
        }
        Status::Installed { .. } => {
            if app.launchable(package).is_some() {
                label.set_text("Open");
                icon.set_icon_name(Some("media-playback-start-symbolic"));
                button.connect_clicked(move |_| app2.open(&name));
            } else {
                label.set_text("Installed");
                icon.set_icon_name(Some("object-select-symbolic"));
                button.set_sensitive(false);
            }
        }
    }
    button
}

pub fn heart_button(app: &Rc<App>, package: &str) -> gtk::Button {
    let on = app.config.borrow().wishes(package);
    // Unicode hearts rather than theme icons: no icon theme on the image
    // ships an outline heart, and a faded filled one reads as "already
    // wishlisted".
    let glyph = gtk::Label::new(Some(if on { "\u{2665}" } else { "\u{2661}" }));
    let b = gtk::Button::builder().child(&glyph).build();
    b.add_css_class("flat");
    b.add_css_class("heart");
    if on {
        b.add_css_class("on");
    }
    b.set_tooltip_text(Some(if on {
        "Remove from wishlist"
    } else {
        "Add to wishlist"
    }));
    b.set_valign(gtk::Align::Start);
    let app = app.clone();
    let name = package.to_string();
    b.connect_clicked(move |_| {
        app.toggle_wish(&name);
    });
    b
}

/// An app tile. Clicking the tile opens the detail view; the button acts.
pub fn app_card(app: &Rc<App>, info: &CardInfo) -> gtk::Box {
    let card = gtk::Box::new(gtk::Orientation::Vertical, 10);
    card.add_css_class("app-card");

    let top = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let well = icon_well(app, &info.package);
    let icon = icon_for(app, &info.package, info.icon.as_deref(), 36);
    icon.add_css_class("app-icon");
    well.append(&icon);
    top.append(&well);

    let text = gtk::Box::new(gtk::Orientation::Vertical, 1);
    text.set_hexpand(true);
    text.set_valign(gtk::Align::Center);
    let name = gtk::Label::new(Some(&info.title));
    name.add_css_class("app-name");
    name.set_xalign(0.0);
    name.set_ellipsize(gtk::pango::EllipsizeMode::End);
    // Bound the natural width, or one long package name widens every tile
    // in a homogeneous grid and the row collapses to a single column.
    name.set_max_width_chars(16);
    text.append(&name);
    let kind = gtk::Label::new(Some(&info.kind));
    kind.add_css_class("app-kind");
    kind.set_xalign(0.0);
    kind.set_ellipsize(gtk::pango::EllipsizeMode::End);
    kind.set_max_width_chars(18);
    text.append(&kind);
    top.append(&text);
    top.append(&heart_button(app, &info.package));
    card.append(&top);

    let meta = gtk::Box::new(gtk::Orientation::Horizontal, 6);
    match app.status(&info.package) {
        Status::Updatable { .. } => meta.append(&badge("update", "update")),
        Status::Installed { .. } => meta.append(&badge("installed", "installed")),
        Status::NotInstalled => {}
    }
    if !info.origin.is_empty() {
        meta.append(&origin_badge(&info.origin, info.aur));
    } else if let Some(e) = crate::catalog::entry(&info.package) {
        // Curated entries only know their package name; the badge appears
        // once a search or the detail view has told us where it lives.
        let _ = e;
    }
    if info.popularity > 0.0 {
        let p = gtk::Label::new(Some(&format!("▲ {:.1}", info.popularity)));
        p.add_css_class("app-kind");
        p.set_tooltip_text(Some("AUR popularity"));
        meta.append(&p);
    }
    if meta.first_child().is_some() {
        card.append(&meta);
    }

    let action = action_button(app, &info.package);
    action.set_hexpand(true);
    card.append(&action);

    // The tile itself opens the detail view. Buttons claim their own clicks
    // first, so this only fires on the card body.
    let gesture = gtk::GestureClick::new();
    gesture.set_button(1);
    let app2 = app.clone();
    let name = info.package.clone();
    let title = info.title.clone();
    gesture.connect_released(move |g, _, _, _| {
        g.set_state(gtk::EventSequenceState::Claimed);
        super::detail::show(&app2, &name, &title);
    });
    card.add_controller(gesture);
    card.set_cursor_from_name(Some("pointer"));
    if !info.description.is_empty() {
        card.set_tooltip_text(Some(&info.description));
    }
    card
}

/// A compact row for lists: icon, name, subtitle, trailing widgets.
pub fn package_row(app: &Rc<App>, info: &CardInfo, subtitle: &str) -> adw::ActionRow {
    let row = adw::ActionRow::builder()
        .title(glib::markup_escape_text(&info.title))
        .subtitle(glib::markup_escape_text(subtitle))
        .activatable(true)
        .build();
    let icon = icon_for(app, &info.package, info.icon.as_deref(), 28);
    row.add_prefix(&icon);
    let app2 = app.clone();
    let name = info.package.clone();
    let title = info.title.clone();
    row.connect_activated(move |_| super::detail::show(&app2, &name, &title));
    row
}
