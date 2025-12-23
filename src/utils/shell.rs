use crate::utils::errors::{ModeError, Result};
use std::env;
use std::path::PathBuf;

/// Supported shell types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellType {
    Bash,
    Zsh,
}

impl ShellType {
    /// Returns the name of the shell
    pub fn name(&self) -> &'static str {
        match self {
            ShellType::Bash => "bash",
            ShellType::Zsh => "zsh",
        }
    }

    /// Returns the RC file name for this shell
    pub fn rc_file_name(&self) -> &'static str {
        match self {
            ShellType::Bash => ".bashrc",
            ShellType::Zsh => ".zshrc",
        }
    }
}

/// Detects the user's current shell
///
/// Checks the SHELL environment variable to determine the shell type
pub fn detect_shell() -> Result<ShellType> {
    let shell_path = env::var("SHELL")
        .map_err(|_| ModeError::ShellDetection("SHELL environment variable not set".to_string()))?;

    if shell_path.contains("bash") {
        Ok(ShellType::Bash)
    } else if shell_path.contains("zsh") {
        Ok(ShellType::Zsh)
    } else {
        Err(ModeError::ShellDetection(format!(
            "Unsupported shell: {}. Only bash and zsh are supported.",
            shell_path
        )))
    }
}

/// Gets the path to the RC file for the given shell type
///
/// Returns the full path to the shell's RC file (e.g., ~/.bashrc or ~/.zshrc)
pub fn get_rc_file_path(shell: ShellType) -> Result<PathBuf> {
    let home = env::var("HOME")
        .map_err(|_| ModeError::ShellDetection("HOME environment variable not set".to_string()))?;

    let rc_path = PathBuf::from(home).join(shell.rc_file_name());

    // Check if file exists
    if !rc_path.exists() {
        return Err(ModeError::RcFileNotFound(format!(
            "{} does not exist",
            rc_path.display()
        )));
    }

    // Check if file is writable
    if rc_path.metadata()
        .map(|m| m.permissions().readonly())
        .unwrap_or(true)
    {
        return Err(ModeError::RcFileNotWritable(format!(
            "{} is not writable",
            rc_path.display()
        )));
    }

    Ok(rc_path)
}

/// Validates an alias name
///
/// Alias names must:
/// - Not be empty
/// - Contain only alphanumeric characters and underscores
/// - Not start with a digit
/// - Not be a shell reserved keyword
pub fn validate_alias_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(ModeError::InvalidAliasName(
            "Alias name cannot be empty".to_string(),
        ));
    }

    if name.chars().nth(0).unwrap().is_ascii_digit() {
        return Err(ModeError::InvalidAliasName(
            "Alias name cannot start with a digit".to_string(),
        ));
    }

    if !name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_')
    {
        return Err(ModeError::InvalidAliasName(
            "Alias name can only contain letters, numbers, and underscores".to_string(),
        ));
    }

    // Check for reserved keywords
    let reserved = vec![
        "if", "then", "else", "elif", "fi", "case", "esac", "for", "select", "while",
        "until", "do", "done", "in", "function", "time", "!", "[[", "]]", "{", "}",
    ];

    if reserved.contains(&name) {
        return Err(ModeError::InvalidAliasName(format!(
            "'{}' is a shell reserved keyword",
            name
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_alias_name() {
        // Valid names
        assert!(validate_alias_name("ll").is_ok());
        assert!(validate_alias_name("my_alias").is_ok());
        assert!(validate_alias_name("alias123").is_ok());

        // Invalid names
        assert!(validate_alias_name("").is_err());
        assert!(validate_alias_name("123alias").is_err());
        assert!(validate_alias_name("my-alias").is_err());
        assert!(validate_alias_name("my alias").is_err());
        assert!(validate_alias_name("if").is_err());
    }

    #[test]
    fn test_shell_type() {
        assert_eq!(ShellType::Bash.name(), "bash");
        assert_eq!(ShellType::Zsh.name(), "zsh");
        assert_eq!(ShellType::Bash.rc_file_name(), ".bashrc");
        assert_eq!(ShellType::Zsh.rc_file_name(), ".zshrc");
    }
}
