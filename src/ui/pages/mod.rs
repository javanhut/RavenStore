//! The pages, in sidebar order.

pub mod categories;
pub mod discover;
pub mod installed;
pub mod search;
pub mod settings;
pub mod updates;
pub mod wishlist;

use std::rc::Rc;

use gtk4 as gtk;

use super::App;

#[derive(Clone)]
pub struct PageInfo {
    pub id: &'static str,
    pub title: &'static str,
    pub icon: &'static str,
    pub build: fn(&Rc<App>) -> gtk::Widget,
    /// Settings sits below a separator.
    pub separated: bool,
    /// The colour of the icon tile beside the title in the sidebar; see
    /// `.nav-icon` in `data/raven-glass.css`. Names a domain, never the
    /// accent, so the sidebar stays legible under any accent.
    pub tint: &'static str,
}

pub fn all() -> Vec<PageInfo> {
    vec![
        PageInfo {
            id: "discover",
            title: "Discover",
            icon: "go-home-symbolic",
            build: discover::build,
            separated: false,
            tint: "blue",
        },
        PageInfo {
            id: "categories",
            title: "Categories",
            icon: "view-grid-symbolic",
            build: categories::build,
            separated: false,
            tint: "orange",
        },
        PageInfo {
            id: "installed",
            title: "Installed",
            icon: "folder-download-symbolic",
            build: installed::build,
            separated: false,
            tint: "green",
        },
        PageInfo {
            id: "updates",
            title: "Updates",
            icon: "view-refresh-symbolic",
            build: updates::build,
            separated: false,
            tint: "gray",
        },
        PageInfo {
            id: "wishlist",
            title: "Wishlist",
            icon: "starred-symbolic",
            build: wishlist::build,
            separated: false,
            tint: "pink",
        },
        PageInfo {
            id: "settings",
            title: "Settings",
            icon: "emblem-system-symbolic",
            build: settings::build,
            separated: true,
            tint: "graphite",
        },
    ]
}

pub fn ids() -> Vec<&'static str> {
    all().iter().map(|p| p.id).collect()
}
