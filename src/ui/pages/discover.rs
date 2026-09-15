//! Discover: the featured carousel, popular apps with a category filter,
//! browse by category, and a side panel with pending updates and Raven
//! Picks.

use std::cell::Cell;
use std::rc::Rc;

use gtk4 as gtk;
use libadwaita as adw;
use libadwaita::prelude::*;

use crate::backend::human_bytes;
use crate::catalog::{self, Entry, Feature, Shelf};
use crate::ui::widgets::{self, CardInfo};
use crate::ui::App;

/// Shelves offered as filter tabs over Popular Apps, after "All".
const TABS: usize = 6;
/// Cards in the Popular Apps grid: two rows of three.
const POPULAR_CARDS: usize = 6;

pub fn build(app: &Rc<App>) -> gtk::Widget {
    let (root, content) = widgets::page("", "");
    content.set_margin_top(4);

    let columns = gtk::Box::new(gtk::Orientation::Horizontal, 20);
    columns.add_css_class("columns");
    let main = gtk::Box::new(gtk::Orientation::Vertical, 14);
    main.set_hexpand(true);
    let side = gtk::Box::new(gtk::Orientation::Vertical, 18);
    side.add_css_class("side-panel");
    side.set_valign(gtk::Align::Start);
    columns.append(&main);
    columns.append(&side);
    content.append(&columns);

    // ---- featured -------------------------------------------------------
    let carousel = adw::Carousel::new();
    carousel.set_spacing(16);
    carousel.set_allow_scroll_wheel(false);
    let mut hero_actions: Vec<(gtk::Box, &'static str)> = Vec::new();
    for (i, f) in catalog::FEATURED.iter().enumerate() {
        if let Some(e) = catalog::entry(f.package) {
            let (card, slot) = hero(app, e, f, i);
            carousel.append(&card);
            hero_actions.push((slot, e.package));
        }
    }
    let lines = adw::CarouselIndicatorLines::new();
    lines.set_carousel(Some(&carousel));
    lines.set_halign(gtk::Align::End);
    lines.set_valign(gtk::Align::End);
    lines.set_margin_end(34);
    lines.set_margin_bottom(20);
    let overlay = gtk::Overlay::new();
    overlay.set_child(Some(&carousel));
    overlay.add_overlay(&lines);
    main.append(&overlay);
    {
        let carousel = carousel.clone();
        glib::timeout_add_local(std::time::Duration::from_secs(9), move || {
            let n = carousel.n_pages();
            if n > 1 {
                let next = (carousel.position().round() as u32 + 1) % n;
                carousel.scroll_to(&carousel.nth_page(next), true);
            }
            glib::ControlFlow::Continue
        });
    }

    // ---- popular --------------------------------------------------------
    // None is "All"; Some(i) is SHELVES[i].
    let tab: Rc<Cell<Option<usize>>> = Rc::new(Cell::new(None));
    {
        let app2 = app.clone();
        let tab = tab.clone();
        main.append(&widgets::section_header(
            "Popular Apps",
            Some(Box::new(move || match tab.get() {
                Some(i) => super::categories::open_shelf(&app2, &catalog::SHELVES[i]),
                None => app2.navigate("categories"),
            })),
        ));
    }
    let popular = widgets::flow(3);
    let tabs = gtk::Box::new(gtk::Orientation::Horizontal, 4);
    tabs.add_css_class("filter-tabs");
    let mut first: Option<gtk::ToggleButton> = None;
    let choices = std::iter::once(None).chain((0..TABS.min(catalog::SHELVES.len())).map(Some));
    for choice in choices {
        let label = choice.map_or("All", |i| catalog::SHELVES[i].title);
        let b = gtk::ToggleButton::with_label(label);
        b.set_group(first.as_ref());
        if first.is_none() {
            b.set_active(true);
            first = Some(b.clone());
        }
        let app2 = app.clone();
        let tab = tab.clone();
        let popular = popular.clone();
        b.connect_toggled(move |b| {
            if b.is_active() {
                tab.set(choice);
                fill_popular(&app2, &popular, choice);
            }
        });
        tabs.append(&b);
    }
    // Scrolls sideways when the window is too narrow for every tab.
    let tabs_scroller = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::External)
        .vscrollbar_policy(gtk::PolicyType::Never)
        .child(&tabs)
        .build();
    main.append(&tabs_scroller);
    main.append(&popular);

    // ---- browse by category ---------------------------------------------
    main.append(&widgets::section_header("Browse by Category", None));
    let browse = widgets::flow(catalog::SHELVES.len() as u32);
    browse.set_min_children_per_line(2);
    browse.set_column_spacing(12);
    for shelf in catalog::SHELVES {
        browse.insert(&browse_tile(app, shelf), -1);
    }
    main.append(&browse);

    // ---- side panel: updates --------------------------------------------
    let (updates_card, updates_body) = widgets::card("", "");
    updates_card.add_css_class("side-card");
    let head = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let ut = gtk::Label::new(Some("Updates"));
    ut.add_css_class("side-title");
    ut.set_xalign(0.0);
    head.append(&ut);
    let count = widgets::badge("0", "count");
    count.set_visible(false);
    head.append(&count);
    let spacer = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    head.append(&spacer);
    let update_all = gtk::Button::with_label("Update All");
    update_all.add_css_class("update-all");
    update_all.set_visible(false);
    {
        let app2 = app.clone();
        update_all.connect_clicked(move |_| app2.update_all());
    }
    head.append(&update_all);
    updates_body.append(&head);
    let updates_list = gtk::Box::new(gtk::Orientation::Vertical, 2);
    updates_body.append(&updates_list);
    {
        let app2 = app.clone();
        updates_body.append(&more_button(
            "View All Updates",
            Box::new(move || app2.navigate("updates")),
        ));
    }
    side.append(&updates_card);

    // ---- side panel: picks ----------------------------------------------
    let (picks_card, picks_body) = widgets::card("", "");
    picks_card.add_css_class("side-card");
    let ph = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let pi = gtk::Image::from_icon_name("starred-symbolic");
    pi.add_css_class("pick-star");
    ph.append(&pi);
    let pt = gtk::Label::new(Some("Raven Picks"));
    pt.add_css_class("side-title");
    ph.append(&pt);
    picks_body.append(&ph);
    let ps = gtk::Label::new(Some("Handpicked for a better computing experience."));
    ps.add_css_class("side-subtitle");
    ps.set_xalign(0.0);
    ps.set_wrap(true);
    picks_body.append(&ps);
    let picks = gtk::Box::new(gtk::Orientation::Vertical, 2);
    picks_body.append(&picks);
    {
        let app2 = app.clone();
        picks_body.append(&more_button(
            "Explore All Picks",
            Box::new(move || app2.navigate("categories")),
        ));
    }
    side.append(&picks_card);

    // ---- keep everything current ---------------------------------------
    let refresh = {
        let popular = popular.clone();
        let updates_list = updates_list.clone();
        let count = count.clone();
        let update_all = update_all.clone();
        let picks = picks.clone();
        move |app: &Rc<App>| {
            fill_popular(app, &popular, tab.get());
            for (slot, package) in &hero_actions {
                widgets::clear_box(slot);
                slot.append(&hero_action(app, package));
            }

            widgets::clear_box(&updates_list);
            let (cands, checked) = {
                let st = app.state.borrow();
                (st.updates.candidates.clone(), st.checked)
            };
            match (checked, cands.len()) {
                (false, _) => {
                    updates_list.append(&widgets::dim_label("Checking…"));
                    count.set_visible(false);
                    update_all.set_visible(false);
                }
                (true, 0) => {
                    updates_list.append(&widgets::dim_label("Everything is up to date."));
                    count.set_visible(false);
                    update_all.set_visible(false);
                }
                (true, n) => {
                    count.set_text(&n.to_string());
                    count.set_visible(true);
                    update_all.set_visible(true);
                    for c in cands.iter().take(3) {
                        updates_list.append(&update_row(app, c));
                    }
                }
            }

            widgets::clear_box(&picks);
            for pkg in catalog::PICKS {
                if let Some(e) = catalog::entry(pkg) {
                    picks.append(&pick_row(app, e));
                }
            }
        }
    };
    refresh(app);
    app.on_change(refresh);

    root.upcast()
}

