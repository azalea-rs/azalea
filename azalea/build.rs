use std::env;

fn main() {
    let rust_toolchain = env::var("RUSTUP_TOOLCHAIN").unwrap();
    if rust_toolchain.starts_with("stable") {
        panic!("Azalea currently requires nightly Rust. You can use `rustup override set nightly` to set the toolchain for this directory.");
    }
}
