# `azalea-brigadier`

A Rust port of Mojang's [Brigadier](https://github.com/Mojang/brigadier) command parsing and dispatching library.

# Examples

```rust
use azalea_brigadier::prelude::*;
use std::sync::Arc;

#[derive(Debug, PartialEq)]
struct CommandSource {}

let mut subject = CommandDispatcher::new();
subject.register(literal("foo").executes(|_| 42));

assert_eq!(
    subject
        .execute("foo", Arc::new(CommandSource {}))
        .unwrap(),
    42
);
```

See the [tests](https://github.com/azalea-rs/azalea/tree/main/azalea-brigadier/tests) for more.

