use crate::utils::{self, file_ops, Result, ShellType};
use std::path::PathBuf;

/// Alias Manager state machine
#[derive(Debug, Clone)]
pub enum AliasManagerState {
    /// Entering alias name
    EnteringName { input: String },
    /// Entering command
    EnteringCommand { name: String, input: String },
    /// Confirming the alias before creating
    Confirming { name: String, command: String },
    /// Processing (creating alias and backup)
    Processing,
    /// Success state with message
    Success { message: String },
    /// Error state with message
    Error { message: String },
}

/// Alias Manager feature
#[derive(Debug)]
pub struct AliasManager {
    pub state: AliasManagerState,
    shell: Option<ShellType>,
    rc_file: Option<PathBuf>,
}

impl AliasManager {
    /// Creates a new Alias Manager
    pub fn new() -> Self {
        Self {
            state: AliasManagerState::EnteringName {
                input: String::new(),
            },
            shell: None,
            rc_file: None,
        }
    }

    /// Initializes the alias manager by detecting shell and RC file
    pub fn initialize(&mut self) -> Result<()> {
        let shell = utils::detect_shell()?;
        let rc_file = utils::get_rc_file_path(shell)?;

        self.shell = Some(shell);
        self.rc_file = Some(rc_file);

        Ok(())
    }

    /// Handles input for the current state
    pub fn handle_char(&mut self, c: char) {
        match &mut self.state {
            AliasManagerState::EnteringName { input } => {
                input.push(c);
            }
            AliasManagerState::EnteringCommand { input, .. } => {
                input.push(c);
            }
            _ => {}
        }
    }

    /// Handles backspace
    pub fn handle_backspace(&mut self) {
        match &mut self.state {
            AliasManagerState::EnteringName { input } => {
                input.pop();
            }
            AliasManagerState::EnteringCommand { input, .. } => {
                input.pop();
            }
            _ => {}
        }
    }

    /// Advances to the next state
    pub fn advance(&mut self) -> Result<()> {
        match &self.state {
            AliasManagerState::EnteringName { input } => {
                let name = input.trim().to_string();

                // Validate alias name
                if let Err(e) = utils::validate_alias_name(&name) {
                    self.state = AliasManagerState::Error {
                        message: e.to_string(),
                    };
                    return Ok(());
                }

                // Check for duplicates
                if let Some(rc_file) = &self.rc_file {
                    match file_ops::check_duplicate_alias(rc_file, &name) {
                        Ok(true) => {
                            self.state = AliasManagerState::Error {
                                message: format!("Alias '{}' already exists in RC file", name),
                            };
                            return Ok(());
                        }
                        Ok(false) => {
                            // Continue to next state
                            self.state = AliasManagerState::EnteringCommand {
                                name,
                                input: String::new(),
                            };
                        }
                        Err(e) => {
                            self.state = AliasManagerState::Error {
                                message: format!("Failed to check for duplicates: {}", e),
                            };
                            return Ok(());
                        }
                    }
                }
            }
            AliasManagerState::EnteringCommand { name, input } => {
                let command = input.trim().to_string();

                if command.is_empty() {
                    self.state = AliasManagerState::Error {
                        message: "Command cannot be empty".to_string(),
                    };
                    return Ok(());
                }

                self.state = AliasManagerState::Confirming {
                    name: name.clone(),
                    command,
                };
            }
            AliasManagerState::Confirming { name, command } => {
                // Clone values before changing state
                let name_clone = name.clone();
                let command_clone = command.clone();

                self.state = AliasManagerState::Processing;

                // Create the alias
                if let Some(rc_file) = &self.rc_file {
                    match file_ops::append_alias(rc_file, &name_clone, &command_clone) {
                        Ok(backup_path) => {
                            let shell_name = self.shell.map(|s| s.name()).unwrap_or("shell");
                            self.state = AliasManagerState::Success {
                                message: format!(
                                    "Success! Alias '{}' added to {}\n\n\
                                    Backup created: {}\n\n\
                                    To use the alias, reload your shell:\n  source ~/{}\n\n\
                                    Or start a new terminal session.",
                                    name_clone,
                                    rc_file.display(),
                                    backup_path.display(),
                                    self.shell
                                        .map(|s| s.rc_file_name())
                                        .unwrap_or(&shell_name)
                                ),
                            };
                        }
                        Err(e) => {
                            self.state = AliasManagerState::Error {
                                message: format!("Failed to create alias: {}", e),
                            };
                        }
                    }
                }
            }
            _ => {}
        }

        Ok(())
    }

    /// Goes back to the previous state or cancels
    pub fn go_back(&mut self) {
        match &self.state {
            AliasManagerState::EnteringCommand { .. } => {
                self.state = AliasManagerState::EnteringName {
                    input: String::new(),
                };
            }
            AliasManagerState::Confirming { .. } => {
                self.state = AliasManagerState::EnteringName {
                    input: String::new(),
                };
            }
            _ => {}
        }
    }

    /// Returns whether the feature is done (success or error)
    pub fn is_done(&self) -> bool {
        matches!(
            self.state,
            AliasManagerState::Success { .. } | AliasManagerState::Error { .. }
        )
    }

    /// Resets the manager to initial state
    pub fn reset(&mut self) {
        self.state = AliasManagerState::EnteringName {
            input: String::new(),
        };
    }

    /// Gets the current prompt text
    pub fn get_prompt(&self) -> String {
        match &self.state {
            AliasManagerState::EnteringName { .. } => {
                "Enter alias name (e.g., 'll', 'gs'):".to_string()
            }
            AliasManagerState::EnteringCommand { .. } => {
                "Enter command (e.g., 'ls -la', 'git status'):".to_string()
            }
            AliasManagerState::Confirming { name, command } => {
                format!(
                    "Create this alias?\n\nalias {}='{}'\n\n[Y]es / [N]o",
                    name, command
                )
            }
            AliasManagerState::Processing => "Creating alias...".to_string(),
            AliasManagerState::Success { message } => message.clone(),
            AliasManagerState::Error { message } => format!("Error: {}", message),
        }
    }

    /// Gets the current input text
    pub fn get_input(&self) -> String {
        match &self.state {
            AliasManagerState::EnteringName { input } => input.clone(),
            AliasManagerState::EnteringCommand { input, .. } => input.clone(),
            _ => String::new(),
        }
    }
}

impl Default for AliasManager {
    fn default() -> Self {
        Self::new()
    }
}
