use crate::utils::{self, file_ops, Result, ShellType};
use std::env;
use std::path::PathBuf;

/// Bookmark Manager state machine
#[derive(Debug, Clone)]
pub enum BookmarkManagerState {
    /// Confirming the bookmark
    Confirming { directory: String },
    /// Processing (creating/updating alias)
    Processing,
    /// Success state - triggers immediate exit
    Success { message: String },
    /// Error state with message
    Error { message: String },
}

/// Bookmark Manager feature
#[derive(Debug)]
pub struct BookmarkManager {
    pub state: BookmarkManagerState,
    shell: Option<ShellType>,
    rc_file: Option<PathBuf>,
}

impl BookmarkManager {
    /// Creates a new Bookmark Manager
    pub fn new() -> Self {
        // Get current directory immediately
        let current_dir = env::current_dir()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| String::from("unknown"));

        Self {
            state: BookmarkManagerState::Confirming {
                directory: current_dir,
            },
            shell: None,
            rc_file: None,
        }
    }

    /// Initializes the bookmark manager by detecting shell and RC file
    pub fn initialize(&mut self) -> Result<()> {
        let shell = utils::detect_shell()?;
        let rc_file = utils::get_rc_file_path(shell)?;

        self.shell = Some(shell);
        self.rc_file = Some(rc_file);

        Ok(())
    }

    /// Confirms and creates the bookmark
    pub fn confirm_bookmark(&mut self) -> Result<()> {
        if let BookmarkManagerState::Confirming { directory } = &self.state {
            let dir_clone = directory.clone();
            self.state = BookmarkManagerState::Processing;

            // Create the temp alias - will overwrite if it exists
            if let Some(rc_file) = &self.rc_file {
                // First, remove any existing "temp" alias
                if let Err(e) = file_ops::remove_alias(rc_file, "temp") {
                    // It's okay if removal fails (alias might not exist)
                    eprintln!("Note: Could not remove existing temp alias: {}", e);
                }

                // Now add the new temp alias
                // Use double quotes inside the command to handle spaces
                match file_ops::append_alias(rc_file, "temp", &format!("cd \"{}\"", dir_clone)) {
                    Ok(backup_path) => {
                        let shell_name = self.shell.map(|s| s.name()).unwrap_or("bash");

                        self.state = BookmarkManagerState::Success {
                            message: format!(
                                "✓ Temporary bookmark created!\n\n\
                                Directory: {}\n\
                                Alias: temp\n\
                                Backup: {}\n\n\
                                IMPORTANT: Reload your shell to use the alias:\n\n\
                                source ~/.{}rc\n\
                                OR\n\
                                exec {}\n\n\
                                Then type: temp",
                                dir_clone,
                                backup_path.display(),
                                shell_name,
                                shell_name
                            ),
                        };
                    }
                    Err(e) => {
                        self.state = BookmarkManagerState::Error {
                            message: format!("Failed to create bookmark: {}", e),
                        };
                    }
                }
            }
        }

        Ok(())
    }

    /// Cancels the bookmark
    pub fn cancel(&mut self) {
        // Don't change state - let the app handle going back to menu
    }

    /// Returns whether the feature is done (success or error)
    pub fn is_done(&self) -> bool {
        matches!(
            self.state,
            BookmarkManagerState::Success { .. } | BookmarkManagerState::Error { .. }
        )
    }

    /// Returns true if the bookmark was successfully created (should exit app)
    pub fn should_exit_app(&self) -> bool {
        matches!(self.state, BookmarkManagerState::Success { .. })
    }

    /// Gets the prompt text for the current state
    pub fn get_prompt(&self) -> String {
        match &self.state {
            BookmarkManagerState::Confirming { directory } => {
                directory.clone()
            }
            BookmarkManagerState::Processing => "Creating bookmark...".to_string(),
            BookmarkManagerState::Success { message } => message.clone(),
            BookmarkManagerState::Error { message } => format!("Error: {}", message),
        }
    }

    /// Gets confirmation data for structured display
    pub fn get_confirmation_data(&self) -> Option<String> {
        match &self.state {
            BookmarkManagerState::Confirming { directory } => Some(directory.clone()),
            _ => None,
        }
    }

    /// Gets the RC file path
    pub fn get_rc_file(&self) -> Option<&PathBuf> {
        self.rc_file.as_ref()
    }
}

impl Default for BookmarkManager {
    fn default() -> Self {
        Self::new()
    }
}
