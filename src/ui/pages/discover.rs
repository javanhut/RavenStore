//! Discover: featured apps, popular picks, a couple of categories, and a
//! side panel with pending updates.

use std::rc::Rc;

use gtk4 as gtk;
use libadwaita as adw;
use libadwaita::prelude::*;

use crate::backend::human_bytes;
use crate::catalog::{self, Entry};
use crate::ui::widgets::{self, CardInfo};
use crate::ui::App;

pub fn build(app: &Rc<App>) -> gtk::Widget {
    let (root, content) = widgets::page("", "");

    let columns = gtk::Box::new(gtk::Orientation::Horizontal, 18);
    columns.add_css_class("columns");
    let main = gtk::Box::new(gtk::Orientation::Vertical, 12);
    main.set_hexpand(true);
    let side = gtk::Box::new(gtk::Orientation::Vertical, 14);
    side.add_css_class("side-panel");
    side.set_valign(gtk::Align::Start);
    columns.append(&main);
    columns.append(&side);
    content.append(&columns);

    // ---- featured -------------------------------------------------------
    main.append(&widgets::section_header("Featured", None));
    let carousel = adw::Carousel::new();
    carousel.set_spacing(12);
    carousel.set_allow_scroll_wheel(false);
    for (i, (pkg, tag)) in catalog::FEATURED.iter().enumerate() {
        if let Some(e) = catalog::entry(pkg) {
            carousel.append(&hero(app, e, tag, i));
        }
    }
    let dots = adw::CarouselIndicatorDots::new();
    dots.set_carousel(Some(&carousel));
    main.append(&carousel);
    main.append(&dots);

    // ---- popular --------------------------------------------------------
    {
        let app2 = app.clone();
        main.append(&widgets::section_header(
            "Popular Apps",
            Some(Box::new(move || app2.navigate("categories"))),
        ));
    }
    let popular = widgets::flow(4);
    main.append(&popular);

    // ---- two categories -------------------------------------------------
    let mut category_flows = Vec::new();
    for id in ["productivity", "development"] {
        let Some(cat) = catalog::category(id) else {
            continue;
        };
        let app2 = app.clone();
        main.append(&widgets::section_header(
            cat.title,
            Some(Box::new(move || super::categories::open(&app2, id))),
        ));
        let fb = widgets::flow(4);
        main.append(&fb);
        category_flows.push((id, fb));
    }

    // ---- side panel -----------------------------------------------------
    let (updates_card, updates_body) = widgets::card("", "");
    let head = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    let ut = gtk::Label::new(Some("Updates"));
    ut.add_css_class("section-title");
    ut.set_xalign(0.0);
    head.append(&ut);
    let count = widgets::badge("0", "count");
    count.set_visible(false);
    head.append(&count);
    let spacer = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    head.append(&spacer);
    let update_all = gtk::Button::with_label("Update All");
    update_all.add_css_class("suggested-action");
    update_all.set_visible(false);
    {
        let app2 = app.clone();
        update_all.connect_clicked(move |_| app2.update_all());
    }
    head.append(&update_all);
    updates_body.append(&head);
    let updates_list = gtk::Box::new(gtk::Orientation::Vertical, 6);
    updates_body.append(&updates_list);
    let view_all = gtk::Button::new();
    let va = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    let val = gtk::Label::new(Some("View All Updates"));
    val.set_hexpand(true);
    val.set_xalign(0.0);
    va.append(&val);
    va.append(&gtk::Image::from_icon_name("go-next-symbolic"));
    view_all.set_child(Some(&va));
    {
        let app2 = app.clone();
        view_all.connect_clicked(move |_| app2.navigate("updates"));
    }
    updates_body.append(&view_all);
    side.append(&updates_card);

    let (picks_card, picks_body) = widgets::card("", "");
    let ph = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    let pi = gtk::Image::from_icon_name("starred-symbolic");
    pi.add_css_class("accent");
    ph.append(&pi);
    let pt = gtk::Label::new(Some("Raven Picks"));
    pt.add_css_class("section-title");
    ph.append(&pt);
    picks_body.append(&ph);
    let picks = gtk::Box::new(gtk::Orientation::Vertical, 2);
    picks_body.append(&picks);
    let explore = gtk::Button::new();
    let eb = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    eb.append(&gtk::Image::from_icon_name("view-grid-symbolic"));
    let el = gtk::Label::new(Some("Explore All Categories"));
    el.set_hexpand(true);
    el.set_xalign(0.0);
    eb.append(&el);
    eb.append(&gtk::Image::from_icon_name("go-next-symbolic"));
    explore.set_child(Some(&eb));
    {
        let app2 = app.clone();
        explore.connect_clicked(move |_| app2.navigate("categories"));
    }
    picks_body.append(&explore);
    side.append(&picks_card);

    content.append(&footer());

    // ---- keep everything current ---------------------------------------
    let refresh = {
        let popular = popular.clone();
        let category_flows = category_flows.clone();
        let updates_list = updates_list.clone();
        let count = count.clone();
        let update_all = update_all.clone();
        let picks = picks.clone();
        move |app: &Rc<App>| {
            widgets::clear_flow(&popular);
            for pkg in catalog::POPULAR {
                if let Some(e) = catalog::entry(pkg) {
                    popular.insert(&widgets::app_card(app, &CardInfo::from_entry(e)), -1);
                }
            }
            for (id, fb) in &category_flows {
                widgets::clear_flow(fb);
                for e in catalog::in_category(id).into_iter().take(4) {
                    fb.insert(&widgets::app_card(app, &CardInfo::from_entry(e)), -1);
                }
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
            for (i, pkg) in catalog::PICKS.iter().enumerate() {
                if let Some(e) = catalog::entry(pkg) {
                    picks.append(&pick_row(app, e, i + 1));
                }
            }
        }
    };
    refresh(app);
    app.on_change(refresh);

    root.upcast()
}

fn hero(app: &Rc<App>, e: &Entry, tag: &str, index: usize) -> gtk::Box {
    let card = gtk::Box::new(gtk::Orientation::Horizontal, 20);
    card.add_css_class("hero");
    card.add_css_class(&format!("hero-{}", index % 4));
    card.set_hexpand(true);

    let text = gtk::Box::new(gtk::Orientation::Vertical, 8);
    text.set_hexpand(true);
    text.set_valign(gtk::Align::Center);
    let badge = widgets::badge(tag, "");
    badge.set_halign(gtk::Align::Start);
    text.append(&badge);
    let title = gtk::Label::new(Some(e.title));
    title.add_css_class("hero-title");
    title.set_xalign(0.0);
    text.append(&title);
    let tagline = gtk::Label::new(Some(e.tagline));
    tagline.add_css_class("hero-text");
    tagline.set_xalign(0.0);
    tagline.set_wrap(true);
    tagline.set_max_width_chars(40);
    text.append(&tagline);

    let buttons = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    buttons.set_margin_top(6);
    let action = widgets::action_button(app, e.package);
    action.remove_css_class("suggested-action");
    action.add_css_class("pill");
    if let Some(l) = action
        .child()
        .and_then(|c| c.first_child())
        .and_downcast::<gtk::Label>()
    {
        if l.text() == "Install" {
            l.set_text("Install Now");
        }
    }
    buttons.append(&action);
    let view = gtk::Button::with_label("View App");
    view.add_css_class("pill");
    {
        let app2 = app.clone();
        let name = e.package.to_string();
        let title = e.title.to_string();
        view.connect_clicked(move |_| crate::ui::detail::show(&app2, &name, &title));
    }
    buttons.append(&view);
    text.append(&buttons);
    card.append(&text);

    let icon = widgets::icon_for(app, e.package, Some(e.icon), 120);
    icon.add_css_class("hero-icon");
    icon.set_valign(gtk::Align::Center);
    icon.set_margin_end(10);
    card.append(&icon);
    card
}

fn update_row(app: &Rc<App>, c: &crate::backend::rvn::Candidate) -> gtk::Box {
    let row = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    row.add_css_class("update-row");
    let curated = catalog::entry(&c.name);
    let icon = widgets::icon_for(app, &c.name, curated.map(|e| e.icon), 28);
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
    let go = gtk::Button::from_icon_name("go-up-symbolic");
    go.add_css_class("circular");
    go.add_css_class("accent");
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

fn pick_row(app: &Rc<App>, e: &Entry, rank: usize) -> gtk::Button {
    let row = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    row.add_css_class("pick-row");
    let r = gtk::Label::new(Some(&rank.to_string()));
    r.add_css_class("rank");
    row.append(&r);
    row.append(&widgets::icon_for(app, e.package, Some(e.icon), 22));
    let name = gtk::Label::new(Some(e.title));
    name.set_xalign(0.0);
    name.set_hexpand(true);
    row.append(&name);
    if app.is_installed(e.package) {
        row.append(&widgets::badge("installed", "installed"));
    }
    let b = gtk::Button::builder().child(&row).build();
    b.add_css_class("flat");
    let app2 = app.clone();
    let name = e.package.to_string();
    let title = e.title.to_string();
    b.connect_clicked(move |_| crate::ui::detail::show(&app2, &name, &title));
    b
}

fn footer() -> gtk::Box {
    let row = gtk::Box::new(gtk::Orientation::Horizontal, 30);
    row.add_css_class("footer-note");
    row.set_margin_top(10);
    row.set_homogeneous(true);
    for (icon, t, s) in [
        (
            "security-high-symbolic",
            "Secure & Trusted",
            "Packages are checksum and signature verified",
        ),
        (
            "applications-engineering-symbolic",
            "Open Source First",
            "Built on the Arch repositories and the AUR",
        ),
        (
            "system-users-symbolic",
            "Community Driven",
            "By Linux users, for everyone",
        ),
    ] {
        let item = gtk::Box::new(gtk::Orientation::Horizontal, 10);
        let i = gtk::Image::from_icon_name(icon);
        i.set_pixel_size(22);
        item.append(&i);
        let text = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let tl = gtk::Label::new(Some(t));
        tl.add_css_class("t");
        tl.set_xalign(0.0);
        text.append(&tl);
        let sl = gtk::Label::new(Some(s));
        sl.add_css_class("s");
        sl.set_xalign(0.0);
        sl.set_wrap(true);
        text.append(&sl);
        item.append(&text);
        row.append(&item);
    }
    row
}
