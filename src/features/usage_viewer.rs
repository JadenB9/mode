use crate::utils::Result;
use std::process::Command;

/// Usage Viewer state machine
#[derive(Debug, Clone)]
pub enum UsageViewerState {
    /// Ready to open browser
    Ready,
    /// Opening browser
    Opening,
    /// Success state
    Success { message: String },
    /// Error state with message
    Error { message: String },
}

/// Usage Viewer feature
#[derive(Debug)]
pub struct UsageViewer {
    pub state: UsageViewerState,
}

impl UsageViewer {
    /// Creates a new Usage Viewer
    pub fn new() -> Self {
        Self {
            state: UsageViewerState::Ready,
        }
    }

    /// Opens the Claude Console in the browser
    pub fn open_browser(&mut self) -> Result<()> {
        self.state = UsageViewerState::Opening;

        let url = "https://claude.ai/settings/usage";

        // Try different commands based on platform
        let result = if cfg!(target_os = "linux") {
            // On WSL, use PowerShell to open browser
            // First try powershell.exe (WSL), then fallback to xdg-open (native Linux)
            Command::new("powershell.exe")
                .args(&["Start-Process", url])
                .output()
                .or_else(|_| Command::new("xdg-open").arg(url).output())
        } else if cfg!(target_os = "macos") {
            Command::new("open").arg(url).output()
        } else if cfg!(target_os = "windows") {
            Command::new("cmd").args(&["/c", "start", url]).output()
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Unsupported platform",
            ))
        };

        match result {
            Ok(_) => {
                self.state = UsageViewerState::Success {
                    message: format!(
                        "✓ Opening Claude Usage page in browser\n\n\
                        URL: {}\n\n\
                        View your API usage, token consumption, and billing details.",
                        url
                    ),
                };
                Ok(())
            }
            Err(e) => {
                self.state = UsageViewerState::Error {
                    message: format!(
                        "Failed to open browser: {}\n\n\
                        Please visit manually:\n{}",
                        e, url
                    ),
                };
                Ok(())
            }
        }
    }

    /// Returns whether the feature is done (success or error)
    pub fn is_done(&self) -> bool {
        matches!(
            self.state,
            UsageViewerState::Success { .. } | UsageViewerState::Error { .. }
        )
    }

    /// Gets the prompt text for the current state
    pub fn get_prompt(&self) -> String {
        match &self.state {
            UsageViewerState::Ready => {
                "Open Claude Console in your browser to view usage statistics?".to_string()
            }
            UsageViewerState::Opening => "Opening browser...".to_string(),
            UsageViewerState::Success { message } => message.clone(),
            UsageViewerState::Error { message } => format!("Error: {}", message),
        }
    }
}

impl Default for UsageViewer {
    fn default() -> Self {
        Self::new()
    }
}
