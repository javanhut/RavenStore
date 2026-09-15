//! Ordering search results so the thing people are looking for comes first.
//!
//! A search for "firefox" returns the browser alongside dozens of add-ons
//! and language packs, and rvn does not tell them apart. Installing
//! `firefox-adblock-plus` when you meant Firefox is an easy mistake, so the
//! results are split in two: applications, then packages — libraries,
//! plugins, language packs and the other parts that are usually pulled in
//! as dependencies. Each half is ordered best first.
//!
//! rvn reports no star ratings. The signals it does give are used instead:
//! an exact name match, a place in the curated catalogue, the official
//! repositories over the AUR, and the AUR's own popularity score.

use std::cmp::Ordering;

use crate::backend::rvn::Package;
use crate::catalog;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    App,
    Package,
}

#[derive(Debug, Default)]
pub struct Ranked {
    pub apps: Vec<Package>,
    pub packages: Vec<Package>,
}

/// Name prefixes of language bindings, fonts, plugins and other components.
const COMPONENT_PREFIXES: &[&str] = &[
    "lib32-",
    "python-",
    "python2-",
    "perl-",
    "ruby-",
    "lua-",
    "lua51-",
    "lua52-",
    "lua53-",
    "haskell-",
    "nodejs-",
    "node-",
    "go-",
    "rust-",
    "r-",
    "php-",
    "ocaml-",
    "java-",
    "texlive-",
    "ttf-",
    "otf-",
    "woff-",
    "woff2-",
    "noto-fonts",
    "gst-",
    "gstreamer",
    "qt5-",
    "qt6-",
    "kf5-",
    "kf6-",
    "vim-",
    "emacs-",
    "gnome-shell-extension-",
    "xf86-",
    "mingw-w64-",
    "hunspell-",
    "aspell-",
    "mythes-",
    "hyphen-",
];

const COMPONENT_SUFFIXES: &[&str] = &[
    "-doc",
    "-docs",
    "-headers",
    "-devel",
    "-dev",
    "-debug",
    "-dbg",
    "-data",
    "-common",
    "-plugin",
    "-plugins",
    "-extension",
    "-extensions",
    "-theme",
    "-themes",
    "-locale",
    "-dictionary",
];

const COMPONENT_INFIXES: &[&str] = &[
    "-i18n",
    "-l10n",
    "-langpack",
    "-lang-",
    "-plugin-",
    "-addon",
];

/// Whether a package is something to open, or a part of something else.
pub fn kind(p: &Package) -> Kind {
    if catalog::entry(&p.name).is_some() {
        return Kind::App;
    }
    let name = p.name.to_lowercase();
    // `lib…` is a library, but LibreOffice and LibreWolf are not.
    let library = name.starts_with("lib") && !name.starts_with("libre");
    let component = library
        || COMPONENT_PREFIXES.iter().any(|s| name.starts_with(s))
        || COMPONENT_SUFFIXES.iter().any(|s| name.ends_with(s))
        || COMPONENT_INFIXES.iter().any(|s| name.contains(s))
        || p.groups
            .iter()
            .any(|g| g.ends_with("-addons") || g.ends_with("-plugins"));
    if component || extends_a_dependency(&name, &p.depends) {
        Kind::Package
    } else {
        Kind::App
    }
}

/// `firefox-adblock-plus` depends on `firefox`: an add-on for it, not an
/// app of its own.
fn extends_a_dependency(name: &str, depends: &[String]) -> bool {
    depends.iter().any(|d| {
        let dep = d
            .split(['<', '>', '='])
            .next()
            .unwrap_or_default()
            .to_lowercase();
        !dep.is_empty() && name.len() > dep.len() && name.starts_with(&format!("{dep}-"))
    })
}

