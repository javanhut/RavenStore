//! The curated catalogue behind Discover and Categories.
//!
//! rvn knows every package in the repositories and the AUR, but a store
//! front needs a human touch: which of the 90,000 packages are the apps
//! people actually look for, what to call them, and which icon to show.
//! That is this table. Everything else is reachable through search.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Category {
    pub id: &'static str,
    pub title: &'static str,
    pub icon: &'static str,
    pub blurb: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct Entry {
    /// The package name rvn installs.
    pub package: &'static str,
    /// Display name.
    pub title: &'static str,
    /// One or two words: "Web Browser".
    pub kind: &'static str,
    /// A sentence for the hero cards and detail view.
    pub tagline: &'static str,
    /// A symbolic icon that ships with the image, shown until the app is
    /// installed and its own icon can be used.
    pub icon: &'static str,
    pub category: &'static str,
}

pub const CATEGORIES: &[Category] = &[
    Category {
        id: "web",
        title: "Web",
        icon: "web-browser-symbolic",
        blurb: "Browsers and the open web",
    },
    Category {
        id: "productivity",
        title: "Productivity",
        icon: "x-office-document-symbolic",
        blurb: "Documents, notes, mail and planning",
    },
    Category {
        id: "development",
        title: "Developer Tools",
        icon: "utilities-terminal-symbolic",
        blurb: "Editors, languages, version control",
    },
    Category {
        id: "media",
        title: "Music & Video",
        icon: "multimedia-player-symbolic",
        blurb: "Players, streaming and recording",
    },
    Category {
        id: "graphics",
        title: "Graphics & Design",
        icon: "applications-graphics-symbolic",
        blurb: "Photo, vector, painting and 3D",
    },
    Category {
        id: "communication",
        title: "Communication",
        icon: "user-available-symbolic",
        blurb: "Chat, calls and video meetings",
    },
    Category {
        id: "games",
        title: "Games",
        icon: "applications-games-symbolic",
        blurb: "Launchers, stores and emulators",
    },
    Category {
        id: "system",
        title: "System & Utilities",
        icon: "preferences-system-symbolic",
        blurb: "Terminals, monitors, disks and backups",
    },
    Category {
        id: "customization",
        title: "Customization",
        icon: "preferences-desktop-appearance-symbolic",
        blurb: "Themes, icons and a prompt that looks the part",
    },
];

