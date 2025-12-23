# MODE - Project Context for AI Assistants

## Project Overview

**MODE** is a professional terminal utility manager built with Rust and ratatui. It provides a beautiful, fast TUI (Text User Interface) for managing various terminal configurations and utilities.

**Current Version**: 0.1.0
**Author**: Jaden Butler (JadenB9)
**Repository**: https://github.com/JadenB9/mode
**License**: MIT

## What This Project Does

MODE is a menu-driven terminal application that helps users manage shell configurations. Currently implemented:

1. **Alias Manager**: Create and manage shell aliases
   - Auto-detects bash or zsh
   - Validates alias names (no reserved keywords, special chars)
   - Checks for duplicates before creating
   - Creates timestamped backups before modifying RC files
   - Uses atomic file operations for safety
   - Provides clear reload instructions

2. **Future Features** (placeholders): 6 additional menu items ready for implementation

## Technical Architecture

### Tech Stack
- **Language**: Rust 2021 edition
- **TUI Framework**: ratatui 0.28
- **Terminal Control**: crossterm 0.28
- **Error Handling**: anyhow + thiserror
- **File Operations**: tempfile for atomic writes
- **Time**: chrono for timestamps

### Performance Targets
- Startup time: < 100ms
- Input latency: < 16ms (60 FPS)
- Memory usage: < 5MB
- Binary size: < 2MB (stripped release)

### Project Structure

```
mode/
├── src/
│   ├── main.rs              # Entry point, terminal lifecycle
│   ├── lib.rs               # Public API exports
│   ├── app.rs               # Application state machine
│   ├── ui.rs                # UI rendering coordinator
│   ├── event.rs             # Event handling (background thread)
│   │
│   ├── menu/
│   │   ├── mod.rs
│   │   ├── items.rs         # Menu item definitions (7 items)
│   │   └── state.rs         # Menu navigation state
│   │
│   ├── features/
│   │   ├── mod.rs
│   │   ├── alias_manager.rs # Alias Manager implementation
│   │   └── placeholder.rs   # Template for future features
│   │
│   ├── ui_components/
│   │   ├── mod.rs
│   │   ├── logo.rs          # ASCII art "MODE" logo
│   │   ├── theme.rs         # Color scheme (purple theme)
│   │   ├── menu_view.rs     # Menu rendering
│   │   └── input_dialog.rs  # Input dialogs and messages
│   │
│   └── utils/
│       ├── mod.rs
│       ├── shell.rs         # Shell detection, validation
│       ├── file_ops.rs      # Safe file operations
│       └── errors.rs        # Error types
│
├── docs/
│   └── ARCHITECTURE.md      # Detailed architecture documentation
│
├── install.sh               # Installation script
├── uninstall.sh             # Uninstallation script
├── README.md                # User documentation
├── LICENSE                  # MIT License
└── Cargo.toml               # Rust project manifest
```

## Key Design Patterns

### 1. State Machine Pattern
The application uses a hierarchical state machine:

```
AppState:
  - MainMenu
  - FeatureActive(ActiveFeature)
  - Exiting

ActiveFeature:
  - AliasManager(AliasManager)
  - Placeholder(PlaceholderFeature)

AliasManagerState:
  - EnteringName
  - EnteringCommand
  - Confirming
  - Processing
  - Success
  - Error
```

### 2. Event Loop
- Background thread polls for terminal events (keyboard, mouse, resize)
- 250ms tick rate for smooth animations
- Channel-based communication to main thread
- Graceful cleanup on exit

### 3. Safe File Operations
All file modifications follow this pattern:
1. Create timestamped backup
2. Read current content
3. Validate changes
4. Write to temp file
5. Atomic rename (temp → original)
6. Verify success

### 4. Rendering Pipeline
- Immediate mode rendering with ratatui
- Double buffering (automatic)
- Only redraw on state changes or events
- Constraint-based layouts

## Color Scheme

**Current Theme**: Purple & Pink
- **Primary** (Logo): Dark Purple `#5B21B6` - RGB(91, 33, 182)
- **Secondary**: Medium Purple `#A855F7` - RGB(168, 85, 247)
- **Accent**: Light Purple/Pink `#E879F9` - RGB(232, 121, 249)
- **Background**: Dark `#1E1E2E` - RGB(30, 30, 46)
- **Text**: Light Gray `#CDD6F4` - RGB(205, 214, 244)
- **Error**: Red `#F38BA8` - RGB(243, 139, 168)
- **Success**: Light Purple/Pink `#E879F9` - RGB(232, 121, 249)

## Navigation Controls

- **↑/↓** or **j/k**: Navigate menu items
- **Enter**: Select/Confirm
- **ESC**: Go back/Cancel
- **q**: Quit (from main menu)
- **Mouse**: Click and scroll support

## File Safety Features

