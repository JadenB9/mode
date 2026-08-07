# mode

```
 ███╗   ███╗ ██████╗ ██████╗ ███████╗
 ████╗ ████║██╔═══██╗██╔══██╗██╔════╝
 ██╔████╔██║██║   ██║██║  ██║█████╗
 ██║╚██╔╝██║██║   ██║██║  ██║██╔══╝
 ██║ ╚═╝ ██║╚██████╔╝██████╔╝███████╗
 ╚═╝     ╚═╝ ╚═════╝ ╚═════╝ ╚══════╝
```

A terminal utility manager with a keyboard-driven TUI, written in Rust with
[ratatui](https://github.com/ratatui-org/ratatui). This is the Linux/WSL
version — the Python macOS one is [mode-terminal](https://github.com/JadenB9/mode-terminal).

You run `mode`, arrow-key to a tool, and it handles the fiddly shell stuff for
you. Anything that edits a file (your shell rc, for instance) makes a
timestamped backup first.

## What actually works

- **Alias Manager** — create shell aliases with name validation and duplicate
  detection. Writes to `~/.bashrc` or `~/.zshrc` (whichever your `$SHELL` is)
  after backing it up.
- **Process Manager** — kill lingering dev servers (Cursor, Claude Code, vite,
  and friends) that didn't shut down cleanly.
- **Bookmark Directory** — save the current directory as a `temp` alias so you
  can jump back to it later.
- **Usage Viewer** — open the Claude API usage dashboard in your browser.
- **Port Scanner** — scan common ports, the top 100, all 65,535, or a custom
  range, with basic service detection and the option to save results.

The menu also lists Env Manager, PATH Manager, SSH Manager, Git Shortcuts,
System Info, and Settings — those are **placeholders** right now (they show
"Coming soon" when selected). They're in the menu so the layout is stable as I
fill them in.

## Install

```bash
# one-liner
curl -sSL https://raw.githubusercontent.com/JadenB9/mode/main/quick-install.sh | bash

# or with cargo, if you have Rust
cargo install --git https://github.com/JadenB9/mode.git

# or from source
git clone https://github.com/JadenB9/mode.git && cd mode && ./install.sh
```

The installer builds in release mode, drops the binary in `~/.local/bin`,
makes sure that's on your `PATH`, and sets up the shell wrapper that alias and
bookmark integration need. Open a new terminal (or `source ~/.bashrc`)
afterward, then run `mode`.

## Keys

| Key | Action |
|-----|--------|
| `↑ ↓` / `j k` | Move through the menu |
| `Enter` | Select |
| `Esc` | Back / cancel |
| `q` | Quit (from the main menu) |

## Why aliases and bookmarks need a wrapper

A program can't change its parent shell's environment — when `mode` exits, any
`cd` or `export` it ran is gone. So the alias/bookmark features write to your
rc file and the install step adds a small shell function (`mode-wrapper.sh` /
`.zsh`) that sources those changes back into your live shell. That's the only
reason the wrapper exists.

## Building

Needs Rust 1.70+.

```bash
cargo build --release   # optimized binary in target/release/mode
cargo test              # unit tests (alias validation, shell detection, file ops)
cargo run               # run in place
```

## Layout

```
src/
  main.rs            entry point + terminal setup
  app.rs             application state machine
  event.rs           input/event loop
  ui.rs              render coordinator
  menu/              menu items, state, navigation
  features/          the five working tools + placeholder
  ui_components/     logo, menu view, input dialog, theme
  utils/             shell detection, file ops (atomic writes + backups), errors
```

## License

MIT — see [LICENSE](LICENSE).
