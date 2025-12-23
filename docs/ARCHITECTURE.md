# MODE Architecture Documentation

This document provides a comprehensive overview of the MODE application architecture.

## Table of Contents

1. [System Overview](#system-overview)
2. [Module Structure](#module-structure)
3. [State Machine](#state-machine)
4. [Event Loop](#event-loop)
5. [UI Rendering](#ui-rendering)
6. [Feature System](#feature-system)
7. [File Operations](#file-operations)
8. [Extension Points](#extension-points)

## System Overview

MODE is a terminal user interface (TUI) application built using the actor/state machine pattern. The application consists of several key components that work together to provide a smooth, professional user experience.

### Key Design Principles

1. **Safety First**: All file operations use atomic writes with automatic backups
2. **Performance**: Rust + ratatui provide sub-16ms rendering times
3. **Modularity**: Easy to add new features through the Feature trait pattern
4. **User Experience**: Smooth animations, intuitive controls, professional aesthetics

## Module Structure

```
src/
├── main.rs              # Entry point, terminal lifecycle
├── lib.rs               # Public API exports
├── app.rs               # Application state machine
├── ui.rs                # UI rendering coordinator
├── event.rs             # Event handling system
├── menu/
│   ├── mod.rs
│   ├── items.rs         # Menu item definitions
│   └── state.rs         # Menu navigation state
├── features/
│   ├── mod.rs
│   ├── alias_manager.rs # Alias Manager implementation
│   └── placeholder.rs   # Placeholder for future features
├── ui_components/
│   ├── mod.rs
│   ├── logo.rs          # ASCII art logo
│   ├── theme.rs         # Color scheme and styles
│   ├── menu_view.rs     # Menu rendering
│   └── input_dialog.rs  # Input dialogs and messages
└── utils/
    ├── mod.rs
    ├── shell.rs         # Shell detection and validation
    ├── file_ops.rs      # Safe file operations
    └── errors.rs        # Error types
```

### Module Responsibilities

#### `main.rs`
- Terminal initialization and cleanup
- Enter/exit alternate screen
- Enable/disable raw mode
- Main event loop execution
- Error handling and graceful shutdown

#### `app.rs`
- Application state management
- Event routing based on current state
- Feature activation and lifecycle
- State transitions

#### `ui.rs`
- Coordinates all UI rendering
- Dispatches to appropriate renderers based on state
- Frame management

#### `event.rs`
- Background event polling thread
- Keyboard, mouse, and resize events
- Tick generation for animations
- Event channel communication

## State Machine

The application uses a hierarchical state machine pattern:

### Top-Level States

```
┌──────────────┐
│  Main Menu   │
└──────┬───────┘
       │
       ├─────────────┐
       ↓             ↓
┌──────────────┐   ┌──────────────┐
│   Feature    │   │   Exiting    │
│   Active     │   └──────────────┘
└──────────────┘
```

### Feature States (Alias Manager Example)

```
┌──────────────────┐
│ Entering Name    │
└────────┬─────────┘
         ↓
┌──────────────────┐
│ Entering Command │
└────────┬─────────┘
         ↓
┌──────────────────┐
│   Confirming     │
└────────┬─────────┘
         ↓
┌──────────────────┐
│   Processing     │
└────────┬─────────┘
         ↓
┌──────────────────┐    ┌──────────────────┐
│     Success      │    │      Error       │
└──────────────────┘    └──────────────────┘
```

### State Transitions

| From State | Event | To State |
|------------|-------|----------|
| MainMenu | Enter on item | FeatureActive |
| MainMenu | 'q' key | Exiting |
| FeatureActive | ESC | MainMenu |
| FeatureActive | Feature complete | MainMenu |

## Event Loop

The event loop uses a producer-consumer pattern with a background thread.

### Event Flow

```
┌─────────────────┐
│  Event Thread   │
│  (Background)   │
│                 │
│  ┌───────────┐  │
│  │ Poll      │  │
│  │ Events    │  │
│  └─────┬─────┘  │
│        │        │
│        ↓        │
│  ┌───────────┐  │
│  │ Generate  │  │
│  │ Ticks     │  │
│  └─────┬─────┘  │
│        │        │
└────────┼────────┘
         │
         ↓ (Channel)
┌────────┴────────┐
│   Main Thread   │
│                 │
│  ┌───────────┐  │
│  │  Receive  │  │
│  │  Event    │  │
│  └─────┬─────┘  │
│        │        │
│        ↓        │
│  ┌───────────┐  │
│  │  Handle   │  │
│  │  Event    │  │
│  └─────┬─────┘  │
│        │        │
│        ↓        │
│  ┌───────────┐  │
│  │  Render   │  │
│  └───────────┘  │
└─────────────────┘
```

### Event Types

- **Key**: Keyboard input (arrows, letters, Enter, ESC)
- **Mouse**: Mouse events (clicks, scrolls)
- **Resize**: Terminal size changes
- **Tick**: Periodic events for animations (250ms)

## UI Rendering

MODE uses ratatui's immediate mode rendering pattern.

### Rendering Pipeline

```
Terminal::draw()
     ↓
ui::render(app)
     ↓
match app.state {
    MainMenu => menu_view::render_menu()
    FeatureActive => render_feature()
}
     ↓
Frame buffer updated
     ↓
Terminal displays changes
```

### Layout System

Uses ratatui's constraint-based layout:

```
┌────────────────────────────────────┐
│          Logo (Height: 8)          │
├────────────────────────────────────┤
│                                    │
│        Menu (Min Height: 10)       │
│                                    │
├────────────────────────────────────┤
│        Help Text (Height: 3)       │
└────────────────────────────────────┘
```

### Theme System

Centralized in `ui_components/theme.rs`:

- Color constants (PRIMARY, SECONDARY, ACCENT, etc.)
- Style presets (logo(), menu_item(), input(), etc.)
- Consistent visual identity across all components

## Feature System

Features are implemented as state machines within the `ActiveFeature` enum.

### Feature Interface Pattern

```rust
pub enum ActiveFeature {
    AliasManager(AliasManager),
    Placeholder(PlaceholderFeature),
    // Future features here
}
```

### Adding a New Feature

1. Create new file in `src/features/`
2. Define feature struct with state enum
3. Implement state machine logic
4. Add to `MenuItem` enum in `menu/items.rs`
5. Add to `ActiveFeature` in `app.rs`
6. Add rendering logic in `ui.rs`

Example structure:

```rust
pub struct MyFeature {
    state: MyFeatureState,
}

pub enum MyFeatureState {
    Initial,
    Processing,
    Success { message: String },
    Error { message: String },
}

impl MyFeature {
    pub fn new() -> Self { /* ... */ }
    pub fn handle_input(&mut self, input: char) { /* ... */ }
    pub fn advance(&mut self) -> Result<()> { /* ... */ }
}
```

## File Operations

All file operations follow strict safety protocols.

### Atomic Write Pattern

```
1. Read current file content
   ↓
2. Create timestamped backup
   ↓
3. Create temp file in same directory
   ↓
4. Write new content to temp
   ↓
5. Flush and sync temp file
   ↓
6. Atomic rename (temp → original)
   ↓
7. Verify success
```

### Backup Strategy

Backup files use the format: `.bashrc.backup.20250122_183045`

- Timestamp format: `YYYYMMDD_HHMMSS`
- Stored in same directory as original
- Never automatically deleted
- User can manually clean up old backups

### Shell Detection

1. Read `$SHELL` environment variable
2. Check if contains "bash" or "zsh"
3. Determine RC file path (`~/.bashrc` or `~/.zshrc`)
4. Verify file exists and is writable
5. Return `ShellType` enum

### Alias Validation

- Must not be empty
- Must not start with a digit
- Must contain only alphanumeric characters and underscores
- Must not be a shell reserved keyword
- Must not already exist in RC file

## Extension Points

### Adding Menu Items

Edit `src/menu/items.rs`:

```rust
pub enum MenuItem {
    AliasManager,
    NewFeature,  // Add here
    // ...
}

impl MenuItem {
    pub fn all() -> Vec<MenuItem> {
        vec![
            MenuItem::AliasManager,
            MenuItem::NewFeature,  // And here
            // ...
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            MenuItem::NewFeature => "My New Feature",
            // ...
        }
    }

    pub fn is_active(&self) -> bool {
        match self {
            MenuItem::NewFeature => true,
            // ...
        }
    }
}
```

### Custom Error Types

Add to `src/utils/errors.rs`:

```rust
#[derive(Error, Debug)]
pub enum ModeError {
    // Existing errors...

    #[error("My custom error: {0}")]
    CustomError(String),
}
```

### Custom UI Components

Create new file in `src/ui_components/`:

```rust
use ratatui::{Frame, layout::Rect};
use super::theme::Theme;

pub fn render_my_component(frame: &mut Frame, area: Rect) {
    // Use Theme for consistent styling
    let style = Theme::text();

    // Render your component
}
```

## Performance Considerations

### Optimizations

1. **Rendering**: Only redraw on state changes or events
2. **Event Loop**: 250ms tick rate balances responsiveness and CPU usage
3. **Binary Size**: Release mode with LTO and strip reduces size
4. **Memory**: Minimal allocations, stack-based where possible

### Profiling

```bash
# Build with profiling symbols
cargo build --release

# Profile with perf
perf record ./target/release/mode
perf report

# Memory profiling
valgrind --tool=massif ./target/release/mode
```

## Testing Strategy

### Unit Tests

- Shell detection logic
- Alias validation
- File backup creation
- Menu navigation

### Integration Tests

- Full alias creation workflow
- Error recovery
- State transitions

### Manual Testing

- Different terminal emulators
- Both bash and zsh
- Edge cases (empty files, large files, permission errors)
- Keyboard, mouse, and resize events

## Future Enhancements

Planned features with architectural considerations:

1. **Config File**: Use `serde` + `toml` for user preferences
2. **Plugin System**: Dynamic feature loading
3. **Themes**: User-customizable color schemes
4. **History**: Track and undo changes
5. **Import/Export**: Backup and restore configurations

---

Last Updated: 2025-12-22
Version: 0.1.0