/// Fill Popular Apps for a tab: the curated popular list for "All", or a
/// shelf's apps.
fn fill_popular(app: &Rc<App>, fb: &gtk::FlowBox, tab: Option<usize>) {
    widgets::clear_flow(fb);
    let entries: Vec<&Entry> = match tab {
        Some(i) => catalog::in_categories(catalog::SHELVES[i].categories),
        None => catalog::POPULAR
            .iter()
            .filter_map(|p| catalog::entry(p))
            .collect(),
    };
    for e in entries.into_iter().take(POPULAR_CARDS) {
        fb.insert(&widgets::app_card(app, &CardInfo::from_entry(e)), -1);
    }
}

/// A hero card, and the slot its install button is kept current in.
fn hero(app: &Rc<App>, e: &Entry, f: &Feature, index: usize) -> (gtk::Box, gtk::Box) {
    let card = gtk::Box::new(gtk::Orientation::Horizontal, 24);
    card.add_css_class("hero");
    card.add_css_class(&format!("hero-{}", index % 4));
    card.set_hexpand(true);

    let text = gtk::Box::new(gtk::Orientation::Vertical, 6);
    text.set_hexpand(true);
    text.set_valign(gtk::Align::Center);
    let badge = widgets::badge("Featured", "featured");
    badge.set_halign(gtk::Align::Start);
    badge.set_margin_bottom(4);
    text.append(&badge);
    let title = gtk::Label::new(Some(e.title));
    title.add_css_class("hero-title");
    title.set_xalign(0.0);
    text.append(&title);
    let headline = gtk::Label::new(Some(f.headline));
    headline.add_css_class("hero-headline");
    headline.set_xalign(0.0);
    text.append(&headline);
    let blurb = gtk::Label::new(Some(f.blurb));
    blurb.add_css_class("hero-text");
    blurb.set_xalign(0.0);
    blurb.set_wrap(true);
    blurb.set_max_width_chars(46);
    blurb.set_margin_top(6);
    text.append(&blurb);

    let buttons = gtk::Box::new(gtk::Orientation::Horizontal, 14);
    buttons.set_margin_top(16);
    let slot = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    slot.append(&hero_action(app, e.package));
    buttons.append(&slot);
    let learn = gtk::Button::new();
    let lb = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    lb.append(&gtk::Label::new(Some("Learn More")));
    lb.append(&gtk::Image::from_icon_name("go-next-symbolic"));
    learn.set_child(Some(&lb));
    learn.add_css_class("hero-secondary");
    {
        let app2 = app.clone();
        let name = e.package.to_string();
        let title = e.title.to_string();
        learn.connect_clicked(move |_| crate::ui::detail::show(&app2, &name, &title));
    }
    buttons.append(&learn);
    text.append(&buttons);

    // A flow rather than a row, so the chips wrap on a narrow window
    // instead of holding it wide.
    let chips = gtk::FlowBox::new();
    chips.set_selection_mode(gtk::SelectionMode::None);
    chips.set_column_spacing(28);
    chips.set_row_spacing(8);
    chips.set_max_children_per_line(f.chips.len().max(1) as u32);
    chips.set_margin_top(22);
    for (icon, label) in f.chips {
        let chip = gtk::Box::new(gtk::Orientation::Horizontal, 8);
        chip.add_css_class("hero-chip");
        chip.append(&gtk::Image::from_icon_name(icon));
        chip.append(&gtk::Label::new(Some(label)));
        chips.insert(&chip, -1);
    }
    text.append(&chips);
    card.append(&text);

    // The artwork: the app's icon in a glowing orb. The image ships no
    // brand art, so the orb carries the colour. The orb and the aside are
    // wide-only: they give way when the window narrows.
    let orb = gtk::Box::new(gtk::Orientation::Vertical, 0);
    orb.add_css_class("hero-orb");
    orb.add_css_class("wide-only");
    orb.add_css_class(&format!("orb-{}", index % 4));
    orb.set_valign(gtk::Align::Center);
    let icon = widgets::icon_for(app, e.package, Some(e.icon), 104);
    icon.set_vexpand(true);
    icon.set_valign(gtk::Align::Center);
    orb.append(&icon);
    card.append(&orb);

    let aside = gtk::Label::new(Some(f.aside));
    aside.add_css_class("hero-aside");
    aside.add_css_class("wide-only");
    aside.set_xalign(0.0);
    aside.set_wrap(true);
    aside.set_max_width_chars(16);
    aside.set_valign(gtk::Align::Center);
    aside.set_margin_end(12);
    card.append(&aside);
    (card, slot)
}

