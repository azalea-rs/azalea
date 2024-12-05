use std::env;
use std::process::Command;

fn main() {
    // If using `rustup`, check the toolchain via `RUSTUP_TOOLCHAIN`
    if let Ok(toolchain) = env::var("RUSTUP_TOOLCHAIN") {
        if toolchain.contains("nightly") {
            return;
        } else {
            panic!("Azalea currently requires nightly Rust. You can run `rustup override set nightly` to set the toolchain for this directory.");
        }
    }

    // Get the path to the Rust compiler, defaulting to `rustc`
    let rustc_path = env::var("RUSTC")
        .or_else(|_| env::var("CARGO_BUILD_RUSTC"))
        .unwrap_or(String::from("rustc"));

    // Run `rustc -V` to check the toolchain version
    let rustc_command = Command::new(&rustc_path).arg("-V").output().unwrap();

    if rustc_command.status.success() {
        let rustc_output = String::from_utf8(rustc_command.stdout).unwrap();
        if !rustc_output.contains("nightly") {
            panic!("Azalea currently requires nightly Rust. Please check the documentation for your installation method and ensure you are using the nightly toolchain.");
        }
    } else {
        let rustc_output = String::from_utf8(rustc_command.stderr).unwrap();
        panic!("Failed to run `{rustc_path} -V` to check the toolchain version, {rustc_output}");
    }
}
