# Mode Installation & Setup

## Quick Start

### 1. Build the Project
```bash
cargo build --release
```

### 2. Setup Shell Integration (Recommended)

For seamless integration where aliases and bookmarks work immediately without manual shell reloading:

#### For Bash Users
Add this to your `~/.bashrc`:
```bash
# Mode shell integration
source /path/to/mode/mode-wrapper.sh
```

#### For Zsh Users
Add this to your `~/.zshrc`:
```bash
# Mode shell integration
source /path/to/mode/mode-wrapper.zsh
```

#### Reload Your Shell
```bash
# For Bash
source ~/.bashrc

# For Zsh
source ~/.zshrc
```

### 3. Run Mode
```bash
mode
```

## Features

### 1. Alias Manager
Create shell aliases quickly with an intuitive TUI.

### 2. Process Manager
Kill lingering development processes:
- Cursor servers
- Claude Code
- Development servers (vite, webpack, etc.)

### 3. Bookmark Directory
Save your current directory as a `temp` alias for quick navigation.

## How Shell Integration Works

When you create an alias or bookmark using mode:
1. Mode creates the alias in your shell's RC file
2. Mode outputs a special command: `MODE_EXIT_CMD:source ~/.bashrc`
3. The wrapper function captures this and executes it
4. Your shell reloads, making the new alias/bookmark available immediately

**Without shell integration**: You'll need to manually run `source ~/.bashrc` (or `exec bash`) after creating aliases/bookmarks.

**With shell integration**: Everything works seamlessly - aliases and bookmarks are available immediately!

## Troubleshooting

### Aliases/Bookmarks Don't Work Immediately
Make sure you've sourced the wrapper script in your shell's RC file and reloaded your shell.

### Mode Command Not Found
Either:
1. Add the mode binary to your PATH, or
2. Update the wrapper scripts to use the full path to the mode binary

### Wrapper Function Not Working
Make sure you:
1. Sourced (not executed) the wrapper script
2. Reloaded your shell after adding it to your RC file
