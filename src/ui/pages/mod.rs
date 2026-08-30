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
}

pub fn all() -> Vec<PageInfo> {
    vec![
        PageInfo {
            id: "discover",
            title: "Discover",
            icon: "go-home-symbolic",
            build: discover::build,
            separated: false,
        },
        PageInfo {
            id: "categories",
            title: "Categories",
            icon: "view-grid-symbolic",
            build: categories::build,
            separated: false,
        },
        PageInfo {
            id: "installed",
            title: "Installed",
            icon: "folder-download-symbolic",
            build: installed::build,
            separated: false,
        },
        PageInfo {
            id: "updates",
            title: "Updates",
            icon: "view-refresh-symbolic",
            build: updates::build,
            separated: false,
        },
        PageInfo {
            id: "wishlist",
            title: "Wishlist",
            icon: "starred-symbolic",
            build: wishlist::build,
            separated: false,
        },
        PageInfo {
            id: "settings",
            title: "Settings",
            icon: "emblem-system-symbolic",
            build: settings::build,
            separated: true,
        },
    ]
}

pub fn ids() -> Vec<&'static str> {
    all().iter().map(|p| p.id).collect()
}
