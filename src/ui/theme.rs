//! The look: Raven Glass — the stylesheet shared with Settings and Power
//! (`data/raven-glass.css`, kept identical across the three repos) plus the
//! store's own classes — hero cards, app tiles, category tiles.

use gtk::prelude::*;
use gtk4 as gtk;
use libadwaita as adw;

use crate::config::ThemeMode;

pub const BASE_CSS: &str = concat!(
    include_str!("../../data/raven-glass.css"),
    r#"
/* ── Raven Store ─────────────────────────────────────────────────────── */

/* App tile: icon, name, kind, then the action button. */
.app-card {
  padding: 14px;
  min-width: 150px;
}
.app-card:hover { background-color: alpha(#ffffff, 0.09); border-color: alpha(#ffffff, 0.14); }
.app-card:active { background-color: alpha(#ffffff, 0.12); }
.app-card .app-name { font-weight: 600; font-size: 14px; letter-spacing: -0.1px; }
.app-card .app-kind, .app-kind { font-size: 12.5px; letter-spacing: 0.1px; color: alpha(@window_fg_color, 0.70); }
.app-card .app-icon { -gtk-icon-size: 40px; }
.icon-well { background-color: alpha(#ffffff, 0.07); border-radius: 12px; padding: 8px; box-shadow: inset 0 1px 0 alpha(#ffffff, 0.06); }
.icon-well image { color: @accent_bg_color; }
.icon-well.web image { color: #3B9EFF; }        .icon-well.web { background-color: alpha(#3B9EFF, 0.14); }
.icon-well.productivity image { color: #F5A623; } .icon-well.productivity { background-color: alpha(#F5A623, 0.14); }
.icon-well.development image { color: #B279F7; } .icon-well.development { background-color: alpha(#B279F7, 0.14); }
.icon-well.media image { color: #F7768E; }      .icon-well.media { background-color: alpha(#F7768E, 0.14); }
.icon-well.graphics image { color: #22C5DD; }   .icon-well.graphics { background-color: alpha(#22C5DD, 0.14); }
.icon-well.communication image { color: #5FCF5F; } .icon-well.communication { background-color: alpha(#5FCF5F, 0.14); }
.icon-well.games image { color: #7AA2F7; }      .icon-well.games { background-color: alpha(#7AA2F7, 0.14); }
.icon-well.system image { color: @window_fg_color; }     .icon-well.system { background-color: alpha(#ffffff, 0.08); }
.icon-well.real { background-color: transparent; padding: 0; box-shadow: none; }
.app-card button.action { min-height: 28px; padding: 0 12px; border-radius: 8px; font-size: 12px; }
.app-card button.heart, button.heart {
  min-height: 24px; min-width: 24px; padding: 2px 6px; border-radius: 999px;
  font-size: 16px; color: alpha(@window_fg_color, 0.60);
}
button.heart.on { color: #F7768E; }

/* Hero: a big featured card with a gradient backdrop. */
.hero {
  border-radius: 20px;
  padding: 28px 30px;
  min-height: 190px;
  border: 1px solid alpha(#ffffff, 0.12);
  box-shadow: inset 0 1px 0 alpha(#ffffff, 0.18), 0 12px 32px alpha(#000000, 0.30);
}
.hero .hero-title { font-size: 28px; font-weight: 800; letter-spacing: -0.8px; color: #ffffff; }
.hero .hero-text { font-size: 14px; color: alpha(#ffffff, 0.82); }
.hero .badge { background-color: alpha(#ffffff, 0.18); color: #ffffff; }
.hero .hero-icon { -gtk-icon-size: 120px; opacity: 0.95; }
.hero button.pill {
  border-radius: 999px; padding: 0 18px; min-height: 32px;
  background-color: alpha(#ffffff, 0.16); color: #ffffff;
  border: 1px solid alpha(#ffffff, 0.28);
  box-shadow: inset 0 1px 0 alpha(#ffffff, 0.22);
}
.hero button.pill:hover { background-color: alpha(#ffffff, 0.26); }
.hero button.pill:active { background-color: alpha(#ffffff, 0.34); }
.hero-0 { background-image: linear-gradient(135deg, #17254f, #2f2264 55%, #66358a); }
.hero-1 { background-image: linear-gradient(135deg, #0b2f4b, #174f74 55%, #24799a); }
.hero-2 { background-image: linear-gradient(135deg, #2f1641, #66274b 55%, #9e3d5b); }
.hero-3 { background-image: linear-gradient(135deg, #0f2a30, #194b3e 55%, #27754d); }
carouselindicatordots { margin-top: 4px; }

/* Category tile */
.category-tile { padding: 18px 16px; min-width: 170px; }
.category-tile:hover { background-color: alpha(#ffffff, 0.09); border-color: alpha(#ffffff, 0.14); }
.category-tile:active { background-color: alpha(#ffffff, 0.12); }
.category-tile image { color: @accent_bg_color; -gtk-icon-size: 28px; }
.category-tile .cat-title { font-weight: 600; font-size: 14px; letter-spacing: -0.1px; }

/* Side panel on Discover */
.side-panel { min-width: 280px; }
.pick-row { padding: 6px 2px; }
.pick-row .rank { min-width: 18px; color: alpha(@window_fg_color, 0.70); font-weight: 700; }
.update-row { padding: 8px 4px; }
.update-row .version { font-size: 12.5px; letter-spacing: 0.1px; color: alpha(@window_fg_color, 0.70); }

/* Transaction dialog */
.tx-stage { font-weight: 600; }
.tx-log {
  font-family: monospace; font-size: 11.5px;
  background-color: alpha(#000000, 0.30);
  border: 1px solid alpha(#ffffff, 0.08);
  border-radius: 10px; padding: 8px;
}
.tx-log text { background-color: transparent; }
.footer-note { padding: 10px 0; }
.footer-note image { color: @accent_bg_color; }
.footer-note .t { font-weight: 600; font-size: 12px; color: @accent_bg_color; }
.footer-note .s { font-size: 11.5px; letter-spacing: 0.1px; color: alpha(@window_fg_color, 0.70); }
"#
);

const LIGHT_CSS: &str = concat!(
    include_str!("../../data/raven-glass-light.css"),
    ".tx-log { background-color: alpha(#000000, 0.06); border-color: alpha(#000000, 0.08); }\n"
);

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
        if light { LIGHT_CSS } else { "" }
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
