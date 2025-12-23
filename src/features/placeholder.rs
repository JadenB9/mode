/// Placeholder feature for future implementation
#[derive(Debug)]
pub struct PlaceholderFeature {
    name: String,
}

impl PlaceholderFeature {
    /// Creates a new placeholder feature
    pub fn new(name: String) -> Self {
        Self { name }
    }

    /// Returns the message to display
    pub fn get_message(&self) -> String {
        format!(
            "{} - Coming Soon!\n\n\
            This feature is planned for a future release.\n\n\
            Press ESC to return to the main menu.",
            self.name
        )
    }
}
