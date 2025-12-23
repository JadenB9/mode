# Mode Installation & Setup

## Quick Start

### Recommended: Use the Install Script

```bash
git clone https://github.com/JadenB9/mode.git
cd mode
./install.sh
source ~/.bashrc  # Reload shell (or open a new terminal)
mode              # Launch MODE
```

The install script automatically:
- Builds MODE in release mode
- Installs the binary to `~/.local/bin`
- Sets up shell integration automatically
- Configures your `.bashrc` or `.zshrc` with the wrapper function

### Manual Installation

If you prefer to install manually:

#### 1. Build the Project
```bash
cargo build --release
```

#### 2. Install the Binary
```bash
cp target/release/mode ~/.local/bin/
chmod +x ~/.local/bin/mode
```

Make sure `~/.local/bin` is in your PATH:
```bash
export PATH="$HOME/.local/bin:$PATH"
```

#### 3. Setup Shell Integration (Optional but Recommended)

Shell integration enables automatic shell reloading after creating aliases and bookmarks.

##### For Bash Users
Add this to your `~/.bashrc`:
```bash
# Mode shell integration - auto-reload after creating aliases/bookmarks
mode() {
    local cmd_file="$HOME/.mode_exit_cmd"

    # Remove old command file
    rm -f "$cmd_file"

    # Run mode using the installed binary
    command mode "$@"
    local exit_code=$?

    # Check if mode wrote an exit command
    if [ -f "$cmd_file" ]; then
        local exit_cmd
        exit_cmd=$(cat "$cmd_file")
        rm -f "$cmd_file"

        if [ -n "$exit_cmd" ]; then
            eval "$exit_cmd"
            echo "✓ Shell configuration reloaded"
        fi
    fi

    return $exit_code
}
```

##### For Zsh Users
Add this to your `~/.zshrc`:
```bash
# Mode shell integration - auto-reload after creating aliases/bookmarks
mode() {
    local cmd_file="$HOME/.mode_exit_cmd"

    # Remove old command file
    rm -f "$cmd_file"

    # Run mode using the installed binary
    command mode "$@"
    local exit_code=$?

    # Check if mode wrote an exit command
    if [ -f "$cmd_file" ]; then
        local exit_cmd
        exit_cmd=$(cat "$cmd_file")
        rm -f "$cmd_file"

        if [ -n "$exit_cmd" ]; then
            eval "$exit_cmd"
            echo "✓ Shell configuration reloaded"
        fi
    fi

    return $exit_code
}
```

#### 4. Reload Your Shell
```bash
# For Bash
source ~/.bashrc

# For Zsh
source ~/.zshrc
```

#### 5. Run Mode
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