/// Splits `results` into apps and packages, each ordered best first.
/// Curated apps matching the query that the search did not return are
/// added, so "vs code" still finds `code`.
pub fn arrange(query: &str, results: &[Package]) -> Ranked {
    let q = query.trim().to_lowercase();
    let mut all = results.to_vec();
    for e in catalog::matching(&q) {
        if !all.iter().any(|p| p.name == e.package) {
            all.push(Package {
                name: e.package.into(),
                description: e.tagline.into(),
                ..Package::default()
            });
        }
    }
    let (mut apps, mut packages): (Vec<Package>, Vec<Package>) =
        all.into_iter().partition(|p| kind(p) == Kind::App);
    apps.sort_by(|a, b| compare(a, b, &q));
    packages.sort_by(|a, b| compare(a, b, &q));
    Ranked { apps, packages }
}

/// Best first: exact match, curated, name starts with the query, official
/// over AUR, AUR popularity, then the shorter name. The sort is stable, so
/// rvn's own order breaks what is left.
fn compare(a: &Package, b: &Package, q: &str) -> Ordering {
    let (ka, kb) = (key(a, q), key(b, q));
    kb.exact
        .cmp(&ka.exact)
        .then(kb.curated.cmp(&ka.curated))
        .then(kb.prefix.cmp(&ka.prefix))
        .then(kb.official.cmp(&ka.official))
        .then(kb.popularity.total_cmp(&ka.popularity))
        .then(ka.name_len.cmp(&kb.name_len))
}

struct Key {
    exact: bool,
    curated: bool,
    prefix: bool,
    official: bool,
    popularity: f64,
    name_len: usize,
}

fn key(p: &Package, q: &str) -> Key {
    let name = p.name.to_lowercase();
    let title = catalog::entry(&p.name).map(|e| e.title.to_lowercase());
    Key {
        exact: name == q || title.as_deref() == Some(q),
        curated: title.is_some(),
        prefix: name.starts_with(q) || title.is_some_and(|t| t.starts_with(q)),
        official: !p.aur,
        popularity: p.popularity,
        name_len: name.len(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pkg(name: &str) -> Package {
        Package {
            name: name.into(),
            origin: "extra".into(),
            ..Package::default()
        }
    }

    fn aur(name: &str, popularity: f64) -> Package {
        Package {
            name: name.into(),
            origin: "aur".into(),
            aur: true,
            popularity,
            ..Package::default()
        }
    }

    fn names(list: &[Package]) -> Vec<&str> {
        list.iter().map(|p| p.name.as_str()).collect()
    }

    #[test]
    fn the_browser_comes_before_its_add_ons() {
        let results = vec![
            Package {
                depends: vec!["firefox".into()],
                groups: vec!["firefox-addons".into()],
                ..pkg("firefox-adblock-plus")
            },
            Package {
                depends: vec!["firefox-developer-edition>=156.0b5".into()],
                ..pkg("firefox-developer-edition-i18n-ach")
            },
            pkg("firefox-developer-edition"),
            pkg("firefox"),
        ];
        let r = arrange("firefox", &results);
        assert_eq!(names(&r.apps), ["firefox", "firefox-developer-edition"]);
        assert_eq!(
            names(&r.packages),
            ["firefox-adblock-plus", "firefox-developer-edition-i18n-ach"]
        );
    }

    #[test]
    fn exact_official_match_leads_then_aur_by_popularity() {
        let results = vec![
            aur("discord-chat-exporter-gui-bin", 0.87),
            aur("discord_arch_electron", 1.58),
            pkg("discord"),
        ];
        let r = arrange("discord", &results);
        assert_eq!(
            names(&r.apps),
            [
                "discord",
                "discord_arch_electron",
                "discord-chat-exporter-gui-bin"
            ]
        );
    }

    #[test]
    fn libraries_and_bindings_are_packages() {
        for name in [
            "libvlc",
            "python-requests",
            "ttf-fira-code",
            "vlc-plugin-ffmpeg",
        ] {
            assert_eq!(kind(&pkg(name)), Kind::Package, "{name}");
        }
        for name in ["libreoffice-fresh", "librewolf", "vlc", "obs-studio"] {
            assert_eq!(kind(&pkg(name)), Kind::App, "{name}");
        }
    }

    #[test]
    fn curated_apps_the_search_missed_are_added() {
        let r = arrange("VS Code", &[]);
        assert_eq!(names(&r.apps), ["code"]);
    }
}
