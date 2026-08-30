//! Wishlist: packages the user hearted, ready to install later.

use std::rc::Rc;

use gtk4 as gtk;
use libadwaita::prelude::*;

use crate::catalog;
use crate::ui::widgets::{self, CardInfo};
use crate::ui::App;

pub fn build(app: &Rc<App>) -> gtk::Widget {
    let (root, content) = widgets::page(
        "Wishlist",
        "Apps you want to come back to. Tap the heart on any app to add it.",
    );
    let cards = widgets::flow(4);
    content.append(&cards);
    let empty = widgets::empty_state(
        "starred-symbolic",
        "Your wishlist is empty",
        "Heart an app anywhere in the store and it will wait for you here.",
    );
    content.append(&empty);

    let refresh = move |app: &Rc<App>| {
        widgets::clear_flow(&cards);
        let wishes = app.config.borrow().wishlist.clone();
        empty.set_visible(wishes.is_empty());
        for name in wishes {
            let info = match catalog::entry(&name) {
                Some(e) => CardInfo::from_entry(e),
                None => {
                    let installed = app.state.borrow().installed.get(&name).cloned();
                    match installed {
                        Some(p) => CardInfo::from_package(&p),
                        None => CardInfo {
                            package: name.clone(),
                            title: name.clone(),
                            kind: "package".into(),
                            icon: None,
                            description: String::new(),
                            origin: String::new(),
                            aur: false,
                            popularity: 0.0,
                        },
                    }
                }
            };
            cards.insert(&widgets::app_card(app, &info), -1);
        }
    };
    refresh(app);
    app.on_change(refresh);
    root.upcast()
}
