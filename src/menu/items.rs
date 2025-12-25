/// Menu item definition
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuItem {
    /// Alias Manager - Active feature for managing shell aliases
    AliasManager,
    /// Process Manager - Active feature for killing lingering processes
    ProcessManager,
    /// Bookmark - Active feature for temporary directory bookmarking
    Bookmark,
    /// Usage Viewer - Active feature for viewing Claude API usage
    UsageViewer,
    /// Scanner - Active feature for port scanning
    Scanner,
    /// Environment Variables Manager - Coming soon
    EnvManager,
    /// PATH Manager - Coming soon
    PathManager,
    /// SSH Config Manager - Coming soon
    SshManager,
    /// Git Shortcuts - Coming soon
    GitShortcuts,
    /// System Info Display - Coming soon
    SystemInfo,
    /// Settings - Coming soon
    Settings,
}

impl MenuItem {
    /// Returns all menu items in display order
    /// Active features first, then coming soon items
    pub fn all() -> Vec<MenuItem> {
        vec![
            // Active features
            MenuItem::AliasManager,
            MenuItem::ProcessManager,
            MenuItem::Bookmark,
            MenuItem::UsageViewer,
            MenuItem::Scanner,
            // Coming soon
            MenuItem::EnvManager,
            MenuItem::PathManager,
            MenuItem::SshManager,
            MenuItem::GitShortcuts,
            MenuItem::SystemInfo,
            MenuItem::Settings,
        ]
    }

    /// Returns the display name of the menu item
    pub fn name(&self) -> &'static str {
        match self {
            MenuItem::AliasManager => "Alias Manager",
            MenuItem::ProcessManager => "Process Manager",
            MenuItem::Bookmark => "Bookmark Directory",
            MenuItem::UsageViewer => "Usage Viewer",
            MenuItem::Scanner => "Port Scanner",
            MenuItem::EnvManager => "Environment Variables",
            MenuItem::PathManager => "PATH Manager",
            MenuItem::SshManager => "SSH Config",
            MenuItem::GitShortcuts => "Git Shortcuts",
            MenuItem::SystemInfo => "System Info",
            MenuItem::Settings => "Settings",
        }
    }

    /// Returns the description of the menu item
    pub fn description(&self) -> &'static str {
        match self {
            MenuItem::AliasManager => "Create and manage shell aliases with validation and backups",
            MenuItem::ProcessManager => "Kill lingering development server processes (cursor, claude-code, vite, etc.)",
            MenuItem::Bookmark => "Quickly save and navigate to current directory using 'temp' alias",
            MenuItem::UsageViewer => "Open Claude API usage dashboard in your browser",
            MenuItem::Scanner => "Scan network ports with service detection and custom ranges",
            MenuItem::EnvManager => "View, edit, and manage environment variables (coming soon)",
            MenuItem::PathManager => "Organize and clean up your PATH with validation (coming soon)",
            MenuItem::SshManager => "Manage SSH host configurations and connection settings (coming soon)",
            MenuItem::GitShortcuts => "Create git aliases and shortcuts with templates (coming soon)",
            MenuItem::SystemInfo => "Monitor CPU, memory, disk usage and system resources (coming soon)",
            MenuItem::Settings => "Customize themes, shortcuts, and app preferences (coming soon)",
        }
    }

    /// Returns whether the menu item is active (implemented)
    pub fn is_active(&self) -> bool {
        matches!(self, MenuItem::AliasManager | MenuItem::ProcessManager | MenuItem::Bookmark | MenuItem::UsageViewer | MenuItem::Scanner)
    }

    /// Returns the total number of menu items
    pub fn count() -> usize {
        Self::all().len()
    }
}
