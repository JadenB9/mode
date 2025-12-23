use crate::utils::Result;
use std::process::Command;

/// Process Manager state machine
#[derive(Debug, Clone)]
pub enum ProcessManagerState {
    /// Selecting which process cleanup to perform
    SelectingAction { selected: usize },
    /// Confirming the action
    Confirming { action: ProcessAction },
    /// Processing (killing processes)
    Processing { action: ProcessAction },
    /// Success state with message
    Success { message: String },
    /// Error state with message
    Error { message: String },
}

/// Available process cleanup actions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessAction {
    /// Kill all Cursor server processes
    KillCursorServers,
    /// Kill common unneeded development processes
    KillUnneededProcesses,
    /// Kill all development servers (Cursor, Claude Code, etc.)
    KillAllDevServers,
}

impl ProcessAction {
    /// Returns all available actions
    pub fn all() -> Vec<ProcessAction> {
        vec![
            ProcessAction::KillCursorServers,
            ProcessAction::KillUnneededProcesses,
            ProcessAction::KillAllDevServers,
        ]
    }

    /// Returns the display name of the action
    pub fn name(&self) -> &'static str {
        match self {
            ProcessAction::KillCursorServers => "Kill Cursor Servers",
            ProcessAction::KillUnneededProcesses => "Kill Unneeded Processes",
            ProcessAction::KillAllDevServers => "Kill All Dev Servers",
        }
    }

    /// Returns the description of the action
    pub fn description(&self) -> &'static str {
        match self {
            ProcessAction::KillCursorServers => {
                "Kills all cursor-server processes running in WSL"
            }
            ProcessAction::KillUnneededProcesses => {
                "Kills common background processes that may be lingering"
            }
            ProcessAction::KillAllDevServers => {
                "Kills Cursor servers, Claude Code, and other dev tool servers"
            }
        }
    }

    /// Executes the action and returns a result message
    pub fn execute(&self) -> Result<String> {
        match self {
            ProcessAction::KillCursorServers => Self::kill_cursor_servers(),
            ProcessAction::KillUnneededProcesses => Self::kill_unneeded_processes(),
            ProcessAction::KillAllDevServers => Self::kill_all_dev_servers(),
        }
    }

    /// Kills all Cursor server processes
    fn kill_cursor_servers() -> Result<String> {
        let output = Command::new("pkill")
            .arg("-f")
            .arg("cursor-server")
            .output()?;

        if output.status.success() || output.status.code() == Some(1) {
            // Exit code 1 means no processes found, which is OK
            Ok("✓ Cursor server processes terminated successfully".to_string())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(crate::utils::ModeError::Generic(format!("Failed to kill Cursor servers: {}", stderr)))
        }
    }

    /// Kills common unneeded processes
    fn kill_unneeded_processes() -> Result<String> {
        let processes = vec![
            "cursor-server",
            "claude",
            "node_modules/.bin",
            "webpack-dev-server",
            "vite",
            "npm-run-all",
        ];

        let mut killed = Vec::new();
        let mut errors = Vec::new();

        for process in processes {
            let output = Command::new("pkill")
                .arg("-f")
                .arg(process)
                .output()?;

            if output.status.success() {
                killed.push(process);
            } else if output.status.code() != Some(1) {
                // 1 means no process found, which is fine
                errors.push(format!("{}: {}", process, String::from_utf8_lossy(&output.stderr)));
            }
        }

        if errors.is_empty() {
            Ok(format!(
                "✓ Unneeded processes terminated\n\nCleaned up: {}",
                if killed.is_empty() {
                    "no processes found (already clean)".to_string()
                } else {
                    killed.join(", ")
                }
            ))
        } else {
            Err(crate::utils::ModeError::Generic(format!("Some errors occurred:\n{}", errors.join("\n"))))
        }
    }

    /// Kills all development server processes
    fn kill_all_dev_servers() -> Result<String> {
        let processes = vec![
            "cursor-server",
            "claude-code",
            "claude",
            "code-server",
            "node_modules/.bin",
            "webpack-dev-server",
            "vite",
            "npm-run-all",
            "tsx",
            "ts-node",
            "nodemon",
            "next-server",
        ];

        let mut killed = Vec::new();
        let mut errors = Vec::new();

        for process in processes {
            let output = Command::new("pkill")
                .arg("-f")
                .arg(process)
                .output()?;

            if output.status.success() {
                killed.push(process);
            } else if output.status.code() != Some(1) {
                errors.push(format!("{}: {}", process, String::from_utf8_lossy(&output.stderr)));
            }
        }

        if errors.is_empty() {
            Ok(format!(
                "✓ All development servers terminated\n\nCleaned up: {}",
                if killed.is_empty() {
                    "no processes found (already clean)".to_string()
                } else {
                    killed.join(", ")
                }
            ))
        } else {
            Err(crate::utils::ModeError::Generic(format!("Some errors occurred:\n{}", errors.join("\n"))))
        }
    }
}

