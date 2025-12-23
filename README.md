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

## ✨ Features

- **🎨 Beautiful TUI**: Professional color scheme with smooth navigation
- **⚡ Blazing Fast**: Built with Rust for maximum performance (< 100ms startup time)
- **🛡️ Safe Operations**: Automatic backups before any file modifications
- **🐚 Multi-Shell Support**: Works with both bash and zsh
- **⌨️ Intuitive Controls**: Arrow keys, vim keys, or mouse navigation
- **📦 Modular Design**: Easy to extend with new features

### Active Features

- **Alias Manager**: Create and manage shell aliases with automatic detection of your shell (bash/zsh)
  - Validates alias names
  - Checks for duplicates
  - Creates timestamped backups
  - Safe atomic file operations

### Coming Soon

- Environment Variables Manager
- PATH Manager
- SSH Config Manager
- Git Shortcuts Manager
- System Info Display
- Settings & Configuration

## 🚀 Quick Start

### Installation

#### Option 1: Using the install script (Recommended)

```bash
git clone https://github.com/JadenB9/mode.git
cd mode
./install.sh
```

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

Simply run:

```bash
mode
```

## 🎮 Usage

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

## 🏗️ Architecture

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

## 🛠️ Development

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

## 🧪 Testing

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

## 📝 Examples

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

## 🔒 Security & Safety

- **Automatic Backups**: Every file modification creates a timestamped backup
- **Atomic Operations**: Uses temporary files and atomic renames
- **Input Validation**: Prevents injection and reserved keyword usage
- **No Remote Access**: Completely offline, no network requests
- **Permission Checks**: Verifies file writability before modifications

## 🎨 Theme

MODE uses a professional color scheme optimized for terminal viewing:

- **Primary**: Cyan (#00D9FF)
- **Secondary**: Purple (#A855F7)
- **Accent**: Green (#10B981)
- **Success**: Green (#A6E3A1)
- **Error**: Red (#F38BA8)

## 📊 Performance

- **Startup Time**: < 100ms
- **Input Latency**: < 16ms (60 FPS)
- **Memory Usage**: < 5MB
- **Binary Size**: < 2MB (stripped release)

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 👤 Author

**JadenB9**
- GitHub: [@JadenB9](https://github.com/JadenB9)

## 🙏 Acknowledgments

- Built with [ratatui](https://github.com/ratatui-org/ratatui) - An amazing TUI framework
- Powered by [crossterm](https://github.com/crossterm-rs/crossterm) - Cross-platform terminal manipulation

## 📞 Support

If you encounter any issues or have questions:
- Open an issue on [GitHub](https://github.com/JadenB9/mode/issues)
- Check the [ARCHITECTURE.md](docs/ARCHITECTURE.md) for implementation details

---

Made with ❤️ and Rust
