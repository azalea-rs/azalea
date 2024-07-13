use std::env;
use std::process::Command;

fn main() {
    let rustc_command = env::var("RUSTC")
        .or_else(|_| env::var("CARGO_BUILD_RUSTC"))
        .unwrap_or(String::from("rustc"));
    let rustc_version_output = Command::new(rustc_command).arg("-V").output().unwrap();
    if !rustc_version_output.status.success()
        || !String::from_utf8(rustc_version_output.stdout)
            .unwrap()
            .contains("nightly")
    {
        panic!("Azalea currently requires nightly Rust. If you have installed Rust with rustup you can use `rustup override set nightly` to set the toolchain for this directory.");
    }
}
