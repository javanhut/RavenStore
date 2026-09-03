//! Two files. `~/.config/raven/desktop.toml` is owned by Settings and read
//! here only for the look (theme, accent, transparency) so the store matches
//! the rest of the desktop. `~/.config/raven/store.toml` is the store's own:
//! wishlist and a few preferences.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

pub const DEFAULT_ACCENT: &str = "#7AA2F7";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ThemeMode {
    Light,
    #[default]
    Dark,
    Auto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Appearance {
    pub theme_mode: ThemeMode,
    pub accent: String,
    pub transparency: bool,
}

impl Default for Appearance {
    fn default() -> Self {
        Self {
            theme_mode: ThemeMode::Dark,
            accent: DEFAULT_ACCENT.into(),
            transparency: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct General {
    pub terminal: String,
}

/// The slice of desktop.toml the store cares about. Unknown keys are
/// ignored so Settings can grow without breaking us.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Desktop {
    pub appearance: Appearance,
    pub general: General,
}

impl Desktop {
    pub fn load() -> Desktop {
        let path = config_dir().join("desktop.toml");
        std::fs::read_to_string(&path)
            .ok()
            .and_then(|t| toml::from_str(&t).ok())
            .unwrap_or_default()
    }

    pub fn terminal(&self) -> String {
        if self.general.terminal.is_empty() {
            "raven-terminal".into()
        } else {
            self.general.terminal.clone()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct StoreConfig {
    /// Skip the AUR entirely: only official repositories are searched or
    /// installed from.
    pub repo_only: bool,
    /// Refresh the repository databases when the store opens. Runs as the
    /// user, so no password, but it hits the mirrors at every launch and the
    /// databases are refreshed before any install anyway; off by default.
    pub refresh_on_start: bool,
    /// Show packages installed as dependencies on the Installed page.
    pub show_dependencies: bool,
    /// Package names the user has hearted.
    pub wishlist: Vec<String>,
}

impl StoreConfig {
    pub fn path() -> PathBuf {
        config_dir().join("store.toml")
    }

    pub fn load() -> StoreConfig {
        std::fs::read_to_string(Self::path())
            .ok()
            .and_then(|t| toml::from_str(&t).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let path = Self::path();
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let text = format!(
            "# Raven Store preferences. Written by raven-store.\n{}",
            toml::to_string_pretty(self)?
        );
        std::fs::write(&path, text)?;
        Ok(())
    }

    pub fn wishes(&self, name: &str) -> bool {
        self.wishlist.iter().any(|w| w == name)
    }
}

fn config_dir() -> PathBuf {
    std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))
        .unwrap_or_else(|| PathBuf::from("."))
        .join("raven")
}
