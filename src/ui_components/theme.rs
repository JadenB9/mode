use ratatui::style::{Color, Modifier, Style};

/// Color scheme for the MODE application
/// Professional theme with cyan, purple, and green accents
pub struct Theme;

impl Theme {
    /// Primary color - Cyan (#00D9FF)
    pub const PRIMARY: Color = Color::Rgb(0, 217, 255);

    /// Secondary color - Purple (#A855F7)
    pub const SECONDARY: Color = Color::Rgb(168, 85, 247);

    /// Accent color - Green (#10B981)
    pub const ACCENT: Color = Color::Rgb(16, 185, 129);

    /// Background - Dark (#1E1E2E)
    pub const BACKGROUND: Color = Color::Rgb(30, 30, 46);

    /// Text - Light Gray (#CDD6F4)
    pub const TEXT: Color = Color::Rgb(205, 214, 244);

    /// Selected item - Bright Cyan
    pub const SELECTED: Color = Color::Rgb(0, 255, 255);

    /// Error - Red (#F38BA8)
    pub const ERROR: Color = Color::Rgb(243, 139, 168);

    /// Success - Green (#A6E3A1)
    pub const SUCCESS: Color = Color::Rgb(166, 227, 161);

    /// Border color
    pub const BORDER: Color = Color::Rgb(108, 112, 134);

    /// Dimmed text
    pub const DIM: Color = Color::Rgb(127, 132, 156);

    // Style presets

    /// Default text style
    pub fn text() -> Style {
        Style::default().fg(Self::TEXT)
    }

    /// Logo style (primary color, bold)
    pub fn logo() -> Style {
        Style::default()
            .fg(Self::PRIMARY)
            .add_modifier(Modifier::BOLD)
    }

    /// Menu item style
    pub fn menu_item() -> Style {
        Style::default().fg(Self::TEXT)
    }

    /// Selected menu item style
    pub fn menu_item_selected() -> Style {
        Style::default()
            .fg(Self::SELECTED)
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::REVERSED)
    }

    /// Menu item active (has content)
    pub fn menu_item_active() -> Style {
        Style::default().fg(Self::ACCENT)
    }

    /// Menu item placeholder (coming soon)
    pub fn menu_item_placeholder() -> Style {
        Style::default().fg(Self::DIM)
    }

    /// Border style
    pub fn border() -> Style {
        Style::default().fg(Self::BORDER)
    }

    /// Title style
    pub fn title() -> Style {
        Style::default()
            .fg(Self::SECONDARY)
            .add_modifier(Modifier::BOLD)
    }

    /// Help text style
    pub fn help() -> Style {
        Style::default().fg(Self::DIM)
    }

    /// Error message style
    pub fn error() -> Style {
        Style::default()
            .fg(Self::ERROR)
            .add_modifier(Modifier::BOLD)
    }

    /// Success message style
    pub fn success() -> Style {
        Style::default()
            .fg(Self::SUCCESS)
            .add_modifier(Modifier::BOLD)
    }

    /// Input field style
    pub fn input() -> Style {
        Style::default()
            .fg(Self::TEXT)
            .bg(Color::Rgb(40, 40, 60))
    }

    /// Input field focused style
    pub fn input_focused() -> Style {
        Style::default()
            .fg(Self::TEXT)
            .bg(Color::Rgb(50, 50, 80))
            .add_modifier(Modifier::BOLD)
    }

    /// Cursor style
    pub fn cursor() -> Style {
        Style::default()
            .fg(Self::BACKGROUND)
            .bg(Self::PRIMARY)
    }

    /// Dimmed text style
    pub fn dim() -> Style {
        Style::default().fg(Self::DIM)
    }

    /// Secondary style
    pub fn secondary() -> Style {
        Style::default().fg(Self::SECONDARY)
    }

    /// Accent style
    pub fn accent() -> Style {
        Style::default().fg(Self::ACCENT)
    }
}
