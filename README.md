# MODE - Terminal Utility Manager

```
 ███╗   ███╗ ██████╗ ██████╗ ███████╗
 ████╗ ████║██╔═══██╗██╔══██╗██╔════╝
 ██╔████╔██║██║   ██║██║  ██║█████╗
 ██║╚██╔╝██║██║   ██║██║  ██║██╔══╝
 ██║ ╚═╝ ██║╚██████╔╝██████╔╝███████╗
 ╚═╝     ╚═╝ ╚═════╝ ╚═════╝ ╚══════╝
```

A professional, blazing-fast terminal utility manager with a beautiful TUI interface. Built with Rust and ratatui for smooth, stutter-free performance.

## Features

- **Beautiful TUI**: Professional color scheme with smooth navigation
- **Blazing Fast**: Built with Rust for maximum performance (< 100ms startup time)
- **Safe Operations**: Automatic backups before any file modifications
- **Multi-Shell Support**: Works with both bash and zsh
- **Intuitive Controls**: Arrow keys, vim keys, or mouse navigation
- **Modular Design**: Easy to extend with new features

### Active Features

- **Alias Manager**: Create and manage shell aliases with automatic detection of your shell (bash/zsh)
  - Validates alias names
  - Checks for duplicates
  - Creates timestamped backups
  - Safe atomic file operations

- **Process Manager**: Kill lingering development processes
  - Cursor servers
  - Claude Code servers
  - Common development servers (vite, webpack, etc.)

- **Bookmark Directory**: Save current directory as 'temp' alias
  - Quick temporary directory bookmarking
  - Instant shell integration with wrapper scripts

- **Usage Viewer**: View Claude API usage statistics
  - Opens usage dashboard in browser
  - Quick access to API metrics

- **Port Scanner**: Network port scanning tool
  - Quick scan of common ports (14 ports)
  - Standard scan of top 100 ports
  - Full scan of all 65535 ports
  - Custom port range scanning with flexible syntax
  - Target validation for IP addresses and hostnames
  - Service detection for identified ports
  - Save results to file
  - Beautiful results display with scrolling

### Coming Soon

- **Environment Variables Manager**: Manage and edit environment variables
  - View all environment variables in a searchable list
  - Add, edit, and delete environment variables
  - Automatic backup of shell configuration files
  - Support for both session and persistent variables

- **PATH Manager**: Organize and clean up your PATH
  - View all PATH entries with validation
  - Add or remove directories from PATH
  - Detect and highlight non-existent paths
  - Reorder PATH entries for priority management

- **SSH Config Manager**: Manage SSH host configurations
  - View and edit SSH config entries
  - Add new hosts with templates
  - Test connections and validate configurations
  - Organize hosts by groups or tags

- **Git Shortcuts Manager**: Create and manage git aliases and shortcuts
  - Pre-built templates for common git workflows
  - Custom git alias creation
  - Integration with global git config
  - Visual git command builder

- **System Info Display**: Quick overview of system resources
  - CPU, memory, and disk usage
  - Network interface information
  - Running processes summary
  - System uptime and load averages

- **Dotfiles Backup**: Backup and restore configuration files
  - Automated backup of dotfiles and configs
  - Version control integration
  - Selective restore of specific files
  - Cloud sync support for backups

- **Settings & Configuration**: Customize MODE behavior
  - Theme customization
  - Keyboard shortcut configuration
  - Default behaviors and preferences
  - Import/export settings

## Quick Start

### Installation

Choose your preferred method:

#### 🚀 Quick Install (One Command)

```bash
curl -sSL https://raw.githubusercontent.com/JadenB9/mode/main/quick-install.sh | bash
```

Then reload your shell: `source ~/.bashrc` (or open a new terminal)

#### 📦 Using Cargo (if you have Rust installed)

```bash
cargo install --git https://github.com/JadenB9/mode.git
```

**Coming soon:** `cargo install mode` (once published to crates.io)

#### 🔧 From Source (Full Control)

```bash
git clone https://github.com/JadenB9/mode.git && cd mode && ./install.sh
```

---

**What the installer does automatically:**
- ✅ Installs Rust if not present (via rustup)
- ✅ Installs build tools (gcc/build-essential) if needed
- ✅ Builds MODE in release mode
- ✅ Installs the binary to `~/.local/bin`
- ✅ Adds to PATH if not already there
- ✅ Sets up shell integration for seamless alias/bookmark functionality
- ✅ Returns you to your original directory

**After installation:** Just type `mode` from anywhere!

#### Option 2: Manual installation

