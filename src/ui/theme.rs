//! The look: Raven Glass — the stylesheet shared with Settings and Power
//! (`data/raven-glass.css`, kept identical across the three repos) plus the
//! store's own classes — the shell, hero cards, app tiles, the side panel.

use gtk::prelude::*;
use gtk4 as gtk;
use libadwaita as adw;

use crate::config::ThemeMode;

pub const BASE_CSS: &str = concat!(
    include_str!("../../data/raven-glass.css"),
    r#"
/* ── Raven Store ─────────────────────────────────────────────────────── */

/* The ground: deep night blue, a little lighter toward the top left. */
window.raven {
  background-color: #0b1222;
  background-image:
    radial-gradient(ellipse at 18% 0%, alpha(#1e3a8a, 0.30), transparent 60%),
    radial-gradient(ellipse at 100% 100%, alpha(#0e7490, 0.14), transparent 55%),
    linear-gradient(160deg, #0e1729, #0a1020 60%, #0b1224);
}
window.raven.glass {
  background-color: alpha(#0b1222, 0.80);
  background-image: linear-gradient(160deg, alpha(#1a2a55, 0.30), alpha(#0a1020, 0.05));
}
window.raven.glass .sidebar { background-color: alpha(#08101f, 0.45); }

/* ── Sidebar ─────────────────────────────────────────────────────────── */
.sidebar { padding: 26px 16px 26px 16px; border-right: 1px solid alpha(#ffffff, 0.06); }
.sidebar .brand { margin: 0 6px 30px 8px; }
.sidebar .brand image { -gtk-icon-size: 56px; color: @accent_bg_color; }
.sidebar .app-title { font-size: 21px; font-weight: 600; letter-spacing: -0.2px; }
.sidebar .app-subtitle { font-size: 11.5px; color: alpha(@window_fg_color, 0.70); }
.sidebar list.navigation-sidebar row {
  min-height: 46px;
  padding: 0 16px;
  margin: 3px 0;
  border-radius: 12px;
  border: 1px solid transparent;
}
.sidebar list.navigation-sidebar row:selected {
  background-color: transparent;
  background-image: linear-gradient(90deg, alpha(@accent_bg_color, 0.30), alpha(@accent_bg_color, 0.10));
  border-color: alpha(@accent_bg_color, 0.55);
  box-shadow: 0 0 18px alpha(@accent_bg_color, 0.22), inset 0 1px 0 alpha(#ffffff, 0.10);
}
.sidebar list.navigation-sidebar row label { font-size: 15px; }
.sidebar list.navigation-sidebar row.nav-sep { min-height: 0; padding: 0; margin: 0; border: none; }
.sidebar separator { margin: 18px 10px; }
.sidebar .nav-glyph { -gtk-icon-size: 20px; color: alpha(@window_fg_color, 0.85); }
.sidebar list.navigation-sidebar row:selected .nav-glyph { color: #ffffff; }
.sidebar .badge.count, .side-card .badge.count, .section-heading + .badge.count {
  background-color: alpha(@accent_bg_color, 0.18);
  color: @accent_bg_color;
  border: 1px solid alpha(@accent_bg_color, 0.45);
  padding: 1px 8px;
}
.sidebar list.navigation-sidebar row:selected .badge { background-color: alpha(@accent_bg_color, 0.30); color: #ffffff; }
.sidebar-motto { font-size: 15px; color: alpha(@window_fg_color, 0.62); margin: 0 12px; }
.motto-rule { min-height: 2px; min-width: 28px; border-radius: 2px; background-color: @accent_bg_color; margin: 14px 12px 0 12px; }

/* ── Header: the search field ────────────────────────────────────────── */
headerbar.store-header { min-height: 78px; padding: 0 16px 0 22px; border-bottom: none; background: transparent; }
.top-search {
  min-height: 46px;
  border-radius: 12px;
  padding: 0 10px 0 4px;
  background-color: alpha(#ffffff, 0.06);
  border: 1px solid alpha(#ffffff, 0.10);
  box-shadow: inset 0 1px 0 alpha(#ffffff, 0.05);
}
.top-search:focus-within { border-color: alpha(@accent_bg_color, 0.60); box-shadow: 0 0 0 3px alpha(@accent_bg_color, 0.16); }
.top-search entry, .top-search entry:focus-within { background: transparent; border: none; box-shadow: none; min-height: 42px; font-size: 15px; }
.top-search entry image { -gtk-icon-size: 18px; color: alpha(@window_fg_color, 0.70); margin: 0 6px; }
.top-search .kbd { min-height: 22px; padding: 0 7px; font-size: 12px; }
headerbar.store-header windowcontrols button {
  min-width: 34px; min-height: 34px; padding: 0; margin: 0 2px;
  background: transparent; border: none; box-shadow: none;
  color: alpha(@window_fg_color, 0.80);
}
headerbar.store-header windowcontrols button:hover { background-color: alpha(#ffffff, 0.08); }
headerbar.store-header windowcontrols button image { background: transparent; box-shadow: none; }
.header-motto { font-size: 14px; color: alpha(@window_fg_color, 0.30); letter-spacing: 0.3px; margin: 0 22px 0 30px; }

/* ── Section headings and filter tabs ────────────────────────────────── */
.section-heading { font-size: 20px; font-weight: 600; letter-spacing: -0.2px; }
button.view-all { color: @accent_bg_color; font-size: 14px; font-weight: 400; padding: 0 6px; }
button.view-all image { -gtk-icon-size: 14px; }
.filter-tabs button {
  min-height: 32px; padding: 0 14px; border-radius: 8px;
  background-color: transparent; border: 1px solid transparent; box-shadow: none;
  font-size: 14px; font-weight: 400; color: alpha(@window_fg_color, 0.82);
}
.filter-tabs button:hover { background-color: alpha(#ffffff, 0.06); }
.filter-tabs button:checked {
  background-color: alpha(@accent_bg_color, 0.16);
  border-color: alpha(@accent_bg_color, 0.50);
  color: #ffffff;
  box-shadow: none;
}

/* ── App tile: icon, name, kind, then meta beside the action. ────────── */
.app-card {
  padding: 16px 16px 14px 18px;
  min-width: 220px;
  border-radius: 14px;
  background-color: alpha(#ffffff, 0.045);
  border: 1px solid alpha(#ffffff, 0.08);
}
.app-card:hover { background-color: alpha(#ffffff, 0.075); border-color: alpha(@accent_bg_color, 0.30); }
.app-card:active { background-color: alpha(#ffffff, 0.10); }
.app-name { font-weight: 600; font-size: 15px; letter-spacing: -0.1px; }
.app-kind { font-size: 13px; letter-spacing: 0.1px; color: alpha(@window_fg_color, 0.62); }
.app-meta label { font-size: 13px; color: alpha(@window_fg_color, 0.68); }
.app-meta .glyph { font-size: 12px; color: @accent_bg_color; }
.app-meta.installed .glyph { color: @success_color; }
.app-meta.update .glyph { color: @warning_color; }
.icon-well {
  min-width: 56px; min-height: 56px;
  border-radius: 14px;
  background-color: alpha(#ffffff, 0.07);
  box-shadow: inset 0 1px 0 alpha(#ffffff, 0.06);
}
.icon-well image { color: @accent_bg_color; }
.icon-well.web image { color: #3B9EFF; }        .icon-well.web { background-color: alpha(#3B9EFF, 0.14); }
.icon-well.productivity image { color: #F5A623; } .icon-well.productivity { background-color: alpha(#F5A623, 0.14); }
.icon-well.development image { color: #5AA8FF; } .icon-well.development { background-color: alpha(#5AA8FF, 0.14); }
.icon-well.media image { color: #F7768E; }      .icon-well.media { background-color: alpha(#F7768E, 0.14); }
.icon-well.graphics image { color: #22C5DD; }   .icon-well.graphics { background-color: alpha(#22C5DD, 0.14); }
.icon-well.communication image { color: #8B8CF8; } .icon-well.communication { background-color: alpha(#8B8CF8, 0.16); }
.icon-well.games image { color: #A78BFA; }      .icon-well.games { background-color: alpha(#A78BFA, 0.14); }
.icon-well.system image { color: @window_fg_color; }     .icon-well.system { background-color: alpha(#ffffff, 0.08); }
.icon-well.customization image { color: #4ADE80; } .icon-well.customization { background-color: alpha(#4ADE80, 0.14); }
.icon-well.real { background-color: transparent; box-shadow: none; }
.app-card button.action, .app-card button.action.suggested-action {
  min-height: 32px; padding: 0 20px; border-radius: 8px;
  font-size: 14px; font-weight: 600;
  background-color: alpha(@accent_bg_color, 0.12);
  background-image: none;
  color: #e6fbff;
  border: 1px solid alpha(@accent_bg_color, 0.45);
  box-shadow: inset 0 1px 0 alpha(#ffffff, 0.06);
}
.app-card button.action:hover { background-color: alpha(@accent_bg_color, 0.24); }
button.heart {
  min-height: 24px; min-width: 24px; padding: 2px 6px; border-radius: 999px;
  font-size: 16px; color: alpha(@window_fg_color, 0.60);
}
button.heart.on { color: #F7768E; }

/* ── Hero: the featured carousel ─────────────────────────────────────── */
.hero {
  border-radius: 20px;
  padding: 26px 30px 24px 30px;
  min-height: 300px;
  border: 1px solid alpha(#ffffff, 0.10);
  box-shadow: inset 0 1px 0 alpha(#ffffff, 0.14);
}
.hero-0 {
  background-image:
    radial-gradient(ellipse at 62% 0%, alpha(#8b5cf6, 0.55), transparent 55%),
    radial-gradient(ellipse at 42% 30%, alpha(#22d3ee, 0.26), transparent 45%),
    linear-gradient(180deg, #161d46, #1b1a48 50%, #0d1230);
}
.hero-1 {
  background-image:
    radial-gradient(ellipse at 60% 0%, alpha(#0ea5e9, 0.50), transparent 55%),
    radial-gradient(ellipse at 35% 35%, alpha(#6366f1, 0.25), transparent 45%),
    linear-gradient(180deg, #0f2140, #10284a 50%, #0a1428);
}
.hero-2 {
  background-image:
    radial-gradient(ellipse at 62% 0%, alpha(#db2777, 0.45), transparent 55%),
    radial-gradient(ellipse at 40% 30%, alpha(#8b5cf6, 0.30), transparent 45%),
    linear-gradient(180deg, #231437, #2a1540 50%, #120c24);
}
.hero-3 {
  background-image:
    radial-gradient(ellipse at 62% 0%, alpha(#10b981, 0.42), transparent 55%),
    radial-gradient(ellipse at 38% 30%, alpha(#22d3ee, 0.24), transparent 45%),
    linear-gradient(180deg, #0e2530, #102c33 50%, #09161e);
}
.hero .badge.featured {
  background-color: alpha(#ffffff, 0.10);
  color: alpha(#ffffff, 0.90);
  border: 1px solid alpha(#ffffff, 0.14);
  border-radius: 8px;
  padding: 3px 10px;
  font-size: 12px; font-weight: 500;
}
.hero-title { font-size: 44px; font-weight: 800; letter-spacing: -1px; color: #ffffff; }
.hero-headline { font-size: 24px; font-weight: 500; letter-spacing: 0.3px; color: alpha(#ffffff, 0.92); }
.hero-text { font-size: 15px; color: alpha(#ffffff, 0.78); }
.hero-aside { font-size: 16px; color: alpha(#ffffff, 0.82); }
.hero button.hero-primary {
  min-height: 44px; padding: 0 30px; border-radius: 10px;
  background-color: @accent_bg_color; background-image: none;
  color: #04222b; font-size: 15px; font-weight: 600;
  border: none;
  box-shadow: 0 0 20px alpha(@accent_bg_color, 0.35), inset 0 1px 0 alpha(#ffffff, 0.35);
}
.hero button.hero-primary:hover { background-image: linear-gradient(alpha(#ffffff, 0.14), alpha(#ffffff, 0.14)); }
.hero button.hero-primary:disabled { opacity: 0.7; }
.hero button.hero-secondary {
  min-height: 44px; padding: 0 22px; border-radius: 10px;
  background-color: alpha(#0b1020, 0.35);
  color: #ffffff; font-size: 15px; font-weight: 500;
  border: 1px solid alpha(#ffffff, 0.22);
}
.hero button.hero-secondary:hover { background-color: alpha(#ffffff, 0.10); }
.hero-chip image { color: @accent_bg_color; -gtk-icon-size: 16px; }
.hero-chip label { font-size: 13px; color: alpha(#ffffff, 0.82); }
.hero-orb { min-width: 220px; min-height: 220px; border-radius: 999px; }
.hero-orb image { color: #ffffff; }
.orb-0 { background-image: radial-gradient(circle at 38% 32%, #ffd35c, #ff7a2f 40%, #c0267f 72%, #5b21b6); box-shadow: 0 0 70px alpha(#ff7a2f, 0.40); }
.orb-1 { background-image: radial-gradient(circle at 38% 32%, #7dd3fc, #0ea5e9 45%, #1d4ed8 80%); box-shadow: 0 0 70px alpha(#0ea5e9, 0.40); }
.orb-2 { background-image: radial-gradient(circle at 38% 32%, #f9a8d4, #db2777 45%, #6d28d9 85%); box-shadow: 0 0 70px alpha(#db2777, 0.38); }
.orb-3 { background-image: radial-gradient(circle at 38% 32%, #6ee7b7, #10b981 45%, #0f766e 85%); box-shadow: 0 0 70px alpha(#10b981, 0.38); }
carouselindicatorlines { color: @accent_bg_color; }

/* ── Browse by Category ──────────────────────────────────────────────── */
button.browse-tile {
  padding: 18px 6px 16px 6px;
  min-width: 100px;
  border-radius: 14px;
  background-color: alpha(#ffffff, 0.045);
  border: 1px solid alpha(#ffffff, 0.08);
  box-shadow: inset 0 1px 0 alpha(#ffffff, 0.05);
}
button.browse-tile:hover { background-color: alpha(#ffffff, 0.08); border-color: alpha(@accent_bg_color, 0.30); }
button.browse-tile .cat-title { font-size: 14px; font-weight: 500; }
button.browse-tile .cat-count { font-size: 12.5px; font-weight: 400; color: alpha(@window_fg_color, 0.55); }
button.browse-tile.shelf-productivity image { color: #4F9DFF; }
button.browse-tile.shelf-development image { color: #5AA8FF; }
button.browse-tile.shelf-games image { color: #A78BFA; }
button.browse-tile.shelf-media image { color: #F25C5C; }
button.browse-tile.shelf-internet image { color: @accent_bg_color; }
button.browse-tile.shelf-utilities image { color: #A5C0DD; }
button.browse-tile.shelf-customization image { color: #4ADE80; }

/* Category tile on the Categories page */
.category-tile { padding: 18px 16px; min-width: 170px; }
.category-tile:hover { background-color: alpha(#ffffff, 0.09); border-color: alpha(#ffffff, 0.14); }
.category-tile:active { background-color: alpha(#ffffff, 0.12); }
.category-tile image { color: @accent_bg_color; -gtk-icon-size: 28px; }
.category-tile .cat-title { font-weight: 600; font-size: 14px; letter-spacing: -0.1px; }

/* ── Side panel on Discover ──────────────────────────────────────────── */
.side-panel { min-width: 320px; }
.side-card { padding: 18px 16px; border-radius: 16px; }
.side-title { font-size: 20px; font-weight: 600; letter-spacing: -0.2px; }
.side-subtitle { font-size: 13px; color: alpha(@window_fg_color, 0.62); }
.side-card button.update-all {
  min-height: 40px; padding: 0 22px; border-radius: 10px;
  background-color: @accent_bg_color; background-image: none;
  color: #04222b; font-size: 15px; font-weight: 600;
  border: none;
  box-shadow: 0 0 18px alpha(@accent_bg_color, 0.30), inset 0 1px 0 alpha(#ffffff, 0.30);
}
.update-row { padding: 8px 2px; }
.update-row .version { font-size: 13px; letter-spacing: 0.1px; color: alpha(@window_fg_color, 0.65); }
.update-row .pkg-glyph { color: alpha(@window_fg_color, 0.85); }
button.round-download {
  min-width: 40px; min-height: 40px; padding: 0; border-radius: 999px;
  background-color: alpha(@accent_bg_color, 0.14);
  border: 1px solid alpha(@accent_bg_color, 0.40);
  color: @accent_bg_color;
  box-shadow: none;
}
button.round-download:hover { background-color: alpha(@accent_bg_color, 0.26); }
button.more-button {
  min-height: 42px; padding: 0 14px; border-radius: 10px;
  background-color: alpha(#ffffff, 0.03);
  border: 1px solid alpha(#ffffff, 0.12);
  box-shadow: none;
  font-size: 15px; font-weight: 400;
}
button.more-button:hover { background-color: alpha(#ffffff, 0.07); }
.pick-star { color: @accent_bg_color; -gtk-icon-size: 20px; }
button.pick-row { padding: 8px 6px; border-radius: 10px; }
.icon-well.pick-well { min-width: 44px; min-height: 44px; border-radius: 10px; }
.chevron { color: alpha(@window_fg_color, 0.55); -gtk-icon-size: 16px; }

/* ── Search results ──────────────────────────────────────────────────── */
.results-note { font-size: 13px; }

/* Transaction dialog */
.tx-stage { font-weight: 600; }
.tx-log {
  font-family: monospace; font-size: 11.5px;
  background-color: alpha(#000000, 0.30);
  border: 1px solid alpha(#ffffff, 0.08);
  border-radius: 10px; padding: 8px;
}
.tx-log text { background-color: transparent; }
"#
);

const LIGHT_CSS: &str = concat!(
    include_str!("../../data/raven-glass-light.css"),
    r#"
.tx-log { background-color: alpha(#000000, 0.06); border-color: alpha(#000000, 0.08); }
window.raven, window.raven.glass { background-image: none; }
.top-search { background-color: alpha(#ffffff, 0.80); border-color: alpha(#000000, 0.10); }
.header-motto { color: alpha(@window_fg_color, 0.40); }
.sidebar list.navigation-sidebar row:selected label,
.sidebar list.navigation-sidebar row:selected .nav-glyph,
.filter-tabs button:checked { color: @window_fg_color; }
.app-card { background-color: alpha(#ffffff, 0.72); border-color: alpha(#000000, 0.07); }
.app-card button.action, .app-card button.action.suggested-action { color: @window_fg_color; }
button.browse-tile, button.more-button { background-color: alpha(#ffffff, 0.72); border-color: alpha(#000000, 0.08); }
"#
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
