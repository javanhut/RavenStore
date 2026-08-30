# Raven Store

A graphical front-end for [`rvn`](https://github.com/javanhut/RavenPackageManager),
the Raven Linux package manager. Browse a curated catalogue, search the official
repositories and the AUR, install and remove packages, and keep the system up to
date — without a terminal.

The store is optional. Everything it does is `rvn` underneath: it runs
`rvn --json …` as a child process and renders the event stream. The terminal and
the store see the same stages, progress and messages; nothing is reimplemented.

## How it works

| Concern | How the store handles it |
| --- | --- |
| Reading the system | `rvn --json list`, `find`, `info`, `update --dry-run --no-refresh` run as the user and are parsed from stdout |
| Changing the system | `sudo rvn --json -y install/uninstall/update …` with events streamed live into a progress dialog |
| Your password | Asked once in a dialog and handed to `sudo -S`; never stored. If sudo's timestamp is still valid nothing is asked |
| AUR builds | rvn drops privileges to your account for `makepkg` (via `SUDO_USER`), exactly as from a terminal; the build log streams into the dialog's **Details** |
| Look and feel | GTK 4 + libadwaita, the same shell as Raven Settings; theme, accent and transparency are read from `~/.config/raven/desktop.toml` |

There is no polkit agent on Raven Linux, so the store speaks to `sudo` directly.
An account outside the `wheel` group gets a clear message rather than a hang.

## Pages

- **Discover** — featured apps, popular picks, a couple of categories and the
  pending updates at a glance.
- **Categories** — the curated catalogue (`src/catalog.rs`), grouped.
- **Installed** — explicitly installed packages with Open and Remove; the
  Settings page can include dependencies too.
- **Updates** — what is out of date, with *Update All* or per-package updates.
  *Check for updates* refreshes the repository databases (needs your password).
- **Wishlist** — packages you hearted, kept in `~/.config/raven/store.toml`.
- **Settings** — official-repositories-only mode, refresh-on-launch, and buttons
  to run the same operations in a terminal instead.
- **Search** (Ctrl+K) — the repositories and the AUR, with curated matches first.

Icons: an installed app shows its own icon from its desktop entry. Until then a
symbolic glyph tinted by category stands in, because the image does not ship
third-party brand icons.

## Requirements

- `rvn` with `--json` support (RavenPackageManager `main` from 2026-08-30 on)
- GTK 4.12+, libadwaita 1.5+
- `sudo`

## Building

```bash
cargo build --release          # target/release/raven-store
sudo make install              # /usr/local/bin + desktop entry + icon
imlazy install                 # the same, via ImLazy
```

```
raven-store              # opens on Discover
raven-store --updates    # opens on Updates (what Raven Settings launches)
```

## Development

```bash
make check                                   # fmt, clippy -D warnings, tests
RAVEN_STORE_SNAPSHOT=/tmp/shots cargo run    # render every page to PNG and quit
RAVEN_STORE_SNAPSHOT_QUERY=ripgrep …         # …including a search results page
RAVEN_STORE_SNAPSHOT_TX=1 …                  # …with a refresh running
RUST_LOG=debug cargo run                     # tracing output
```

To try the store against a freshly built rvn without installing it, put its
build directory first on `PATH`; the store resolves `rvn` from `PATH` and passes
the absolute path to sudo.
