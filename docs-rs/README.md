## Docs.rs Extensions

This directory includes some templates and styling to extend and modify [rustdoc]'s output
for Azalea's documentation on [docs.rs].

See [Bevy's documentation](https://github.com/bevyengine/bevy/tree/main/docs-rs) for more info.

## Local Testing

Build the documentation with the extension enabled like this:

```bash
RUSTDOCFLAGS="--html-after-content docs-rs/trait-tags.html --cfg docsrs_dep" RUSTFLAGS="--cfg docsrs_dep" cargo doc --no-deps --package <package_name>
```

[rustdoc]: https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html
[docs.rs]: https://docs.rs
