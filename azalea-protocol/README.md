# Azalea Protocol

Sent and receive Minecraft packets. You should probably use `azalea` or `azalea-client` instead.

The goal is to only support the latest Minecraft version in order to ease development.

This is not yet complete, search for `TODO` in the code for things that need to be done.

Unfortunately, using azalea-protocol requires Rust nightly because [specialization](https://github.com/rust-lang/rust/issues/31844) is not stable yet. Use `rustup default nightly` to enable it.
