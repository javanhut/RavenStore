//! The look: Raven's palette as libadwaita named colours (identical to
//! Settings so the two apps read as one desktop), plus the store's own
//! classes — hero cards, app tiles, badges.

use gtk::prelude::*;
use gtk4 as gtk;
use libadwaita as adw;

use crate::config::ThemeMode;

pub const BASE_CSS: &str = r#"
@define-color window_bg_color #16161f;
@define-color window_fg_color #d0d0e0;
@define-color headerbar_bg_color #16161f;
@define-color headerbar_fg_color #d0d0e0;
@define-color headerbar_border_color #2a2a3a;
@define-color headerbar_shade_color rgba(0,0,0,0.36);
@define-color view_bg_color #1a1a26;
@define-color view_fg_color #d0d0e0;
@define-color card_bg_color #1e1e2b;
@define-color card_fg_color #d0d0e0;
@define-color dialog_bg_color #1e1e2b;
@define-color dialog_fg_color #d0d0e0;
@define-color popover_bg_color #1e1e2b;
@define-color popover_fg_color #d0d0e0;
@define-color sidebar_bg_color #141420;
@define-color borders #2a2a3a;
window.raven { background-color: @window_bg_color; }
headerbar { box-shadow: none; border-bottom: 1px solid #2a2a3a; background-color: transparent; }
toolbarview, stack { background-color: transparent; }

window.raven.glass { background-color: alpha(#16161f, 0.72); }
window.raven.glass .sidebar { background-color: alpha(#0e0e16, 0.45); border-right-color: alpha(#ffffff, 0.07); }
window.raven.glass .card, window.raven.glass .raven-card, window.raven.glass .app-card {
  background-color: alpha(#ffffff, 0.085); border-color: alpha(#ffffff, 0.11);
}
window.raven.glass headerbar { border-bottom-color: alpha(#ffffff, 0.07); }
window.raven.glass list.boxed-list, window.raven.glass list.boxed-list row { background-color: alpha(#ffffff, 0.04); }
window.raven.glass entry, window.raven.glass .search-box { background-color: alpha(#ffffff, 0.07); }

.sidebar { background-color: #141420; border-right: 1px solid #2a2a3a; padding: 18px 14px; }
.sidebar .brand { margin: 0 8px 14px 8px; }
.sidebar .brand image { color: @accent_bg_color; -gtk-icon-size: 34px; }
.sidebar .app-title { font-size: 19px; font-weight: 700; }
.sidebar .app-subtitle { font-size: 12px; color: alpha(@window_fg_color, 0.55); }
.sidebar list.navigation-sidebar { background: transparent; }
.sidebar list.navigation-sidebar row { border-radius: 10px; padding: 9px 10px; margin: 2px 0; }
.sidebar list.navigation-sidebar row:selected {
  background-color: alpha(@accent_bg_color, 0.22); color: @window_fg_color;
  box-shadow: inset 0 0 0 1px alpha(@accent_bg_color, 0.55);
}
.sidebar list.navigation-sidebar row image { color: @accent_bg_color; }
.sidebar separator { margin: 8px 6px; }

.card, .raven-card { background-color: #1e1e2b; border: 1px solid #2a2a3a; border-radius: 14px; padding: 16px; }
.raven-card .card-title { font-weight: 600; font-size: 15px; }
.raven-card .card-subtitle, .page-subtitle, .dim { color: alpha(@window_fg_color, 0.6); }
.page-title { font-size: 24px; font-weight: 700; color: @window_fg_color; }
.section-title { font-size: 17px; font-weight: 700; }
.status-card { padding: 12px 14px; }
.badge {
  background-color: alpha(@accent_bg_color, 0.25); color: @accent_bg_color;
  border-radius: 999px; padding: 2px 8px; font-size: 11px; font-weight: 700;
}
.badge.count { min-width: 10px; padding: 1px 7px; }
.badge.origin { background-color: alpha(#ffffff, 0.09); color: alpha(@window_fg_color, 0.75); font-weight: 600; }
.badge.aur { background-color: alpha(#22C5DD, 0.2); color: #22C5DD; }
.badge.installed { background-color: alpha(#5FCF5F, 0.18); color: #5FCF5F; }
.badge.update { background-color: alpha(#F5A623, 0.2); color: #F5A623; }
.note { color: alpha(@window_fg_color, 0.55); font-size: 12px; }
.mono { font-family: monospace; font-size: 12px; }

/* Header search: a rounded field with the shortcut hint inside. */
.search-box {
  background-color: #1e1e2b; border: 1px solid #2a2a3a; border-radius: 12px;
  padding: 2px 10px; min-width: 320px;
}
.search-box entry, .search-box entry:focus { background: transparent; border: none; box-shadow: none; outline: none; min-height: 30px; }
.search-box .kbd {
  border: 1px solid alpha(@window_fg_color, 0.25); border-radius: 6px;
  padding: 0 6px; font-size: 11px; color: alpha(@window_fg_color, 0.6);
}

/* App tile: icon, name, kind, then the action button. */
.app-card {
  background-color: #1e1e2b; border: 1px solid #2a2a3a; border-radius: 14px;
  padding: 14px; min-width: 150px;
}
.app-card:hover { border-color: alpha(@accent_bg_color, 0.5); }
.app-card .app-name { font-weight: 600; font-size: 14px; }
.app-card .app-kind { font-size: 12px; color: alpha(@window_fg_color, 0.6); }
.app-card .app-icon { -gtk-icon-size: 40px; }
.icon-well { background-color: alpha(#ffffff, 0.06); border-radius: 12px; padding: 8px; }
.icon-well image { color: @accent_bg_color; }
.icon-well.web image { color: #3B9EFF; }        .icon-well.web { background-color: alpha(#3B9EFF, 0.14); }
.icon-well.productivity image { color: #F5A623; } .icon-well.productivity { background-color: alpha(#F5A623, 0.14); }
.icon-well.development image { color: #B279F7; } .icon-well.development { background-color: alpha(#B279F7, 0.14); }
.icon-well.media image { color: #F7768E; }      .icon-well.media { background-color: alpha(#F7768E, 0.14); }
.icon-well.graphics image { color: #22C5DD; }   .icon-well.graphics { background-color: alpha(#22C5DD, 0.14); }
.icon-well.communication image { color: #5FCF5F; } .icon-well.communication { background-color: alpha(#5FCF5F, 0.14); }
.icon-well.games image { color: #7AA2F7; }      .icon-well.games { background-color: alpha(#7AA2F7, 0.14); }
.icon-well.system image { color: #d0d0e0; }     .icon-well.system { background-color: alpha(#ffffff, 0.08); }
.icon-well.real { background-color: transparent; padding: 0; }
.app-card button.action { min-height: 26px; padding: 0 12px; border-radius: 8px; font-size: 12px; }
.app-card button.heart, button.heart { min-height: 24px; min-width: 24px; padding: 2px 6px; border-radius: 999px; font-size: 16px; color: alpha(@window_fg_color, 0.5); }
button.heart.on { color: #F7768E; }

/* Hero: a big featured card with a gradient backdrop. */
.hero { border-radius: 16px; padding: 26px 28px; min-height: 190px; border: 1px solid alpha(#ffffff, 0.1); }
.hero .hero-title { font-size: 26px; font-weight: 800; color: #ffffff; }
.hero .hero-text { font-size: 14px; color: alpha(#ffffff, 0.82); }
.hero .badge { background-color: alpha(#ffffff, 0.18); color: #ffffff; }
.hero .hero-icon { -gtk-icon-size: 120px; opacity: 0.95; }
.hero button.pill { border-radius: 10px; padding: 6px 16px; background-color: alpha(#ffffff, 0.16); color: #ffffff; border: 1px solid alpha(#ffffff, 0.25); }
.hero button.pill:hover { background-color: alpha(#ffffff, 0.26); }
.hero-0 { background-image: linear-gradient(135deg, #1c2d63, #3a2b7a 55%, #7a3fa0); }
.hero-1 { background-image: linear-gradient(135deg, #0f3a5a, #1c5f8a 55%, #2b8fb3); }
.hero-2 { background-image: linear-gradient(135deg, #3a1d4f, #7a2f5a 55%, #b8496a); }
.hero-3 { background-image: linear-gradient(135deg, #14323a, #1f5a4a 55%, #2f8a5a); }
carouselindicatordots { margin-top: 4px; }

/* Category tile */
.category-tile { background-color: #1e1e2b; border: 1px solid #2a2a3a; border-radius: 14px; padding: 18px 16px; min-width: 170px; }
.category-tile:hover { border-color: alpha(@accent_bg_color, 0.5); }
.category-tile image { color: @accent_bg_color; -gtk-icon-size: 28px; }
.category-tile .cat-title { font-weight: 700; font-size: 14px; }

/* Side panel on Discover */
.side-panel { min-width: 280px; }
.pick-row { padding: 6px 2px; }
.pick-row .rank { min-width: 18px; color: alpha(@window_fg_color, 0.55); font-weight: 700; }
.update-row { padding: 8px 4px; }
.update-row .version { font-size: 12px; color: alpha(@window_fg_color, 0.6); }
button.circular.accent { background-color: @accent_bg_color; color: #ffffff; }

/* Transaction dialog */
.tx-stage { font-weight: 600; }
.tx-log { font-family: monospace; font-size: 11.5px; background-color: #101018; border-radius: 10px; padding: 8px; }
.tx-log text { background-color: transparent; }
.success { color: #5FCF5F; }
.warning { color: #F5A623; }
.error { color: #F7768E; }
.footer-note { padding: 10px 0; }
.footer-note image { color: @accent_bg_color; }
.footer-note .t { font-weight: 600; font-size: 12px; color: @accent_bg_color; }
.footer-note .s { font-size: 11px; color: alpha(@window_fg_color, 0.55); }
"#;

pub fn load_base() {
    let provider = gtk::CssProvider::new();
    provider.load_from_string(BASE_CSS);
    let display = gtk::gdk::Display::default().expect("no display");
    gtk::style_context_add_provider_for_display(
        &display,
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

thread_local! {
    static ACCENT_PROVIDER: std::cell::RefCell<Option<gtk::CssProvider>> = const { std::cell::RefCell::new(None) };
}

/// Point every `@accent_bg_color` at the chosen hex, and set light/dark.
pub fn apply(window: Option<&adw::ApplicationWindow>, mode: ThemeMode, accent: &str, glass: bool) {
    if let Some(w) = window {
        if glass {
            w.add_css_class("glass");
        } else {
            w.remove_css_class("glass");
        }
    }
    let manager = adw::StyleManager::default();
    manager.set_color_scheme(match mode {
        ThemeMode::Dark => adw::ColorScheme::ForceDark,
        ThemeMode::Light => adw::ColorScheme::ForceLight,
        ThemeMode::Auto => adw::ColorScheme::PreferDark,
    });
    let accent = if is_hex(accent) {
        accent
    } else {
        crate::config::DEFAULT_ACCENT
    };
    let light = matches!(mode, ThemeMode::Light);
    let css = format!(
        "@define-color accent_bg_color {accent};\n@define-color accent_color {accent};\n{}",
        if light {
            "@define-color window_bg_color #eef0f6;\n@define-color window_fg_color #1a1b26;\n@define-color headerbar_bg_color #eef0f6;\n@define-color headerbar_fg_color #1a1b26;\n@define-color headerbar_border_color #d0d3e0;\n@define-color view_bg_color #f7f8fc;\n@define-color view_fg_color #1a1b26;\n@define-color card_bg_color #f7f8fc;\n@define-color card_fg_color #1a1b26;\n@define-color dialog_bg_color #f7f8fc;\n@define-color dialog_fg_color #1a1b26;\n@define-color popover_bg_color #f7f8fc;\n@define-color popover_fg_color #1a1b26;\n@define-color sidebar_bg_color #e6e8f0;\n@define-color borders #d0d3e0;\nheaderbar { border-bottom-color: #d0d3e0; }\n.sidebar { background-color: #e6e8f0; border-right-color: #d0d3e0; }\n.card, .raven-card, .app-card, .category-tile, .search-box { background-color: #f7f8fc; border-color: #d0d3e0; }\n.tx-log { background-color: #e6e8f0; }\nwindow.raven.glass { background-color: alpha(#eef0f6, 0.8); }\nwindow.raven.glass .sidebar { background-color: alpha(#ffffff, 0.35); }\nwindow.raven.glass .card, window.raven.glass .raven-card, window.raven.glass .app-card { background-color: alpha(#ffffff, 0.45); border-color: alpha(#000000, 0.08); }\n"
        } else {
            ""
        }
    );
    let display = gtk::gdk::Display::default().expect("no display");
    ACCENT_PROVIDER.with(|slot| {
        if let Some(old) = slot.borrow_mut().take() {
            gtk::style_context_remove_provider_for_display(&display, &old);
        }
        let provider = gtk::CssProvider::new();
        provider.load_from_string(&css);
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION + 1,
        );
        *slot.borrow_mut() = Some(provider);
    });
}

pub fn is_hex(s: &str) -> bool {
    s.len() == 7 && s.starts_with('#') && s[1..].chars().all(|c| c.is_ascii_hexdigit())
}
