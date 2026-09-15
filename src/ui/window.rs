//! The shell of the window: a full-height sidebar with the brand and
//! navigation, a header carrying the search field, and a stack of pages.
//! Same bones as Raven Settings so the two feel like one desktop.

use std::cell::RefCell;
use std::rc::Rc;

use gtk4 as gtk;
use libadwaita as adw;
use libadwaita::prelude::*;

use super::pages::{self, PageInfo};
use super::{widgets, App};

pub fn build(
    gtk_app: &adw::Application,
    app: &Rc<App>,
) -> (adw::ApplicationWindow, gtk::ListBox, gtk::Stack) {
    let window = adw::ApplicationWindow::builder()
        .application(gtk_app)
        .title("Raven Store")
        .build();
    window.add_css_class("raven");

    let stack = gtk::Stack::builder()
        .transition_type(gtk::StackTransitionType::Crossfade)
        .hexpand(true)
        .vexpand(true)
        .build();

    // ---- sidebar --------------------------------------------------------
    let sidebar = gtk::Box::new(gtk::Orientation::Vertical, 6);
    sidebar.add_css_class("sidebar");
    // The sidebar has no header bar of its own, so the brand doubles as the
    // handle the window is dragged by.
    let handle = gtk::WindowHandle::new();
    handle.set_child(Some(&brand()));
    sidebar.append(&handle);

    let nav = gtk::ListBox::new();
    nav.add_css_class("navigation-sidebar");
    nav.set_selection_mode(gtk::SelectionMode::Single);

    let infos = pages::all();
    let mut badges: Vec<(String, gtk::Label)> = Vec::new();
    for info in &infos {
        if info.separated {
            let sep = gtk::ListBoxRow::builder()
                .child(&gtk::Separator::new(gtk::Orientation::Horizontal))
                .selectable(false)
                .activatable(false)
                .build();
            sep.add_css_class("nav-sep");
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
    let spacer = gtk::Box::new(gtk::Orientation::Vertical, 0);
    spacer.set_vexpand(true);
    sidebar.append(&spacer);
    sidebar.append(&motto());

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

    // ---- header: search ---------------------------------------------------
    let search = gtk::SearchEntry::builder()
        .placeholder_text("Search for apps, games, and more…")
        .hexpand(true)
        .build();
    let search_box = gtk::Box::new(gtk::Orientation::Horizontal, 4);
    search_box.add_css_class("top-search");
    search_box.set_hexpand(true);
    search_box.set_valign(gtk::Align::Center);
    search_box.append(&search);
    for key in ["Ctrl", "K"] {
        let k = gtk::Label::new(Some(key));
        k.add_css_class("kbd");
        k.set_valign(gtk::Align::Center);
        search_box.append(&k);
    }

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

    let clamp = adw::Clamp::builder()
        .maximum_size(940)
        .tightening_threshold(600)
        .hexpand(true)
        .child(&search_box)
        .build();
    let header = adw::HeaderBar::builder()
        .title_widget(&clamp)
        .show_title(true)
        .centering_policy(adw::CenteringPolicy::Loose)
        .build();
    header.add_css_class("store-header");
    let show_sidebar = gtk::ToggleButton::builder()
        .icon_name("sidebar-show-symbolic")
        .tooltip_text("Sections")
        .valign(gtk::Align::Center)
        .visible(false)
        .build();
    header.pack_start(&show_sidebar);
    let header_motto = gtk::Label::new(Some("Build   Use   Belong"));
    header_motto.add_css_class("header-motto");
    header.pack_end(&header_motto);

    let toolbar = adw::ToolbarView::new();
    toolbar.set_top_bar_style(adw::ToolbarStyle::Flat);
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
        .sidebar_width_fraction(0.19)
        .min_sidebar_width(240.0)
        .max_sidebar_width(270.0)
        .build();
    split
        .bind_property("show-sidebar", &show_sidebar, "active")
        .bidirectional()
        .sync_create()
        .build();

    // Two steps down: first the side panels drop under the page, then the
    // sidebar folds away. Only one breakpoint applies at a time (the last
    // one added that matches), so the narrow one stacks the columns too.
    let medium = adw::Breakpoint::new(adw::BreakpointCondition::new_length(
        adw::BreakpointConditionLengthType::MaxWidth,
        1180.0,
        adw::LengthUnit::Px,
    ));
    medium.add_setter(&header_motto, "visible", Some(&false.to_value()));
    let narrow = adw::Breakpoint::new(adw::BreakpointCondition::new_length(
        adw::BreakpointConditionLengthType::MaxWidth,
        860.0,
        adw::LengthUnit::Px,
    ));
    narrow.add_setter(&header_motto, "visible", Some(&false.to_value()));
    narrow.add_setter(&split, "collapsed", Some(&true.to_value()));
    narrow.add_setter(&show_sidebar, "visible", Some(&true.to_value()));
    for bp in [&medium, &narrow] {
        {
            let stack = stack.clone();
            bp.connect_apply(move |_| set_columns_stacked(&stack, true));
        }
        {
            let stack = stack.clone();
            bp.connect_unapply(move |_| set_columns_stacked(&stack, false));
        }
    }
    window.add_breakpoint(medium);
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
    window.set_default_size(1440, 900);

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
    // The masthead carries the Raven mark -- the distro's logo, as named by
    // /etc/os-release -- with the app's own icon and then a stock one as the
    // fallbacks for a system that has not installed it.
    let icon = gtk::Image::from_icon_name("system-software-install-symbolic");
    if let Some(display) = gtk::gdk::Display::default() {
        let theme = gtk::IconTheme::for_display(&display);
        for name in ["raven-logo", "com.ravenstore.Raven"] {
            if theme.has_icon(name) {
                icon.set_icon_name(Some(name));
                break;
            }
        }
    }
    bx.append(&icon);
    let text = gtk::Box::new(gtk::Orientation::Vertical, 2);
    text.set_valign(gtk::Align::Center);
    let t = gtk::Label::new(Some("Raven Store"));
    t.add_css_class("app-title");
    t.set_xalign(0.0);
    text.append(&t);
    let s = gtk::Label::new(Some("Software for a freer tomorrow."));
    s.add_css_class("app-subtitle");
    s.set_xalign(0.0);
    text.append(&s);
    bx.append(&text);
    bx
}

fn nav_row(info: &PageInfo) -> (gtk::ListBoxRow, gtk::Label) {
    let bx = gtk::Box::new(gtk::Orientation::Horizontal, 14);
    let icon = gtk::Image::from_icon_name(info.icon);
    icon.add_css_class("nav-glyph");
    bx.append(&icon);
    let l = gtk::Label::new(Some(info.title));
    l.set_xalign(0.0);
    l.set_hexpand(true);
    bx.append(&l);
    let badge = widgets::badge("", "count");
    badge.set_visible(false);
    bx.append(&badge);
    (gtk::ListBoxRow::builder().child(&bx).build(), badge)
}

/// The three lines at the foot of the sidebar.
fn motto() -> gtk::Box {
    let bx = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let l = gtk::Label::new(Some("Open Source.\nMore Control.\nA Better Tomorrow."));
    l.add_css_class("sidebar-motto");
    l.set_xalign(0.0);
    bx.append(&l);
    let rule = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    rule.add_css_class("motto-rule");
    rule.set_halign(gtk::Align::Start);
    bx.append(&rule);
    bx
}

/// Stack (or unstack) every `.columns` row under `root`, and hide every
/// `.wide-only` widget while stacked.
pub fn set_columns_stacked(root: &impl IsA<gtk::Widget>, stacked: bool) {
    fn walk(w: &gtk::Widget, stacked: bool) {
        if w.has_css_class("wide-only") {
            w.set_visible(!stacked);
        }
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
