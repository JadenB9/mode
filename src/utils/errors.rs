use thiserror::Error;

/// Custom error types for the MODE application
#[derive(Error, Debug)]
pub enum ModeError {
    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Shell detection error
    #[error("Could not detect shell: {0}")]
    ShellDetection(String),

    /// Invalid alias name
    #[error("Invalid alias name: {0}")]
    InvalidAliasName(String),

    /// Duplicate alias
    #[error("Alias '{0}' already exists in RC file")]
    DuplicateAlias(String),

    /// RC file not found
    #[error("RC file not found: {0}")]
    RcFileNotFound(String),

    /// RC file not writable
    #[error("RC file is not writable: {0}")]
    RcFileNotWritable(String),

    /// Backup creation failed
    #[error("Failed to create backup: {0}")]
    BackupFailed(String),

    /// File operation failed
    #[error("File operation failed: {0}")]
    FileOperation(String),

    /// Invalid input
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Terminal error
    #[error("Terminal error: {0}")]
    Terminal(String),

    /// Generic error
    #[error("{0}")]
    Generic(String),
}

/// Result type for MODE operations
pub type Result<T> = std::result::Result<T, ModeError>;