pub const ENTRIES: &[Entry] = &[
    // Customization
    Entry {
        package: "nwg-look",
        title: "nwg-look",
        kind: "GTK Settings",
        tagline: "Set GTK themes, icons, cursors and fonts on Wayland.",
        icon: "preferences-desktop-appearance-symbolic",
        category: "customization",
    },
    Entry {
        package: "qt6ct",
        title: "Qt6 Settings",
        kind: "Qt Settings",
        tagline: "Style, fonts and icons for Qt 6 apps.",
        icon: "preferences-desktop-appearance-symbolic",
        category: "customization",
    },
    Entry {
        package: "kvantum",
        title: "Kvantum",
        kind: "Qt Theme Engine",
        tagline: "SVG-based themes for Qt apps.",
        icon: "preferences-desktop-appearance-symbolic",
        category: "customization",
    },
    Entry {
        package: "papirus-icon-theme",
        title: "Papirus",
        kind: "Icon Theme",
        tagline: "A clean, complete icon theme.",
        icon: "image-x-generic-symbolic",
        category: "customization",
    },
    Entry {
        package: "starship",
        title: "Starship",
        kind: "Shell Prompt",
        tagline: "A fast, minimal prompt for any shell.",
        icon: "utilities-terminal-symbolic",
        category: "customization",
    },
    Entry {
        package: "gnome-tweaks",
        title: "Tweaks",
        kind: "Desktop Tweaks",
        tagline: "Fonts, themes and advanced desktop settings.",
        icon: "preferences-other-symbolic",
        category: "customization",
    },
    // Web
    Entry {
        package: "firefox",
        title: "Firefox",
        kind: "Web Browser",
        tagline: "A fast, private and independent web browser from Mozilla.",
        icon: "web-browser-symbolic",
        category: "web",
    },
    Entry {
        package: "chromium",
        title: "Chromium",
        kind: "Web Browser",
        tagline: "The open-source browser behind Chrome.",
        icon: "web-browser-symbolic",
        category: "web",
    },
    Entry {
        package: "brave-bin",
        title: "Brave",
        kind: "Web Browser",
        tagline: "A privacy-first browser with built-in ad blocking.",
        icon: "web-browser-symbolic",
        category: "web",
    },
    Entry {
        package: "vivaldi",
        title: "Vivaldi",
        kind: "Web Browser",
        tagline: "A browser you can shape around how you work.",
        icon: "web-browser-symbolic",
        category: "web",
    },
    Entry {
        package: "qutebrowser",
        title: "qutebrowser",
        kind: "Web Browser",
        tagline: "A keyboard-driven browser with Vim-style bindings.",
        icon: "web-browser-symbolic",
        category: "web",
    },
    Entry {
        package: "torbrowser-launcher",
        title: "Tor Browser",
        kind: "Web Browser",
        tagline: "Browse anonymously over the Tor network.",
        icon: "web-browser-symbolic",
        category: "web",
    },
    // Productivity
    Entry {
        package: "libreoffice-fresh",
        title: "LibreOffice",
        kind: "Office Suite",
        tagline: "Writer, Calc, Impress and more — the free office suite.",
        icon: "x-office-document-symbolic",
        category: "productivity",
    },
    Entry {
        package: "thunderbird",
        title: "Thunderbird",
        kind: "Email Client",
        tagline: "Email, calendar and contacts in one place.",
        icon: "mail-unread-symbolic",
        category: "productivity",
    },
    Entry {
        package: "obsidian",
        title: "Obsidian",
        kind: "Notes",
        tagline: "A second brain built on plain Markdown files.",
        icon: "accessories-text-editor-symbolic",
        category: "productivity",
    },
    Entry {
        package: "joplin-desktop",
        title: "Joplin",
        kind: "Notes",
        tagline: "Open-source notes and to-dos that sync anywhere.",
        icon: "accessories-text-editor-symbolic",
        category: "productivity",
    },
    Entry {
        package: "keepassxc",
        title: "KeePassXC",
        kind: "Password Manager",
        tagline: "Keep your passwords in an encrypted local vault.",
        icon: "dialog-password-symbolic",
        category: "productivity",
    },
    Entry {
        package: "bitwarden",
        title: "Bitwarden",
        kind: "Password Manager",
        tagline: "Open-source password management for everyone.",
        icon: "dialog-password-symbolic",
        category: "productivity",
    },
    Entry {
        package: "gnome-calculator",
        title: "Calculator",
        kind: "Calculator",
        tagline: "Basic, advanced, financial and programming modes.",
        icon: "accessories-calculator-symbolic",
        category: "productivity",
    },
    Entry {
        package: "evince",
        title: "Document Viewer",
        kind: "PDF Viewer",
        tagline: "Read PDFs, comics and more.",
        icon: "x-office-document-symbolic",
        category: "productivity",
    },
    Entry {
        package: "nextcloud-client",
        title: "Nextcloud",
        kind: "Cloud Storage",
        tagline: "Sync your files with your own cloud.",
        icon: "folder-remote-symbolic",
        category: "productivity",
    },
    Entry {
        package: "syncthing",
        title: "Syncthing",
        kind: "File Sync",
        tagline: "Continuous file synchronisation between your devices.",
        icon: "network-workgroup-symbolic",
        category: "productivity",
    },
    // Development
    Entry {
        package: "code",
        title: "VS Code",
        kind: "Code Editor",
        tagline: "The open-source build of Visual Studio Code.",
        icon: "text-editor-symbolic",
        category: "development",
    },
    Entry {
        package: "neovim",
        title: "Neovim",
        kind: "Text Editor",
        tagline: "Vim, modernised and endlessly extensible.",
        icon: "text-editor-symbolic",
        category: "development",
    },
    Entry {
        package: "helix",
        title: "Helix",
        kind: "Text Editor",
        tagline: "A post-modern modal editor with tree-sitter built in.",
        icon: "text-editor-symbolic",
        category: "development",
    },
    Entry {
        package: "zed",
        title: "Zed",
        kind: "Code Editor",
        tagline: "A high-performance, collaborative editor.",
        icon: "text-editor-symbolic",
        category: "development",
    },
    Entry {
        package: "git",
        title: "Git",
        kind: "Version Control",
        tagline: "The distributed version control system.",
        icon: "document-open-recent-symbolic",
        category: "development",
    },
    Entry {
        package: "github-cli",
        title: "GitHub CLI",
        kind: "Git Client",
        tagline: "GitHub from the command line.",
        icon: "document-open-recent-symbolic",
        category: "development",
    },
    Entry {
        package: "rustup",
        title: "Rust",
        kind: "Language",
        tagline: "The Rust toolchain installer.",
        icon: "system-run-symbolic",
        category: "development",
    },
    Entry {
        package: "go",
        title: "Go",
        kind: "Language",
        tagline: "The Go programming language.",
        icon: "system-run-symbolic",
        category: "development",
    },
    Entry {
        package: "python",
        title: "Python",
        kind: "Language",
        tagline: "The Python programming language.",
        icon: "system-run-symbolic",
        category: "development",
    },
    Entry {
        package: "nodejs",
        title: "Node.js",
        kind: "Runtime",
        tagline: "JavaScript outside the browser.",
        icon: "system-run-symbolic",
        category: "development",
    },
    Entry {
        package: "docker",
        title: "Docker",
        kind: "Containers",
        tagline: "Build, ship and run containers.",
        icon: "package-x-generic-symbolic",
        category: "development",
    },
    Entry {
        package: "dbeaver",
        title: "DBeaver",
        kind: "Database GUI",
        tagline: "A universal database tool.",
        icon: "drive-harddisk-symbolic",
        category: "development",
    },
    Entry {
        package: "wireshark-qt",
        title: "Wireshark",
        kind: "Network Tool",
        tagline: "See what is really on the wire.",
        icon: "network-wired-symbolic",
        category: "development",
    },
    Entry {
        package: "postman-bin",
        title: "Postman",
        kind: "API Tool",
        tagline: "Design, test and document APIs.",
        icon: "send-to-symbolic",
        category: "development",
    },
    // Media
    Entry {
        package: "vlc",
        title: "VLC",
        kind: "Media Player",
        tagline: "Plays everything, everywhere.",
        icon: "multimedia-player-symbolic",
        category: "media",
    },
    Entry {
        package: "mpv",
        title: "mpv",
        kind: "Media Player",
        tagline: "A minimal, scriptable video player.",
        icon: "multimedia-player-symbolic",
        category: "media",
    },
    Entry {
        package: "spotify-launcher",
        title: "Spotify",
        kind: "Music",
        tagline: "Stream millions of songs and podcasts.",
        icon: "audio-x-generic-symbolic",
        category: "media",
    },
    Entry {
        package: "obs-studio",
        title: "OBS Studio",
        kind: "Streaming",
        tagline: "Record and live-stream with a pro-grade studio.",
        icon: "media-record-symbolic",
        category: "media",
    },
    Entry {
        package: "audacity",
        title: "Audacity",
        kind: "Audio Editor",
        tagline: "Record and edit audio.",
        icon: "audio-input-microphone-symbolic",
        category: "media",
    },
    Entry {
        package: "kdenlive",
        title: "Kdenlive",
        kind: "Video Editor",
        tagline: "Non-linear video editing, free and open.",
        icon: "video-x-generic-symbolic",
        category: "media",
    },
    Entry {
        package: "handbrake",
        title: "HandBrake",
        kind: "Video Converter",
        tagline: "Convert video from nearly any format.",
        icon: "camera-video-symbolic",
        category: "media",
    },
    Entry {
        package: "strawberry",
        title: "Strawberry",
        kind: "Music Player",
        tagline: "A music player and collection organiser.",
        icon: "folder-music-symbolic",
        category: "media",
    },
    // Graphics
    Entry {
        package: "gimp",
        title: "GIMP",
        kind: "Image Editor",
        tagline: "The GNU image manipulation program.",
        icon: "image-x-generic-symbolic",
        category: "graphics",
    },
    Entry {
        package: "inkscape",
        title: "Inkscape",
        kind: "Vector Graphics",
        tagline: "Professional vector drawing.",
        icon: "x-office-drawing-symbolic",
        category: "graphics",
    },
    Entry {
        package: "krita",
        title: "Krita",
        kind: "Digital Painting",
        tagline: "Painting and illustration for artists.",
        icon: "applications-graphics-symbolic",
        category: "graphics",
    },
    Entry {
        package: "blender",
        title: "Blender",
        kind: "3D Suite",
        tagline: "Modelling, animation, rendering and more.",
        icon: "applications-engineering-symbolic",
        category: "graphics",
    },
    Entry {
        package: "darktable",
        title: "darktable",
        kind: "Photo Workflow",
        tagline: "Raw developing and photo management.",
        icon: "camera-photo-symbolic",
        category: "graphics",
    },
    Entry {
        package: "shotwell",
        title: "Shotwell",
        kind: "Photo Manager",
        tagline: "Organise and lightly edit your photos.",
        icon: "folder-pictures-symbolic",
        category: "graphics",
    },
    Entry {
        package: "figma-linux",
        title: "Figma",
        kind: "UI Design",
        tagline: "The collaborative design tool, on Linux.",
        icon: "x-office-drawing-symbolic",
        category: "graphics",
    },
    // Communication
    Entry {
        package: "discord",
        title: "Discord",
        kind: "Chat",
        tagline: "Voice, video and text for your communities.",
        icon: "chat-message-new-symbolic",
        category: "communication",
    },
    Entry {
        package: "telegram-desktop",
        title: "Telegram",
        kind: "Messaging",
        tagline: "Fast, secure messaging.",
        icon: "chat-message-new-symbolic",
        category: "communication",
    },
    Entry {
        package: "signal-desktop",
        title: "Signal",
        kind: "Messaging",
        tagline: "Private messaging, end to end.",
        icon: "chat-message-new-symbolic",
        category: "communication",
    },
    Entry {
        package: "slack-desktop",
        title: "Slack",
        kind: "Team Chat",
        tagline: "Where work happens.",
        icon: "chat-message-new-symbolic",
        category: "communication",
    },
    Entry {
        package: "zoom",
        title: "Zoom",
        kind: "Video Calls",
        tagline: "Meetings and webinars.",
        icon: "camera-web-symbolic",
        category: "communication",
    },
    Entry {
        package: "element-desktop",
        title: "Element",
        kind: "Matrix Client",
        tagline: "Decentralised, encrypted chat on Matrix.",
        icon: "chat-message-new-symbolic",
        category: "communication",
    },
    // Games
    Entry {
        package: "steam",
        title: "Steam",
        kind: "Game Store",
        tagline: "Thousands of games, with Proton for Windows titles.",
        icon: "input-gaming-symbolic",
        category: "games",
    },
    Entry {
        package: "lutris",
        title: "Lutris",
        kind: "Game Launcher",
        tagline: "One launcher for every game and store.",
        icon: "input-gaming-symbolic",
        category: "games",
    },
    Entry {
        package: "heroic-games-launcher-bin",
        title: "Heroic",
        kind: "Game Launcher",
        tagline: "Epic, GOG and Amazon games on Linux.",
        icon: "input-gaming-symbolic",
        category: "games",
    },
    Entry {
        package: "prismlauncher",
        title: "Prism Launcher",
        kind: "Minecraft",
        tagline: "Manage Minecraft instances and mods.",
        icon: "input-gaming-symbolic",
        category: "games",
    },
    Entry {
        package: "retroarch",
        title: "RetroArch",
        kind: "Emulation",
        tagline: "A front-end for emulators and game engines.",
        icon: "applications-games-symbolic",
        category: "games",
    },
    Entry {
        package: "minecraft-launcher",
        title: "Minecraft",
        kind: "Game",
        tagline: "The official Minecraft launcher.",
        icon: "input-gaming-symbolic",
        category: "games",
    },
    // System
    Entry {
        package: "alacritty",
        title: "Alacritty",
        kind: "Terminal",
        tagline: "A GPU-accelerated terminal emulator.",
        icon: "utilities-terminal-symbolic",
        category: "system",
    },
    Entry {
        package: "kitty",
        title: "kitty",
        kind: "Terminal",
        tagline: "A fast, feature-rich GPU terminal.",
        icon: "utilities-terminal-symbolic",
        category: "system",
    },
    Entry {
        package: "btop",
        title: "btop",
        kind: "System Monitor",
        tagline: "Resource monitor with a beautiful terminal UI.",
        icon: "computer-symbolic",
        category: "system",
    },
    Entry {
        package: "htop",
        title: "htop",
        kind: "Process Viewer",
        tagline: "An interactive process viewer.",
        icon: "computer-symbolic",
        category: "system",
    },
    Entry {
        package: "fastfetch",
        title: "fastfetch",
        kind: "System Info",
        tagline: "Show off your system at a glance.",
        icon: "dialog-information-symbolic",
        category: "system",
    },
    Entry {
        package: "gnome-disk-utility",
        title: "Disks",
        kind: "Disk Utility",
        tagline: "Manage drives, partitions and images.",
        icon: "drive-harddisk-symbolic",
        category: "system",
    },
    Entry {
        package: "gparted",
        title: "GParted",
        kind: "Partition Editor",
        tagline: "Resize, create and manage partitions.",
        icon: "drive-harddisk-symbolic",
        category: "system",
    },
    Entry {
        package: "timeshift",
        title: "Timeshift",
        kind: "Backups",
        tagline: "System snapshots you can roll back to.",
        icon: "drive-multidisk-symbolic",
        category: "system",
    },
    Entry {
        package: "tmux",
        title: "tmux",
        kind: "Terminal Multiplexer",
        tagline: "Persistent terminal sessions and panes.",
        icon: "utilities-terminal-symbolic",
        category: "system",
    },
    Entry {
        package: "flatpak",
        title: "Flatpak",
        kind: "App Framework",
        tagline: "Sandboxed desktop apps from Flathub.",
        icon: "system-software-install-symbolic",
        category: "system",
    },
    Entry {
        package: "ripgrep",
        title: "ripgrep",
        kind: "Search Tool",
        tagline: "grep, but blazingly fast and gitignore-aware.",
        icon: "system-search-symbolic",
        category: "system",
    },
    Entry {
        package: "flatseal",
        title: "Flatseal",
        kind: "Permissions",
        tagline: "Manage what your Flatpak apps can reach.",
        icon: "security-high-symbolic",
        category: "system",
    },
];

