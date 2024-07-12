use std::process::Command;

fn main() {
    let rustc_version_output = Command::new("rustc").arg("-V").output().unwrap();
    if !rustc_version_output.status.success()
        || !String::from_utf8(rustc_version_output.stdout)
            .unwrap()
            .contains("nightly")
    {
        panic!("Azalea currently requires nightly Rust. If you have installed Rust with rustup you can use `rustup override set nightly` to set the toolchain for this directory.");
    }
}
