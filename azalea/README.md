Azalea is a framework for creating Minecraft bots.

See the project [README](https://github.com/azalea-rs/azalea) for a higher-level overview of Azalea.

# Examples

```rust,no_run
//! A bot that logs chat messages and the number that we've received to the console.

use std::sync::Arc;

use azalea::prelude::*;
use parking_lot::Mutex;

#[tokio::main]
async fn main() -> AppExit {
    let account = Account::offline("bot");
    // or Account::microsoft("example@example.com").await.unwrap();

    ClientBuilder::new()
        .set_handler(handle)
        .start(account, "localhost")
        .await
}

#[derive(Default, Clone, Component)]
pub struct State {
    /// An example field that stores the number of messages that've been
    /// received by the client so far.
    /// 
    /// The state gets cloned whenever the handler is called, so to have all
    /// the clones point to the same data and have it be mutable, we use an
    /// Arc<Mutex<T>>.
    pub messages_received: Arc<Mutex<usize>>
}

async fn handle(bot: Client, event: Event, state: State) -> anyhow::Result<()> {
    match event {
        Event::Chat(m) => {
            let mut messages_received = state.messages_received.lock();
            *messages_received += 1;
            println!("#{messages_received}: {}", m.message().to_ansi());
            // messages_received gets implicitly unlocked here because it's dropped
        }
        _ => {}
    }

    Ok(())
}
```

There are more examples in the [examples directory](https://github.com/azalea-rs/azalea/tree/main/azalea/examples).
You may also find it helpful to read the code for [other people's Azalea bots](https://github.com/azalea-rs/azalea#real-world-bots-using-azalea).

# Installation

First, install Rust nightly with `rustup install nightly` and `rustup default nightly`.

Then, use one of the following commands to add Azalea to your project:

-   Latest bleeding-edge version (recommended): `cargo add azalea --git=https://github.com/azalea-rs/azalea`
-   Latest "stable" release: `cargo add azalea`

## Optimization

For faster compile times, create a `.cargo/config.toml` file in your project and copy
[this file](https://github.com/azalea-rs/azalea/blob/main/.cargo/config_fast_builds.toml)
into it. You may have to install the LLD linker.

For faster performance in debug mode, add the following code to your
Cargo.toml:

```toml
[profile.dev]
opt-level = 1
[profile.dev.package."*"]
opt-level = 3
```

# Documentation

The documentation for the latest Azalea crates.io release is available at [docs.rs/azalea](https://docs.rs/azalea/latest/azalea/) and the docs for the latest bleeding-edge (git) version are at [azalea.matdoes.dev](https://azalea.matdoes.dev/azalea/).

# Swarms

Azalea lets you create "swarms", which are a group of bots in the same world that can perform actions together. See [testbot](https://github.com/azalea-rs/azalea/blob/main/azalea/examples/testbot/main.rs) for an example. Also, if you're using swarms, you should also `use` both `azalea::prelude::*` and `azalea::swarm::prelude::*`.

# Plugins

Azalea uses [Bevy ECS](https://docs.rs/bevy_ecs) internally to store information about the world and clients. Bevy plugins are more powerful than async handler functions, but more difficult to use. See [pathfinder](https://github.com/azalea-rs/azalea/blob/main/azalea/src/pathfinder/mod.rs) as an example of how to make a plugin. You can then enable a plugin by adding `.add_plugin(ExamplePlugin)` in your client/swarm builder.

Everything inside of Azalea is implemented as a Bevy plugin, which means you can disable default behaviors (like, physics or chat signing) by disabling built-in plugins. See [`SwarmBuilder::new_without_plugins`](swarm::SwarmBuilder::new_without_plugins) to learn how to do that.

Also note that just because something is an entity in the ECS doesn't mean that it's a Minecraft entity. You can filter for that by having `With<MinecraftEntityId>` as a filter.

See the [Bevy Cheatbook](https://bevy-cheatbook.github.io/programming/ecs-intro.html) to learn more about Bevy ECS (and the ECS paradigm in general).

# Debugging

Azalea uses several relatively complex features of Rust, which may make debugging certain issues more tricky if you're not familiar with them.

## Logging

One of the most useful tools for debugging issues is logging. The default log level is `info`, but you can make it show more or less information by changing the log level. Enabling logging is done with `RUST_LOG=debug cargo run` on Linux/bash or `set RUST_LOG=debug && cargo run` on Windows. The log levels are `trace`, `debug`, `info`, `warn`, and `error`, in ascending priority.

If it's a crash/panic and you believe it has to do with parsing a packet, you might want to set the level to `trace` since that'll make it show the first few hundred bytes of every packet received. This may produce a lot of logs, so use a command like `RUST_LOG=trace NO_COLOR=1 cargo run &> azalea.log` to nicely pipe it into a file (on Linux).

Note: If you get a `SetLoggerError`, it's because you have multiple loggers. Azalea comes with a logger by default, see [`bevy_log`] for more information. You can disable the default logging plugin by disabling the `log` feature.

## Deadlocks

If your code is simply hanging, it might be a deadlock. Enable `parking_lot`'s `deadlock_detection` feature and copy the deadlock block in [`azalea/examples/testbot.rs`](https://github.com/azalea-rs/azalea/blob/main/azalea/examples/testbot/main.rs) to the beginning of your code and it'll print a long backtrace if a deadlock is detected.

## Backtraces

Backtraces are also useful, though they're sometimes hard to read and don't always contain the actual location of the error. Run your code with `RUST_BACKTRACE=1` to enable full backtraces. If it's very long, often searching for the keyword "azalea" will help you filter out unrelated things and find the actual source of the issue.

# Other notes

## Using `tokio::task::spawn_local` instead of `tokio::spawn`

If you spawn a task with `tokio::spawn` and move your bot into it, it's possible for Tokio to run the handler function or schedule a Minecraft tick at an unexpected moment. For instance, `bot.component::<TicksConnected>() == bot.component::<TicksConnected>()` is not guaranteed to be true inside of a `tokio::spawn`. Azalea already mitigates this in the handler function by using a Tokio [LocalSet](https://docs.rs/tokio/latest/tokio/task/struct.LocalSet.html), but that mitigation does not apply if you call `tokio::spawn` yourself. To avoid this, you must call `tokio::task::spawn_local` in place of `tokio::spawn`. Alternatively, you could also mark your main function with `#[tokio::main(flavor = "current_thread")]`.

## Disabling log messages

You can disable all console messages by setting the `RUST_LOG` environment variable to `off`, or you can filter log messages by setting specific log levels. For example, to disable only pathfinding logs, you can set `RUST_LOG=azalea::pathfinder=off`.

See the [`env_logger`](https://docs.rs/env_logger/latest/env_logger/) crate documentation for more information.

[`bevy_log`]: https://docs.rs/bevy_log

