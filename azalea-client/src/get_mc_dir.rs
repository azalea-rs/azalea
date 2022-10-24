//! Find out where the user's .minecraft directory is.
//!
//! Used for the auth cache.

use std::path::PathBuf;

/// Return the location of the user's .minecraft directory.
///
/// Windows: `%appdata%\.minecraft`\
/// Mac: `$HOME/Library/Application Support/minecraft`\
/// Linux: `$HOME/.minecraft`
///
/// Anywhere else it'll return None.
pub fn minecraft_dir() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        let appdata = std::env::var("APPDATA").ok()?;
        Some(PathBuf::from(appdata).join(".minecraft"))
    }
    #[cfg(target_os = "macos")]
    {
        let home = std::env::var("HOME").ok()?;
        Some(PathBuf::from(home).join("Library/Application Support/minecraft"))
    }
    #[cfg(target_os = "linux")]
    {
        let home = std::env::var("HOME").ok()?;
        Some(PathBuf::from(home).join(".minecraft"))
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        None
    }
}