```bash
git clone https://github.com/JadenB9/mode.git
cd mode
cargo build --release
cp target/release/mode ~/.local/bin/
```

Make sure `~/.local/bin` is in your PATH:

```bash
export PATH="$HOME/.local/bin:$PATH"
```

Add this line to your `~/.bashrc` or `~/.zshrc` to make it permanent.

### Running

After installation, simply run:

```bash
mode
```

**First time after install?** Either:
- Reload your shell: `source ~/.bashrc` (or `source ~/.zshrc` for zsh)
- Or open a new terminal window

## Usage

### Navigation

- **↑/↓** or **j/k**: Navigate menu items
- **Enter**: Select menu item
- **ESC**: Go back / Cancel
- **q**: Quit (from main menu)

### Creating an Alias

1. Launch MODE with `mode`
2. Select "Alias Manager" from the menu
3. Enter your alias name (e.g., `ll`, `gs`)
4. Enter the command (e.g., `ls -la`, `git status`)
5. Confirm the alias
6. MODE will:
   - Detect your shell (bash or zsh)
   - Create a timestamped backup of your RC file
   - Safely append the alias
   - Show you how to reload your shell

## Architecture

MODE is built with a clean, modular architecture:

```
mode/
├── src/
│   ├── main.rs           # Entry point & terminal setup
│   ├── app.rs            # Application state machine
│   ├── ui.rs             # UI rendering coordinator
│   ├── event.rs          # Event handling
│   ├── menu/             # Menu system
│   ├── features/         # Feature implementations
│   ├── ui_components/    # Reusable UI components
│   └── utils/            # Utilities (shell, file ops, errors)
├── install.sh            # Installation script
├── uninstall.sh          # Uninstallation script
└── README.md             # This file
```

## Development

### Building from Source

Requirements:
- Rust 1.70 or later
- Linux (Kali WSL supported)

```bash
# Clone the repository
git clone https://github.com/JadenB9/mode.git
cd mode

# Build in development mode
cargo build

# Build in release mode (optimized)
cargo build --release

# Run tests
cargo test

# Run the application
cargo run
```

### Project Structure

- **App State Machine**: Handles transitions between Main Menu, Active Features, and Exiting states
- **Event Loop**: 250ms tick rate for smooth animations
- **Safe File Operations**: Atomic writes with automatic backups
- **Shell Detection**: Automatically detects bash or zsh from `$SHELL`
- **Input Validation**: Validates alias names against shell reserved keywords

## Testing

Run the test suite:

```bash
cargo test
```

Manual testing checklist:
- ✅ Create alias in bash
- ✅ Create alias in zsh
- ✅ Duplicate alias detection
- ✅ Invalid alias name validation
- ✅ Backup file creation
- ✅ Terminal resize handling
- ✅ All navigation methods (arrows, vim keys, mouse)

## Examples

### Creating a Git Alias

```bash
# Launch mode
mode

# Select "Alias Manager"
# Enter alias name: gs
# Enter command: git status
# Confirm: Y

# Reload shell
source ~/.bashrc  # or source ~/.zshrc
```

Now you can use:

```bash
gs  # Same as: git status
```

### Creating a Custom Alias

```bash
# Create an alias for updating system
# Alias name: update
# Command: sudo apt update && sudo apt upgrade -y
```

## Security & Safety

- **Automatic Backups**: Every file modification creates a timestamped backup
- **Atomic Operations**: Uses temporary files and atomic renames
- **Input Validation**: Prevents injection and reserved keyword usage
- **No Remote Access**: Completely offline, no network requests
- **Permission Checks**: Verifies file writability before modifications

## Theme

MODE uses a professional color scheme optimized for terminal viewing:

- **Primary**: Cyan (#00D9FF)
- **Secondary**: Purple (#A855F7)
- **Accent**: Green (#10B981)
- **Success**: Green (#A6E3A1)
- **Error**: Red (#F38BA8)

## Performance

- **Startup Time**: < 100ms
- **Input Latency**: < 16ms (60 FPS)
- **Memory Usage**: < 5MB
- **Binary Size**: < 2MB (stripped release)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

**JadenB9**
- GitHub: [@JadenB9](https://github.com/JadenB9)

## Acknowledgments

- Built with [ratatui](https://github.com/ratatui-org/ratatui) - An amazing TUI framework
- Powered by [crossterm](https://github.com/crossterm-rs/crossterm) - Cross-platform terminal manipulation

## Support

If you encounter any issues or have questions:
- Open an issue on [GitHub](https://github.com/JadenB9/mode/issues)
- Check the [ARCHITECTURE.md](docs/ARCHITECTURE.md) for implementation details

---

Made with Rust
