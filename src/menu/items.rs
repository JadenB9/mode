/// Menu item definition
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuItem {
    /// Alias Manager - Active feature for managing shell aliases
    AliasManager,
    /// Environment Variables - Placeholder for managing shell exports
    EnvironmentVariables,
    /// PATH Manager - Placeholder for managing PATH entries
    PathManager,
    /// SSH Config Manager - Placeholder for managing ~/.ssh/config
    SshConfigManager,
    /// Git Shortcuts - Placeholder for managing git aliases
    GitShortcuts,
    /// System Info - Placeholder for displaying system information
    SystemInfo,
    /// Settings - Placeholder for app configuration
    Settings,
}

impl MenuItem {
    /// Returns all menu items in display order
    pub fn all() -> Vec<MenuItem> {
        vec![
            MenuItem::AliasManager,
            MenuItem::EnvironmentVariables,
            MenuItem::PathManager,
            MenuItem::SshConfigManager,
            MenuItem::GitShortcuts,
            MenuItem::SystemInfo,
            MenuItem::Settings,
        ]
    }

    /// Returns the display name of the menu item
    pub fn name(&self) -> &'static str {
        match self {
            MenuItem::AliasManager => "Alias Manager",
            MenuItem::EnvironmentVariables => "Environment Variables",
            MenuItem::PathManager => "PATH Manager",
            MenuItem::SshConfigManager => "SSH Config Manager",
            MenuItem::GitShortcuts => "Git Shortcuts",
            MenuItem::SystemInfo => "System Info",
            MenuItem::Settings => "Settings",
        }
    }

    /// Returns the description of the menu item
    pub fn description(&self) -> &'static str {
        match self {
            MenuItem::AliasManager => "Create and manage shell aliases",
            MenuItem::EnvironmentVariables => "Coming soon",
            MenuItem::PathManager => "Coming soon",
            MenuItem::SshConfigManager => "Coming soon",
            MenuItem::GitShortcuts => "Coming soon",
            MenuItem::SystemInfo => "Coming soon",
            MenuItem::Settings => "Coming soon",
        }
    }

    /// Returns whether the menu item is active (implemented)
    pub fn is_active(&self) -> bool {
        matches!(self, MenuItem::AliasManager)
    }

    /// Returns the total number of menu items
    pub fn count() -> usize {
        Self::all().len()
    }
}
