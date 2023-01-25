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
`azalea = "0.5.0"`

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
A bot that logs chat messages sent in the server to the console.

use azalea::prelude::*;
use parking_lot::Mutex;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let account = Account::offline("bot");
    // or Account::microsoft("example@example.com").await.unwrap();

    azalea::start(azalea::Options {
        account,
        address: "localhost",
        state: State::default(),
        plugins: plugins![],
        handle,
    })
    .await
    .unwrap();
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

# Plugins

Azalea uses [Bevy ECS](https://docs.rs/bevy_ecs) internally to store information about the world and clients. Bevy plugins are more powerful than async handler functions, but more difficult to use. See [pathfinder](azalea/src/pathfinder/mod.rs) as an example of how to make a plugin. You can then use a plugin by adding `.add_plugin(ExamplePlugin)` in the client or swarm builder.

Also note that just because something is an entity in the ECS doesn't mean that it's a Minecraft entity. You can filter for that by having `With<MinecraftEntityId>` as a filter.

See the [https://bevy-cheatbook.github.io/programming/ecs-intro.html](Bevy Cheatbook) to learn more about Bevy ECS (and ECS in general).

[`azalea_client`]: https://docs.rs/azalea-client