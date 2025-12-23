use super::items::MenuItem;

/// Menu navigation state
#[derive(Debug, Clone)]
pub struct MenuState {
    /// Currently selected menu item index
    selected: usize,
    /// Total number of menu items
    items_count: usize,
}

impl MenuState {
    /// Creates a new menu state
    pub fn new() -> Self {
        Self {
            selected: 0,
            items_count: MenuItem::count(),
        }
    }

    /// Returns the currently selected index
    pub fn selected(&self) -> usize {
        self.selected
    }

    /// Returns the currently selected menu item
    pub fn selected_item(&self) -> MenuItem {
        MenuItem::all()[self.selected]
    }

    /// Moves selection up
    pub fn previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        } else {
            // Wrap to bottom
            self.selected = self.items_count - 1;
        }
    }

    /// Moves selection down
    pub fn next(&mut self) {
        if self.selected < self.items_count - 1 {
            self.selected += 1;
        } else {
            // Wrap to top
            self.selected = 0;
        }
    }

    /// Sets the selected index directly
    pub fn select(&mut self, index: usize) {
        if index < self.items_count {
            self.selected = index;
        }
    }

    /// Resets selection to the first item
    pub fn reset(&mut self) {
        self.selected = 0;
    }
}

impl Default for MenuState {
    fn default() -> Self {
        Self::new()
    }
}