fn hero_action(app: &Rc<App>, package: &str) -> gtk::Button {
    let b = widgets::action_button(app, package);
    b.remove_css_class("suggested-action");
    b.add_css_class("hero-primary");
    b
}

fn browse_tile(app: &Rc<App>, shelf: &'static Shelf) -> gtk::Button {
    let bx = gtk::Box::new(gtk::Orientation::Vertical, 6);
    bx.set_halign(gtk::Align::Center);
    let icon = gtk::Image::from_icon_name(shelf.icon);
    icon.set_pixel_size(30);
    icon.set_margin_bottom(4);
    bx.append(&icon);
    let t = gtk::Label::new(Some(shelf.title));
    t.add_css_class("cat-title");
    bx.append(&t);
    let n = catalog::in_categories(shelf.categories).len();
    let c = gtk::Label::new(Some(&format!("{n} app{}", if n == 1 { "" } else { "s" })));
    c.add_css_class("cat-count");
    bx.append(&c);
    let b = gtk::Button::builder().child(&bx).build();
    b.add_css_class("browse-tile");
    b.add_css_class(&format!("shelf-{}", shelf.id));
    let app = app.clone();
    b.connect_clicked(move |_| super::categories::open_shelf(&app, shelf));
    b
}

/// A full-width outlined button: label on the left, chevron on the right.
fn more_button(label: &str, on_click: Box<dyn Fn()>) -> gtk::Button {
    let bx = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    let l = gtk::Label::new(Some(label));
    l.set_hexpand(true);
    l.set_xalign(0.0);
    bx.append(&l);
    let chevron = gtk::Image::from_icon_name("go-next-symbolic");
    chevron.add_css_class("chevron");
    bx.append(&chevron);
    let b = gtk::Button::builder().child(&bx).build();
    b.add_css_class("more-button");
    b.set_margin_top(6);
    b.connect_clicked(move |_| on_click());
    b
}

