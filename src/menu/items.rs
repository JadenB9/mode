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
    /// Git Shortcuts - Placeholder for managing git aliases
    GitShortcuts,
    /// Scanner - Active feature for port scanning
    Scanner,
    /// Settings - Placeholder for app configuration
    Settings,
}

impl MenuItem {
    /// Returns all menu items in display order
    pub fn all() -> Vec<MenuItem> {
        vec![
            MenuItem::AliasManager,
            MenuItem::ProcessManager,
            MenuItem::Bookmark,
            MenuItem::UsageViewer,
            MenuItem::GitShortcuts,
            MenuItem::Scanner,
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
            MenuItem::GitShortcuts => "Coming soon",
            MenuItem::Scanner => "Port Scanner",
            MenuItem::Settings => "Coming soon",
        }
    }

    /// Returns the description of the menu item
    pub fn description(&self) -> &'static str {
        match self {
            MenuItem::AliasManager => "Create and manage shell aliases",
            MenuItem::ProcessManager => "Kill lingering dev server processes",
            MenuItem::Bookmark => "Save current directory as 'temp' alias",
            MenuItem::UsageViewer => "View Claude API usage in browser",
            MenuItem::GitShortcuts => "Coming soon",
            MenuItem::Scanner => "Network port scanner with service detection",
            MenuItem::Settings => "Coming soon",
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
