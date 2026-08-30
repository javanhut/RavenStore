//! Desktop entries: which installed package is a launchable app, and the
//! icon and command that go with it.

use std::collections::HashMap;

use gio::prelude::*;

#[derive(Debug, Clone)]
pub struct LaunchableApp {
    pub desktop_id: String,
    /// An icon-theme name or an absolute path, as the desktop entry gives
    /// it. Kept as text so the index can be built off the main thread.
    pub icon: Option<String>,
}

/// Index every desktop entry by the executable it runs (basename), so a
/// package can be matched to an app without a database in between.
pub struct AppIndex {
    by_exec: HashMap<String, LaunchableApp>,
    by_id: HashMap<String, LaunchableApp>,
}

impl AppIndex {
    pub fn scan() -> AppIndex {
        let mut by_exec = HashMap::new();
        let mut by_id = HashMap::new();
        for info in gio::AppInfo::all() {
            if !info.should_show() {
                continue;
            }
            let Some(id) = info.id() else { continue };
            let app = LaunchableApp {
                desktop_id: id.to_string(),
                icon: info
                    .icon()
                    .and_then(|i| i.to_string())
                    .map(|s| s.to_string()),
            };
            let stem = id.to_string().trim_end_matches(".desktop").to_lowercase();
            by_id.insert(stem, app.clone());
            if let Some(exe) = info
                .executable()
                .file_name()
                .map(|f| f.to_string_lossy().to_lowercase())
            {
                by_exec.entry(exe).or_insert(app);
            }
        }
        AppIndex { by_exec, by_id }
    }

    /// The app a package most likely provides. Tries the package name as a
    /// command, then as a desktop id suffix (`org.gnome.Calculator` for
    /// `gnome-calculator` does not match — that is what the catalogue's icon
    /// hint is for).
    pub fn for_package(&self, package: &str) -> Option<&LaunchableApp> {
        let key = package.to_lowercase();
        if let Some(app) = self.by_exec.get(&key) {
            return Some(app);
        }
        for suffix in ["-bin", "-git", "-desktop", "-launcher", "-qt", "-gtk"] {
            if let Some(stem) = key.strip_suffix(suffix) {
                if let Some(app) = self.by_exec.get(stem) {
                    return Some(app);
                }
            }
        }
        self.by_id
            .iter()
            .find(|(id, _)| id.rsplit('.').next() == Some(key.as_str()))
            .map(|(_, app)| app)
    }
}

pub fn launch(desktop_id: &str) -> anyhow::Result<()> {
    let info = gio::DesktopAppInfo::new(desktop_id)
        .ok_or_else(|| anyhow::anyhow!("{desktop_id} is no longer installed"))?;
    info.launch(&[], None::<&gio::AppLaunchContext>)?;
    Ok(())
}
