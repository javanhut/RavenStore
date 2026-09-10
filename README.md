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
| Changing the system | `rvn --json -y install/uninstall/update …`, run as you; rvn hands the work to `rvnd`, the root daemon on `/run/rvn/ctl`, and relays its events, which stream live into a progress dialog |
| Your password | Never asked. `rvnd`'s socket is open to the `wheel` group, so being an administrator is the credential |
| AUR builds | rvnd builds with `makepkg` as the account that asked, exactly as from a terminal; the build log streams into the dialog's **Details** |
| Look and feel | GTK 4 + libadwaita, the same shell as Raven Settings; theme, accent and transparency are read from `~/.config/raven/desktop.toml` |

There is no polkit agent on Raven Linux and the store never runs `sudo`. If
`rvnd` is not running, or your account is outside the `wheel` group, the store
says so — with the fix — instead of attempting the change.

## Pages

- **Discover** — featured apps, popular picks, a couple of categories and the
  pending updates at a glance.
- **Categories** — the curated catalogue (`src/catalog.rs`), grouped.
- **Installed** — explicitly installed packages with Open and Remove; the
  Settings page can include dependencies too.
- **Updates** — what is out of date, with *Update All* or per-package updates.
  *Check for updates* refreshes the repository databases, no password: through
  `rvnd` when it is running, and otherwise as you, in which case rvn syncs a
  per-user copy instead of the system one. Every check — here or in Raven
  Settings — reads whichever copy is fresher, so a check in one app is a check
  in both. The window also watches the database
  directories and reloads when Settings, a terminal, or rvn changes them.
- **Wishlist** — packages you hearted, kept in `~/.config/raven/store.toml`.
- **Settings** — official-repositories-only mode, refresh-on-launch, and buttons
  to run the same operations in a terminal instead.
- **Search** (Ctrl+K) — the repositories and the AUR, with curated matches first.

Icons: an installed app shows its own icon from its desktop entry. Until then a
symbolic glyph tinted by category stands in, because the image does not ship
third-party brand icons.

## Requirements

- `rvn` with `--json` support and `rvnd` running (RavenPackageManager `main`
  from 2026-09-09 on; raven-init starts the daemon)
- GTK 4.12+, libadwaita 1.5+
- membership of the `wheel` group, to install, remove or update

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
build directory first on `PATH`; the store resolves `rvn` from `PATH`. `RVN_SOCKET`
points both rvn and the store's daemon check at another socket, for running
against a development `rvnd`.
