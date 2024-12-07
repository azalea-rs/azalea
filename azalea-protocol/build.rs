use std::env;
use std::process::Command;

/// The maximum recommended toolchain version, as a triple.
const TOOLCHAIN_MAX: (u32, u32, u32) = (2024, 11, 11);

fn main() {
    if let Some(toolchain) = toolchain_version() {
        // If the toolchain is not nightly, do nothing
        if !toolchain.contains("nightly") {
            return;
        }

        // Warn if the toolchain may cause issues
        if !recommended_toolchain(&toolchain).unwrap_or_default() {
            println!("cargo::warning=The current Rust version may cause issues, try using: \"nightly-{}-{}-{}\"", TOOLCHAIN_MAX.0, TOOLCHAIN_MAX.1, TOOLCHAIN_MAX.2);
        }
    }
}

/// Attempt to get the current toolchain version
fn toolchain_version() -> Option<String> {
    // Use the `RUSTUP_TOOLCHAIN` environment variable
    if let Ok(toolchain) = env::var("RUSTUP_TOOLCHAIN") {
        return Some(toolchain);
    }

    // Fallback to running `rustc -V`
    let rustc_path = env::var("RUSTC")
        .or_else(|_| env::var("CARGO_BUILD_RUSTC"))
        .unwrap_or(String::from("rustc"));

    let rustc_command = Command::new(&rustc_path).arg("-V").output().unwrap();
    if rustc_command.status.success() {
        String::from_utf8(rustc_command.stdout).ok()
    } else {
        None
    }
}


/// Attempt to parse the version of the toolchain,
/// returning `Some(true)` if the toolchain should be fine.
fn recommended_toolchain(toolchain: &str) -> Option<bool> {
    let mut split = toolchain.split('-');
    while split.next() != Some("nightly") {}

    let year = split.next()?.parse().ok()?;
    let month = split.next()?.parse().ok()?;
    let day = split.next()?.parse().ok()?;

    Some((year, month, day) <= TOOLCHAIN_MAX)
}
