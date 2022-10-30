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
/// If the environment variable is not set, this will return `None`.
pub fn minecraft_dir() -> Option<PathBuf> {
    let env_var = home_env_var();
    let home = std::env::var(env_var).ok()?;
    let path = PathBuf::from(home).join(minecraft_dir_relative());
    Some(path)
}

/// Return the name of the environment variable that's used for the home folder
/// on the user's operating system.
pub fn home_env_var() -> &'static str {
    #[cfg(target_os = "windows")]
    {
        "USERPROFILE"
    }
    #[cfg(target_os = "macos")]
    {
        "HOME"
    }
    #[cfg(target_os = "linux")]
    {
        "HOME"
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        "HOME"
    }
}

/// Return the path relative to the home folder where we expect to find the
/// .minecraft directory.
pub fn minecraft_dir_relative() -> &'static str {
    #[cfg(target_os = "windows")]
    {
        ".minecraft"
    }
    #[cfg(target_os = "macos")]
    {
        "Library/Application Support/minecraft"
    }
    #[cfg(target_os = "linux")]
    {
        ".minecraft"
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        ".minecraft"
    }
}
