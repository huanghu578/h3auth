//! Path utilities.

use std::path::PathBuf;

pub struct Paths;

impl Paths {
    /// Directory containing the running executable.
    pub fn exe_dir() -> PathBuf {
        std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|d| d.to_path_buf()))
            .unwrap_or_else(|| PathBuf::from("."))
    }

    /// Path to `name` in the executable's directory.
    pub fn beside_exe(name: &str) -> PathBuf {
        Self::exe_dir().join(name)
    }

    /// File name without directory or extension.
    pub fn file_stem(path: &str) -> String {
        std::path::Path::new(path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or(path)
            .to_string()
    }

    /// Final path component, without directory.
    pub fn folder_name(path: &str) -> String {
        std::path::Path::new(path)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or(path)
            .to_string()
    }
}