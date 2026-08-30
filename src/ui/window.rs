//! The shell of the window: sidebar with navigation, header with search, and
//! a stack of pages. Same bones as Raven Settings so the two feel like one
//! desktop.

use std::cell::RefCell;
use std::rc::Rc;

use gtk4 as gtk;
use libadwaita as adw;
use libadwaita::prelude::*;

use super::pages::{self, PageInfo};
use super::{widgets, App};
use crate::backend::system;

pub fn build(
    gtk_app: &adw::Application,
    app: &Rc<App>,
) -> (adw::ApplicationWindow, gtk::ListBox, gtk::Stack) {
    let window = adw::ApplicationWindow::builder()
        .application(gtk_app)
        .title("Raven Store")
        .default_width(1240)
        .default_height(800)
        .build();
    window.add_css_class("raven");

    let stack = gtk::Stack::builder()
        .transition_type(gtk::StackTransitionType::Crossfade)
        .hexpand(true)
        .vexpand(true)
        .build();

    // ---- sidebar --------------------------------------------------------
    let sidebar = gtk::Box::new(gtk::Orientation::Vertical, 10);
    sidebar.add_css_class("sidebar");
    sidebar.append(&brand());

    let nav = gtk::ListBox::new();
    nav.add_css_class("navigation-sidebar");
    nav.set_selection_mode(gtk::SelectionMode::Single);
    nav.set_vexpand(true);

    let infos = pages::all();
    let mut badges: Vec<(String, gtk::Label)> = Vec::new();
    for info in &infos {
        if info.separated {
            let sep = gtk::ListBoxRow::builder()
                .child(&gtk::Separator::new(gtk::Orientation::Horizontal))
                .selectable(false)
                .activatable(false)
                .build();
            sep.set_sensitive(false);
            nav.append(&sep);
        }
        let (row, badge) = nav_row(info);
        badges.push((info.id.to_string(), badge));
        nav.append(&row);
        let page = (info.build)(app);
        stack.add_named(&page, Some(info.id));
    }
    // Search results live outside the nav.
    stack.add_named(&pages::search::build(app), Some("search"));
    sidebar.append(&nav);
    sidebar.append(&status_card(app));

    // Map nav rows to page ids, skipping separator rows.
    let row_ids: Vec<Option<&'static str>> = {
        let mut v = Vec::new();
        for info in &infos {
            if info.separated {
                v.push(None);
            }
            v.push(Some(info.id));
        }
        v
    };
    {
        let stack = stack.clone();
        let row_ids = row_ids.clone();
        nav.connect_row_selected(move |_, row| {
            if let Some(row) = row {
                if let Some(Some(id)) = row_ids.get(row.index() as usize) {
                    stack.set_visible_child_name(id);
                }
            }
        });
    }
    nav.select_row(nav.row_at_index(0).as_ref());

    // Badge on Updates.
    {
        let badges = badges.clone();
        app.on_change(move |app| {
            for (id, badge) in &badges {
                if id == "updates" {
                    match app.update_count() {
                        Some(n) if n > 0 => {
                            badge.set_text(&n.to_string());
                            badge.set_visible(true);
                        }
                        _ => badge.set_visible(false),
                    }
                }
            }
        });
    }

    // ---- header: search -------------------------------------------------
    let search_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
    search_box.add_css_class("search-box");
    search_box.set_halign(gtk::Align::Center);
    search_box.set_hexpand(true);
    search_box.append(&gtk::Image::from_icon_name("system-search-symbolic"));
    let search = gtk::Entry::builder()
        .placeholder_text("Search apps, packages, and more…")
        .hexpand(true)
        .width_chars(36)
        .build();
    search_box.append(&search);
    let kbd = gtk::Label::new(Some("Ctrl+K"));
    kbd.add_css_class("kbd");
    search_box.append(&kbd);

    // Debounced: a query runs 350 ms after the last keystroke, or on Enter.
    let pending: Rc<RefCell<Option<glib::SourceId>>> = Rc::new(RefCell::new(None));
    let last_page: Rc<RefCell<String>> = Rc::new(RefCell::new("discover".into()));
    {
        let app = app.clone();
        let pending = pending.clone();
        let last_page = last_page.clone();
        search.connect_changed(move |e| {
            if let Some(id) = pending.borrow_mut().take() {
                id.remove();
            }
            let q = e.text().trim().to_string();
            if q.is_empty() {
                let back = last_page.borrow().clone();
                app.navigate(&back);
                return;
            }
            if q.chars().count() < 2 {
                return;
            }
            let app = app.clone();
            let pending2 = pending.clone();
            let last_page = last_page.clone();
            let id =
                glib::timeout_add_local_once(std::time::Duration::from_millis(350), move || {
                    *pending2.borrow_mut() = None;
                    remember(&app, &last_page);
                    pages::search::show(&app, &q);
                });
            *pending.borrow_mut() = Some(id);
        });
    }
    {
        let app = app.clone();
        let pending = pending.clone();
        let last_page = last_page.clone();
        search.connect_activate(move |e| {
            if let Some(id) = pending.borrow_mut().take() {
                id.remove();
            }
            let q = e.text().trim().to_string();
            if !q.is_empty() {
                remember(&app, &last_page);
                pages::search::show(&app, &q);
            }
        });
    }

    let header = adw::HeaderBar::builder()
        .title_widget(&search_box)
        .show_title(true)
        .build();
    let show_sidebar = gtk::ToggleButton::builder()
        .icon_name("sidebar-show-symbolic")
        .tooltip_text("Sections")
        .visible(false)
        .build();
    header.pack_start(&show_sidebar);
    let refresh = gtk::Button::from_icon_name("view-refresh-symbolic");
    refresh.set_tooltip_text(Some("Refresh repositories and check for updates"));
    {
        let app = app.clone();
        refresh.connect_clicked(move |_| app.refresh());
    }
    header.pack_end(&refresh);

    let toolbar = adw::ToolbarView::new();
    toolbar.set_top_bar_style(adw::ToolbarStyle::Raised);
    toolbar.add_top_bar(&header);
    toolbar.set_content(Some(&stack));

    let sidebar_scroller = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .propagate_natural_height(false)
        .child(&sidebar)
        .build();
    let split = adw::OverlaySplitView::builder()
        .sidebar(&sidebar_scroller)
        .content(&toolbar)
        .sidebar_width_fraction(0.22)
        .min_sidebar_width(220.0)
        .max_sidebar_width(260.0)
        .build();
    split
        .bind_property("show-sidebar", &show_sidebar, "active")
        .bidirectional()
        .sync_create()
        .build();

    let narrow = adw::Breakpoint::new(adw::BreakpointCondition::new_length(
        adw::BreakpointConditionLengthType::MaxWidth,
        900.0,
        adw::LengthUnit::Px,
    ));
    narrow.add_setter(&split, "collapsed", Some(&true.to_value()));
    narrow.add_setter(&show_sidebar, "visible", Some(&true.to_value()));
    {
        let stack = stack.clone();
        narrow.connect_apply(move |_| set_columns_stacked(&stack, true));
    }
    {
        let stack = stack.clone();
        narrow.connect_unapply(move |_| set_columns_stacked(&stack, false));
    }
    window.add_breakpoint(narrow);
    {
        let split = split.clone();
        nav.connect_row_activated(move |_, _| {
            if split.is_collapsed() {
                split.set_show_sidebar(false);
            }
        });
    }

    app.toasts.set_child(Some(&split));
    window.set_content(Some(&app.toasts));
    window.set_size_request(520, 380);
    window.set_default_size(1180, 760);

    // Ctrl+K (and Ctrl+F) focus search.
    let ctrl = gtk::ShortcutController::new();
    ctrl.set_scope(gtk::ShortcutScope::Global);
    for trigger in ["<Control>k", "<Control>f"] {
        let s2 = search.clone();
        ctrl.add_shortcut(gtk::Shortcut::new(
            gtk::ShortcutTrigger::parse_string(trigger),
            Some(gtk::CallbackAction::new(move |_, _| {
                s2.grab_focus();
                glib::Propagation::Stop
            })),
        ));
    }
    window.add_controller(ctrl);

    (window, nav, stack)
}