fn update_row(app: &Rc<App>, c: &crate::backend::rvn::Candidate) -> gtk::Box {
    let row = gtk::Box::new(gtk::Orientation::Horizontal, 14);
    row.add_css_class("update-row");
    let curated = catalog::entry(&c.name);
    let icon = widgets::icon_for(app, &c.name, curated.map(|e| e.icon), 30);
    icon.add_css_class("pkg-glyph");
    icon.set_valign(gtk::Align::Center);
    row.append(&icon);
    let text = gtk::Box::new(gtk::Orientation::Vertical, 1);
    text.set_hexpand(true);
    let name = gtk::Label::new(Some(curated.map(|e| e.title).unwrap_or(&c.name)));
    name.add_css_class("app-name");
    name.set_xalign(0.0);
    name.set_ellipsize(gtk::pango::EllipsizeMode::End);
    text.append(&name);
    let ver = gtk::Label::new(Some(&format!(
        "{} → {}",
        c.installed_version, c.new_version
    )));
    ver.add_css_class("version");
    ver.set_xalign(0.0);
    ver.set_ellipsize(gtk::pango::EllipsizeMode::End);
    text.append(&ver);
    let size = gtk::Label::new(Some(&if c.aur {
        "build from source".to_string()
    } else {
        human_bytes(c.download_size)
    }));
    size.add_css_class("version");
    size.set_xalign(0.0);
    text.append(&size);
    row.append(&text);
    let go = gtk::Button::from_icon_name("folder-download-symbolic");
    go.add_css_class("round-download");
    go.set_valign(gtk::Align::Center);
    go.set_tooltip_text(Some("Update"));
    {
        let app2 = app.clone();
        let name = c.name.clone();
        go.connect_clicked(move |_| app2.update(std::slice::from_ref(&name)));
    }
    row.append(&go);
    row
}

fn pick_row(app: &Rc<App>, e: &Entry) -> gtk::Button {
    let row = gtk::Box::new(gtk::Orientation::Horizontal, 14);
    let well = widgets::icon_well(app, e.package);
    well.add_css_class("pick-well");
    well.set_valign(gtk::Align::Center);
    let icon = widgets::icon_for(app, e.package, Some(e.icon), 24);
    icon.set_vexpand(true);
    icon.set_valign(gtk::Align::Center);
    well.append(&icon);
    row.append(&well);
    let text = gtk::Box::new(gtk::Orientation::Vertical, 1);
    text.set_hexpand(true);
    text.set_valign(gtk::Align::Center);
    let name = gtk::Label::new(Some(e.title));
    name.add_css_class("app-name");
    name.set_xalign(0.0);
    text.append(&name);
    let tagline = gtk::Label::new(Some(e.tagline));
    tagline.add_css_class("app-kind");
    tagline.set_xalign(0.0);
    tagline.set_ellipsize(gtk::pango::EllipsizeMode::End);
    tagline.set_max_width_chars(30);
    text.append(&tagline);
    row.append(&text);
    let chevron = gtk::Image::from_icon_name("go-next-symbolic");
    chevron.add_css_class("chevron");
    row.append(&chevron);
    let b = gtk::Button::builder().child(&row).build();
    b.add_css_class("flat");
    b.add_css_class("pick-row");
    let app2 = app.clone();
    let name = e.package.to_string();
    let title = e.title.to_string();
    b.connect_clicked(move |_| crate::ui::detail::show(&app2, &name, &title));
    b
}