1. **Automatic Backups**: Format `~/.bashrc.backup.YYYYMMDD_HHMMSS`
2. **Atomic Writes**: Uses temp files + rename for atomicity
3. **Input Validation**: Prevents shell injection and reserved keywords
4. **Duplicate Detection**: Checks before creating aliases
5. **Permission Checks**: Verifies file writability before modifications

## How to Add New Features

### Adding a Menu Item

1. **Define the enum variant** in `src/menu/items.rs`:
   ```rust
   pub enum MenuItem {
       AliasManager,
       YourNewFeature,  // Add here
       // ...
   }
   ```

2. **Update implementations**:
   - Add to `all()` method
   - Add to `name()` method
   - Add to `description()` method
   - Set `is_active()` to true when implemented

3. **Create feature module** in `src/features/`:
   ```rust
   pub struct YourFeature {
       state: YourFeatureState,
   }

   pub enum YourFeatureState {
       Initial,
       Processing,
       Success { message: String },
       Error { message: String },
   }
   ```

4. **Add to ActiveFeature** in `src/app.rs`:
   ```rust
   pub enum ActiveFeature {
       AliasManager(AliasManager),
       YourNewFeature(YourFeature),  // Add here
       // ...
   }
   ```

5. **Add rendering logic** in `src/ui.rs`

## Common Tasks

### Building
```bash
cargo build              # Development build
cargo build --release    # Optimized release build
```

### Testing
```bash
cargo test              # Run all tests
cargo clippy            # Linting
cargo fmt               # Format code
```

### Installation
```bash
./install.sh            # Installs to ~/.local/bin/mode
```

### Running
```bash
mode                    # Run installed version
cargo run              # Run from source
```

## Important Files to Know

### Core Application Logic
- `src/main.rs` - Terminal setup, event loop entry point
- `src/app.rs` - State machine, event routing, feature activation
- `src/ui.rs` - Main UI coordinator

### Menu System
- `src/menu/items.rs` - Menu item definitions (7 total)
- `src/menu/state.rs` - Navigation state (which item is selected)

### Features
- `src/features/alias_manager.rs` - Complete alias creation workflow
- `src/features/placeholder.rs` - Template for future features

### UI Components
- `src/ui_components/theme.rs` - **Color scheme** (modify here for visual changes)
- `src/ui_components/logo.rs` - ASCII art logo
- `src/ui_components/menu_view.rs` - Menu rendering
- `src/ui_components/input_dialog.rs` - Input prompts and messages

### Utilities
- `src/utils/shell.rs` - Shell detection (bash/zsh), alias validation
- `src/utils/file_ops.rs` - Safe file operations, backups
- `src/utils/errors.rs` - Custom error types

## Development Guidelines

### Code Style
- Use `rustfmt` for formatting
- Run `clippy` before committing
- Add doc comments for public APIs
- Keep functions small and focused

### Safety
- NEVER modify files without backups
- ALWAYS validate user input
- Use atomic operations for file writes
- Handle errors gracefully

### Performance
- Minimize allocations in hot paths
- Use stack-based data where possible
- Profile with `cargo flamegraph` if needed
- Keep rendering under 16ms

### Testing
- Write unit tests for validation logic
- Test both bash and zsh paths
- Test error cases (duplicate aliases, invalid names, etc.)
- Manual test on actual terminals

## Known Limitations

1. **Shell Reload**: Cannot reload parent shell from subprocess - users must run `exec bash` or `exec zsh`
2. **Shell Support**: Only bash and zsh currently supported
3. **Platform**: Linux only (Windows/macOS not tested)
4. **Terminal**: Requires ANSI color support

## Future Roadmap

### Planned Features (Menu Items 2-7)
- Environment Variables Manager
- PATH Manager
- SSH Config Manager
- Git Shortcuts Manager
- System Info Display
- Settings & Configuration

### Potential Enhancements
- Plugin system for custom features
- Themes (user-customizable colors)
- History/undo for changes
- Import/export configurations
- Remote config sync

## Quick Reference

### Changing Colors
Edit `src/ui_components/theme.rs` and modify the RGB values in the `Theme` impl block.

### Changing Menu Items
Edit `src/menu/items.rs` - modify the `MenuItem` enum and implementation methods.

### Adding Dependencies
Edit `Cargo.toml` and add to `[dependencies]` section.

### Modifying Success Messages
Feature-specific messages are in each feature's module (e.g., `src/features/alias_manager.rs`).

## Git Workflow

```bash
# Make changes
git add .
git commit -m "Description of changes"
git push origin main

# Create release
git tag -a v0.x.0 -m "Release description"
git push origin v0.x.0
```

## Contact

- **Author**: Jaden Butler
- **GitHub**: [@JadenB9](https://github.com/JadenB9)
- **Email**: jadenb9944@gmail.com
- **Repository**: https://github.com/JadenB9/mode

---

**Last Updated**: 2025-12-22
**For**: AI Assistants / Future Development Reference