/// Note the page to return to when the search field is cleared.
fn remember(app: &App, last_page: &RefCell<String>) {
    if let Some(cur) = app.current_page() {
        if cur != "search" {
            *last_page.borrow_mut() = cur;
        }
    }
}

fn brand() -> gtk::Box {
    let bx = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    bx.add_css_class("brand");
    let icon = gtk::Image::from_icon_name("com.ravenstore.Raven");
    if !gtk::gdk::Display::default()
        .map(|d| gtk::IconTheme::for_display(&d).has_icon("com.ravenstore.Raven"))
        .unwrap_or(false)
    {
        icon.set_icon_name(Some("system-software-install-symbolic"));
    }
    bx.append(&icon);
    let text = gtk::Box::new(gtk::Orientation::Vertical, 0);
    text.set_valign(gtk::Align::Center);
    let t = gtk::Label::new(Some("Raven Store"));
    t.add_css_class("app-title");
    t.set_xalign(0.0);
    text.append(&t);
    let s = gtk::Label::new(Some("Raven Linux"));
    s.add_css_class("app-subtitle");
    s.set_xalign(0.0);
    text.append(&s);
    bx.append(&text);
    bx
}

fn nav_row(info: &PageInfo) -> (gtk::ListBoxRow, gtk::Label) {
    let bx = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    bx.append(&gtk::Image::from_icon_name(info.icon));
    let l = gtk::Label::new(Some(info.title));
    l.set_xalign(0.0);
    l.set_hexpand(true);
    bx.append(&l);
    let badge = widgets::badge("", "count");
    badge.set_visible(false);
    bx.append(&badge);
    (gtk::ListBoxRow::builder().child(&bx).build(), badge)
}

