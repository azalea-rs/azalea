Azalea is a framework for creating Minecraft bots.

Internally, it's just a wrapper over [`azalea_client`], adding useful
functions for making bots. Because of this, lots of the documentation will
refer to `azalea_client`. You can just replace these with `azalea` in your
code, since everything from azalea_client is re-exported in azalea.

# Installation

First, install Rust nightly with `rustup install nightly` and `rustup
default nightly`.

Then, add one of the following lines to your Cargo.toml:

Latest bleeding-edge version:
`azalea = { git="https://github.com/mat-1/azalea" }`\
Latest "stable" release:
`azalea = "0.7.0"`

## Optimization

For faster compile times, make a `.cargo/config.toml` file in your project
and copy
[this file](https://github.com/mat-1/azalea/blob/main/.cargo/config.toml)
into it. You may have to install the LLD linker.

For faster performance in debug mode, add the following code to your
Cargo.toml:
```toml
[profile.dev]
opt-level = 1
[profile.dev.package."*"]
opt-level = 3
```

# Examples

```rust,no_run
//! A bot that logs chat messages sent in the server to the console.

use azalea::prelude::*;
use parking_lot::Mutex;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let account = Account::offline("bot");
    // or Account::microsoft("example@example.com").await.unwrap();

    loop {
        let e = ClientBuilder::new()
            .set_handler(handle)
            .start(account.clone(), "localhost")
            .await;
        eprintln!("{:?}", e);
    }
}

#[derive(Default, Clone, Component)]
pub struct State {}

async fn handle(bot: Client, event: Event, state: State) -> anyhow::Result<()> {
    match event {
        Event::Chat(m) => {
            println!("{}", m.message().to_ansi());
        }
        _ => {}
    }

    Ok(())
}
```

# Swarms

Azalea lets you create "swarms", which are a group of bots in the same world that can perform actions together. See [testbot](https://github.com/mat-1/azalea/blob/main/azalea/examples/testbot.rs) for an example. Also, if you're using swarms, you should also have both `azalea::prelude::*` and `azalea::swarm::prelude::*`.

# Plugins

Azalea uses [Bevy ECS](https://docs.rs/bevy_ecs) internally to store information about the world and clients. Bevy plugins are more powerful than async handler functions, but more difficult to use. See [pathfinder](https://github.com/mat-1/azalea/blob/main/azalea/src/pathfinder/mod.rs) as an example of how to make a plugin. You can then enable a plugin by adding `.add_plugin(ExamplePlugin)` in your client/swarm builder.

Also note that just because something is an entity in the ECS doesn't mean that it's a Minecraft entity. You can filter for that by having `With<MinecraftEntityId>` as a filter.

See the [Bevy Cheatbook](https://bevy-cheatbook.github.io/programming/ecs-intro.html) to learn more about Bevy ECS (and the ECS paradigm in general).

# Debugging

Azalea uses several relatively complex features of Rust, which may make debugging certain issues more tricky if you're not familiar with them.

## Logging

One of the most useful tools for debugging issues is logging. The default log level is `info`, but you can make it show more or less information by changing the log level. Enabling logging is done with `RUST_LOG=debug cargo run` on Linux/bash or `set RUST_LOG=debug && cargo run` on Windows. The log levels are `trace`, `debug`, `info`, `warn`, and `error`, in ascending priority.

If it's a crash/panic and you believe it has to do with parsing a packet, you might want to set the level to `trace` since that'll make it show the first few hundred bytes of every packet received. This may produce a lot of logs, so pipe it into a file with `&> azalea.log` (on Linux).

Note: If you get a `SetLoggerError`, it's because you have multiple loggers. Azalea comes with a logger by default, see [`bevy_log`] for more information. You can disable the default logging plugin by disabling the `log` feature.

## Deadlocks

If your code is simply hanging, it might be a deadlock. Copy the deadlock block in [`azalea/examples/testbot.rs`](https://github.com/mat-1/azalea/blob/main/azalea/examples/testbot.rs) to the beginning of your code and it'll print a long backtrace if a deadlock is detected.

## Backtraces

Backtraces are also useful, though they're sometimes hard to read and don't always contain the actual location of the error. Run your code with `RUST_BACKTRACE=1` to enable full backtraces. If it's very long, often searching for the keyword "azalea" will help you filter out unrelated things and find the actual source of the issue.

[`azalea_client`]: https://docs.rs/azalea-client
[`bevy_log`]: https://docs.rs/bevy_log