/// A hero card on Discover: the words around a featured package.
#[derive(Debug, Clone, Copy)]
pub struct Feature {
    pub package: &'static str,
    /// Under the title: "Private. Fast. Open."
    pub headline: &'static str,
    pub blurb: &'static str,
    /// The line beside the artwork.
    pub aside: &'static str,
    /// (icon, label) pairs along the foot of the card.
    pub chips: &'static [(&'static str, &'static str)],
}

const WORKS_ON_RAVEN: (&str, &str) = ("view-grid-symbolic", "Works Great on Raven");
const OPEN_SOURCE: (&str, &str) = ("text-editor-symbolic", "Open Source");

/// The hero carousel on Discover, in order.
pub const FEATURED: &[Feature] = &[
    Feature {
        package: "firefox",
        headline: "Private. Fast. Open.",
        blurb: "A secure, independent web browser from Mozilla. Built for people, not profit.",
        aside: "A more open internet for a brighter tomorrow.",
        chips: &[
            OPEN_SOURCE,
            ("security-high-symbolic", "Privacy Focused"),
            WORKS_ON_RAVEN,
        ],
    },
    Feature {
        package: "code",
        headline: "Write. Build. Ship.",
        blurb: "The open-source build of Visual Studio Code, ready for every language you use.",
        aside: "Your editor, on a system you control.",
        chips: &[
            OPEN_SOURCE,
            ("utilities-terminal-symbolic", "Every Language"),
            WORKS_ON_RAVEN,
        ],
    },
    Feature {
        package: "obs-studio",
        headline: "Record. Stream. Create.",
        blurb: "Pro-grade recording and live streaming, free for everyone.",
        aside: "Share what you make with the world.",
        chips: &[
            OPEN_SOURCE,
            ("media-record-symbolic", "Record & Stream"),
            WORKS_ON_RAVEN,
        ],
    },
    Feature {
        package: "steam",
        headline: "Play Without Limits.",
        blurb: "Thousands of games, with Proton to run Windows titles on Linux.",
        aside: "Your library, on a system you own.",
        chips: &[
            ("input-gaming-symbolic", "Proton Built In"),
            ("network-workgroup-symbolic", "Play With Friends"),
            WORKS_ON_RAVEN,
        ],
    },
];

