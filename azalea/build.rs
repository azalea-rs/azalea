use std::env;
use std::process::Command;

fn main() {
    match env::var("RUSTUP_TOOLCHAIN") {
        Ok(rust_toolchain) if !rust_toolchain.starts_with("nightly") => {
            // stable & beta
            panic!("Azalea currently requires nightly Rust. You can use `rustup override set nightly` to set the toolchain for this directory.");
        }
        Ok(_) => return, // nightly
        Err(_) => {
            // probably not installed via rustup, run rustc and parse its output
            let rustc_command = env::var("RUSTC")
                .or_else(|_| env::var("CARGO_BUILD_RUSTC"))
                .unwrap_or(String::from("rustc"));
            let rustc_version_output = Command::new(rustc_command).arg("-V").output().unwrap();
            if !rustc_version_output.status.success()
                || !String::from_utf8(rustc_version_output.stdout)
                    .unwrap()
                    .contains("nightly")
            {
                panic!("Azalea currently requires nightly Rust. It seems that you did not install Rust via rustup. Please check the documentation for your installation method, to find out how to use nightly Rust.");
            }
        }
    }
}
