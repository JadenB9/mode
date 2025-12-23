use crate::utils::errors::{ModeError, Result};
use chrono::Local;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;

/// Creates a timestamped backup of the given file
///
/// Returns the path to the backup file
pub fn create_backup(file_path: &Path) -> Result<PathBuf> {
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let backup_path = file_path.with_extension(format!("backup.{}", timestamp));

    fs::copy(file_path, &backup_path).map_err(|e| {
        ModeError::BackupFailed(format!(
            "Failed to create backup at {}: {}",
            backup_path.display(),
            e
        ))
    })?;

    Ok(backup_path)
}

/// Checks if an alias already exists in the RC file
///
/// Returns true if the alias name is already defined
pub fn check_duplicate_alias(rc_file: &Path, alias_name: &str) -> Result<bool> {
    let content = fs::read_to_string(rc_file)?;

    // Look for lines like: alias name= or alias name='
    let alias_pattern = format!("alias {}=", alias_name);
    let alias_pattern_quoted = format!("alias {}", alias_name);

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with(&alias_pattern) || trimmed.starts_with(&alias_pattern_quoted) {
            return Ok(true);
        }
    }

    Ok(false)
}

/// Safely appends an alias to the RC file using atomic file operations
///
/// Steps:
/// 1. Create a backup of the RC file
/// 2. Read current content
/// 3. Create a temporary file
/// 4. Write original content + new alias to temp file
/// 5. Atomically rename temp file to original
pub fn append_alias(rc_file: &Path, alias_name: &str, command: &str) -> Result<PathBuf> {
    // Create backup first
    let backup_path = create_backup(rc_file)?;

    // Read current content
    let mut current_content = String::new();
    fs::File::open(rc_file)?.read_to_string(&mut current_content)?;

    // Ensure file ends with newline
    if !current_content.is_empty() && !current_content.ends_with('\n') {
        current_content.push('\n');
    }

    // Create the alias entry
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    let alias_entry = format!(
        "\n# Added by mode on {}\nalias {}='{}'\n",
        timestamp, alias_name, command
    );

    // Create temp file in the same directory for atomic rename
    let rc_dir = rc_file.parent().ok_or_else(|| {
        ModeError::FileOperation("Could not determine RC file directory".to_string())
    })?;

    let mut temp_file = NamedTempFile::new_in(rc_dir).map_err(|e| {
        ModeError::FileOperation(format!("Failed to create temporary file: {}", e))
    })?;

    // Write combined content to temp file
    temp_file
        .write_all(current_content.as_bytes())
        .map_err(|e| ModeError::FileOperation(format!("Failed to write to temp file: {}", e)))?;

    temp_file
        .write_all(alias_entry.as_bytes())
        .map_err(|e| ModeError::FileOperation(format!("Failed to write alias to temp file: {}", e)))?;

    // Sync to disk
    temp_file
        .flush()
        .map_err(|e| ModeError::FileOperation(format!("Failed to flush temp file: {}", e)))?;

    // Atomically replace the original file
    temp_file.persist(rc_file).map_err(|e| {
        ModeError::FileOperation(format!(
            "Failed to persist temp file to {}: {}",
            rc_file.display(),
            e
        ))
    })?;

    Ok(backup_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_create_backup() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");

        // Create test file
        let mut file = fs::File::create(&file_path).unwrap();
        file.write_all(b"test content").unwrap();

        // Create backup
        let backup_path = create_backup(&file_path).unwrap();

        // Verify backup exists and has same content
        assert!(backup_path.exists());
        let backup_content = fs::read_to_string(&backup_path).unwrap();
        assert_eq!(backup_content, "test content");
    }

    #[test]
    fn test_check_duplicate_alias() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join(".bashrc");

        // Create test RC file with existing alias
        let mut file = fs::File::create(&file_path).unwrap();
        file.write_all(b"alias ll='ls -la'\nalias gs='git status'\n")
            .unwrap();

        // Check for duplicates
        assert!(check_duplicate_alias(&file_path, "ll").unwrap());
        assert!(check_duplicate_alias(&file_path, "gs").unwrap());
        assert!(!check_duplicate_alias(&file_path, "gp").unwrap());
    }

    #[test]
    fn test_append_alias() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join(".bashrc");

        // Create test RC file
        let mut file = fs::File::create(&file_path).unwrap();
        file.write_all(b"# My bashrc\nalias ll='ls -la'\n")
            .unwrap();
        drop(file);

        // Append new alias
        let backup_path = append_alias(&file_path, "gs", "git status").unwrap();

        // Verify backup was created
        assert!(backup_path.exists());

        // Verify new content
        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("alias gs='git status'"));
        assert!(content.contains("Added by mode"));
        assert!(content.contains("alias ll='ls -la'"));
    }
}