/// A group of categories as Discover presents them: the filter tabs over
/// Popular Apps (the first six) and the Browse by Category tiles.
#[derive(Debug, Clone, Copy)]
pub struct Shelf {
    pub id: &'static str,
    pub title: &'static str,
    pub blurb: &'static str,
    pub icon: &'static str,
    pub categories: &'static [&'static str],
}

pub const SHELVES: &[Shelf] = &[
    Shelf {
        id: "productivity",
        title: "Productivity",
        blurb: "Documents, notes, mail and planning",
        icon: "view-grid-symbolic",
        categories: &["productivity"],
    },
    Shelf {
        id: "development",
        title: "Development",
        blurb: "Editors, languages, version control",
        icon: "utilities-terminal-symbolic",
        categories: &["development"],
    },
    Shelf {
        id: "media",
        title: "Media",
        blurb: "Players, editors, painting and 3D",
        icon: "multimedia-player-symbolic",
        categories: &["media", "graphics"],
    },
    Shelf {
        id: "games",
        title: "Games",
        blurb: "Launchers, stores and emulators",
        icon: "input-gaming-symbolic",
        categories: &["games"],
    },
    Shelf {
        id: "internet",
        title: "Internet",
        blurb: "Browsers, chat and video calls",
        icon: "web-browser-symbolic",
        categories: &["web", "communication"],
    },
    Shelf {
        id: "utilities",
        title: "Utilities",
        blurb: "Terminals, monitors, disks and backups",
        icon: "applications-utilities-symbolic",
        categories: &["system"],
    },
    Shelf {
        id: "customization",
        title: "Customization",
        blurb: "Themes, icons and a prompt that looks the part",
        icon: "preferences-desktop-appearance-symbolic",
        categories: &["customization"],
    },
];

