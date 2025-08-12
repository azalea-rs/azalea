# Azalea

[![docs.rs](https://img.shields.io/docsrs/azalea)](https://docs.rs/azalea)

A collection of Rust crates for making Minecraft bots, clients, and tools.

<p align="center">
    <img src="https://github.com/azalea-rs/azalea/assets/27899617/b98a42df-5cf0-4d1f-ae7c-ecca333e3cab" alt="Azalea" height="200">
</p>

<!-- The line below is automatically read and updated by the migrate script, so don't change it manually. -->

_Currently supported Minecraft version: `25w33a`._

> [!WARNING]
> Azalea is still unfinished, though most crates are in a useable state.

## Features

-   [Accurate physics](https://github.com/azalea-rs/azalea/blob/main/azalea-physics/src/lib.rs) (but some features like entity pushing and elytras aren't implemented yet)
-   [Pathfinder](https://azalea.matdoes.dev/azalea/pathfinder/index.html)
-   [Swarms](https://azalea.matdoes.dev/azalea/swarm/index.html)
-   [Breaking blocks](https://azalea.matdoes.dev/azalea/struct.Client.html#method.mine)
-   [Block interactions & building](https://azalea.matdoes.dev/azalea/struct.Client.html#method.block_interact) (this doesn't predict the block interactions/placement on the client yet, but it's usually fine)
-   [Inventories](https://azalea.matdoes.dev/azalea/struct.Client.html#impl-ContainerClientExt-for-Client)
-   [Attacking entities](https://azalea.matdoes.dev/azalea/struct.Client.html#method.attack)
-   [Plugins](#plugins)

## Docs

The "stable" documentation is available at [docs.rs/azalea](https://docs.rs/azalea) and the unstable docs are at [azalea.matdoes.dev](https://azalea.matdoes.dev) (recommended).

## Matrix/Discord

If you'd like to chat about Azalea, you can join the Matrix space at [#azalea:matdoes.dev](https://matrix.to/#/#azalea:matdoes.dev) (recommended) or the Discord server at [discord.gg/FaRey6ytmC](https://discord.gg/FaRey6ytmC) (they're bridged so you don't need to join both).

## Goals

-   Do everything a vanilla client can do.
-   Be intuitive and easy to use.
-   Make it easy to have many bots working at the same time.
-   Don't trigger anticheats.
-   Support the latest Minecraft version.
-   Be fast and memory efficient.

## Non-goals

-   Supporting several versions of Minecraft on the same branch[\*](https://github.com/azalea-rs/azalea-viaversion).
-   Bedrock edition.
-   Graphics.

## Real-world bots using Azalea

Here's an incomplete list of bots built using Azalea, primarily intended as a reference in addition to the existing documentation and examples:

-   [ShayBox/ShaysBot](https://github.com/ShayBox/ShaysBot) - Pearl stasis bot featuring a Discord bot, an HTTP API, and more.
-   [EnderKill98/statis-bot](https://github.com/EnderKill98/stasis-bot) - This bot can automatically detect thrown pearls and later walk there and pull them for you.
-   [as1100k/aether](https://github.com/as1100k/aether) - Collection of Minecraft bots and plugins.
-   [mat-1/potato-bot-2](https://github.com/mat-1/potato-bot-2) - Hardened Discord chat bridge created for the LiveOverflow SMP.
-   [ErrorNoInternet/ErrorNoWatcher](https://github.com/ErrorNoInternet/ErrorNoWatcher) - A Minecraft bot with Lua scripting support.

You can see more projects built with Azalea in the [GitHub dependency graph](https://github.com/azalea-rs/azalea/network/dependents).

## Plugins

Azalea has support for Bevy plugins, which can significantly alter its functionality. Here are some plugins that you may find useful:

-   [azalea-rs/azalea-viaversion](https://github.com/azalea-rs/azalea-viaversion) - Multi-version compatibility for your Azalea bots using ViaProxy.
-   [azalea-rs/azalea-hax](https://github.com/azalea-rs/azalea-hax) - Anti-knockback.

If you've created your own plugin for Azalea, please create a PR to add it to this list :).

## FAQ

-   There's too many console messages, how do I disable them?

    You can disable all console messages by setting the `RUST_LOG` environment variable to `off`, or you can filter log messages by setting specific log levels. For example, to disable only pathfinding logs, you can set `RUST_LOG=azalea::pathfinder=off`.
    
    See the [`env_logger`](https://docs.rs/env_logger/latest/env_logger/) crate documentation for more information.