/// Process Manager feature
#[derive(Debug)]
pub struct ProcessManager {
    pub state: ProcessManagerState,
}

impl ProcessManager {
    /// Creates a new Process Manager
    pub fn new() -> Self {
        Self {
            state: ProcessManagerState::SelectingAction { selected: 0 },
        }
    }

    /// Moves selection up
    pub fn previous(&mut self) {
        if let ProcessManagerState::SelectingAction { selected } = &mut self.state {
            let total = ProcessAction::all().len();
            *selected = if *selected == 0 {
                total - 1
            } else {
                *selected - 1
            };
        }
    }

    /// Moves selection down
    pub fn next(&mut self) {
        if let ProcessManagerState::SelectingAction { selected } = &mut self.state {
            let total = ProcessAction::all().len();
            *selected = (*selected + 1) % total;
        }
    }

    /// Confirms the selected action
    pub fn confirm_selection(&mut self) {
        if let ProcessManagerState::SelectingAction { selected } = self.state {
            let action = ProcessAction::all()[selected];
            self.state = ProcessManagerState::Confirming { action };
        }
    }

    /// Executes the confirmed action
    pub fn execute_action(&mut self) {
        if let ProcessManagerState::Confirming { action } = self.state {
            self.state = ProcessManagerState::Processing { action };

            match action.execute() {
                Ok(message) => {
                    self.state = ProcessManagerState::Success { message };
                }
                Err(e) => {
                    self.state = ProcessManagerState::Error {
                        message: e.to_string(),
                    };
                }
            }
        }
    }

    /// Goes back to the selection state
    pub fn go_back(&mut self) {
        match &self.state {
            ProcessManagerState::Confirming { .. } => {
                self.state = ProcessManagerState::SelectingAction { selected: 0 };
            }
            _ => {}
        }
    }

    /// Returns whether the feature is done (success or error)
    pub fn is_done(&self) -> bool {
        matches!(
            self.state,
            ProcessManagerState::Success { .. } | ProcessManagerState::Error { .. }
        )
    }

    /// Resets the manager to initial state
    pub fn reset(&mut self) {
        self.state = ProcessManagerState::SelectingAction { selected: 0 };
    }

    /// Gets the current selected action (if in SelectingAction state)
    pub fn get_selected(&self) -> Option<usize> {
        if let ProcessManagerState::SelectingAction { selected } = self.state {
            Some(selected)
        } else {
            None
        }
    }

    /// Gets the prompt text for the current state
    pub fn get_prompt(&self) -> String {
        match &self.state {
            ProcessManagerState::SelectingAction { .. } => {
                "Select an action (↑/↓ to navigate, Enter to select, ESC to cancel):".to_string()
            }
            ProcessManagerState::Confirming { action } => {
                format!("{}\n{}", action.name(), action.description())
            }
            ProcessManagerState::Processing { action } => {
                format!("{}...", action.name())
            }
            ProcessManagerState::Success { message } => message.clone(),
            ProcessManagerState::Error { message } => format!("Error: {}", message),
        }
    }

    /// Gets confirmation data for structured display
    pub fn get_confirmation_data(&self) -> Option<(String, String)> {
        match &self.state {
            ProcessManagerState::Confirming { action } => {
                Some((action.name().to_string(), action.description().to_string()))
            }
            _ => None,
        }
    }
}

impl Default for ProcessManager {
    fn default() -> Self {
        Self::new()
    }
}