/// The "Popular Apps" strip on Discover.
pub const POPULAR: &[&str] = &[
    "firefox",
    "code",
    "vlc",
    "discord",
    "gimp",
    "steam",
    "spotify-launcher",
    "obsidian",
];

/// The "Raven Picks" list in Discover's side panel.
pub const PICKS: &[&str] = &["neovim", "alacritty", "keepassxc", "obsidian", "flatseal"];

pub fn category(id: &str) -> Option<&'static Category> {
    CATEGORIES.iter().find(|c| c.id == id)
}

pub fn entry(package: &str) -> Option<&'static Entry> {
    ENTRIES.iter().find(|e| e.package == package)
}

pub fn in_category(id: &str) -> Vec<&'static Entry> {
    ENTRIES.iter().filter(|e| e.category == id).collect()
}

pub fn in_categories(ids: &[&str]) -> Vec<&'static Entry> {
    ENTRIES
        .iter()
        .filter(|e| ids.contains(&e.category))
        .collect()
}

/// Curated entries whose title, package or kind matches `query`.
pub fn matching(query: &str) -> Vec<&'static Entry> {
    let q = query.to_lowercase();
    ENTRIES
        .iter()
        .filter(|e| {
            e.title.to_lowercase().contains(&q)
                || e.package.contains(&q)
                || e.kind.to_lowercase().contains(&q)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_entry_has_a_known_category() {
        for e in ENTRIES {
            assert!(
                category(e.category).is_some(),
                "{} has unknown category {}",
                e.package,
                e.category
            );
        }
    }

    #[test]
    fn featured_popular_and_picks_are_catalogued() {
        for f in FEATURED {
            assert!(
                entry(f.package).is_some(),
                "{} is featured but not in ENTRIES",
                f.package
            );
        }
        for p in POPULAR.iter().chain(PICKS.iter()) {
            assert!(entry(p).is_some(), "{p} is listed but not in ENTRIES");
        }
    }

    #[test]
    fn packages_are_unique() {
        let mut names: Vec<&str> = ENTRIES.iter().map(|e| e.package).collect();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), ENTRIES.len());
    }
}
