pub mod errors;
pub mod file_ops;
pub mod shell;

pub use errors::{ModeError, Result};
pub use shell::{detect_shell, get_rc_file_path, validate_alias_name, ShellType};