/// "System is up to date" at the foot of the sidebar; clicking goes to
/// Updates.
fn status_card(app: &Rc<App>) -> gtk::Button {
    let os = system::os_release();
    let bx = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let icon = gtk::Image::from_icon_name("object-select-symbolic");
    icon.add_css_class("success");
    bx.append(&icon);
    let text = gtk::Box::new(gtk::Orientation::Vertical, 1);
    text.set_hexpand(true);
    let t = gtk::Label::new(Some("Checking for updates…"));
    t.set_xalign(0.0);
    t.add_css_class("name");
    t.set_ellipsize(gtk::pango::EllipsizeMode::End);
    text.append(&t);
    let s = gtk::Label::new(Some(&format!("{} {}", os.name, os.version_id)));
    s.set_xalign(0.0);
    s.add_css_class("dim");
    s.set_ellipsize(gtk::pango::EllipsizeMode::End);
    text.append(&s);
    bx.append(&text);
    bx.append(&gtk::Image::from_icon_name("go-next-symbolic"));
    let button = gtk::Button::builder().child(&bx).build();
    button.add_css_class("raven-card");
    button.add_css_class("status-card");
    button.add_css_class("flat");
    {
        let app = app.clone();
        button.connect_clicked(move |_| app.navigate("updates"));
    }
    app.on_change(move |app| {
        let st = app.state.borrow();
        if let Some(e) = &st.error {
            t.set_text("Could not read packages");
            t.set_tooltip_text(Some(e));
            icon.set_icon_name(Some("dialog-warning-symbolic"));
            icon.remove_css_class("success");
            icon.add_css_class("warning");
            return;
        }
        if st.loading && !st.checked {
            t.set_text("Checking for updates…");
            return;
        }
        match st.checked.then_some(st.updates.candidates.len()) {
            Some(0) => {
                t.set_text("System is up to date");
                icon.set_icon_name(Some("object-select-symbolic"));
                icon.remove_css_class("warning");
                icon.add_css_class("success");
            }
            Some(n) => {
                t.set_text(&format!(
                    "{n} update{} available",
                    if n == 1 { "" } else { "s" }
                ));
                icon.set_icon_name(Some("software-update-available-symbolic"));
                icon.remove_css_class("success");
                icon.add_css_class("warning");
            }
            None => {}
        }
    });
    button
}

/// Stack (or unstack) every `.columns` row under `root`.
pub fn set_columns_stacked(root: &impl IsA<gtk::Widget>, stacked: bool) {
    fn walk(w: &gtk::Widget, stacked: bool) {
        if w.has_css_class("columns") {
            if let Some(b) = w.downcast_ref::<gtk::Box>() {
                b.set_orientation(if stacked {
                    gtk::Orientation::Vertical
                } else {
                    gtk::Orientation::Horizontal
                });
            }
        }
        let mut c = w.first_child();
        while let Some(ch) = c {
            walk(&ch, stacked);
            c = ch.next_sibling();
        }
    }
    walk(root.upcast_ref(), stacked);
}
