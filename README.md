# Azalea

[![docs.rs](https://img.shields.io/docsrs/azalea)](https://docs.rs/azalea)

A collection of Rust crates for making Minecraft bots, clients, and tools.

<p align="center">
    <img src="https://github.com/azalea-rs/azalea/assets/27899617/b98a42df-5cf0-4d1f-ae7c-ecca333e3cab" alt="Azalea" height="200">
</p>

<!-- The line below is automatically read and updated by the migrate script, so don't change it manually. -->

_Currently supported Minecraft version: `1.21.11`._

> [!WARNING]
> Many parts of Azalea are still unfinished and will receive breaking changes in the future. Most breaking changes will be listed in the [changelog](CHANGELOG.md).

## Features

- [Accurate physics](https://azalea.matdoes.dev/azalea_physics/) (but some features like entity pushing and elytras aren't implemented yet)
- [Pathfinder](https://azalea.matdoes.dev/azalea/pathfinder/index.html)
- [Swarms](https://azalea.matdoes.dev/azalea/swarm/index.html)
- [Breaking blocks](https://azalea.matdoes.dev/azalea/struct.Client.html#method.mine)
- [Block interactions & building](https://azalea.matdoes.dev/azalea/struct.Client.html#method.block_interact) (this doesn't predict the block interactions/placement on the client yet, but it's usually fine)
- [Inventories](https://azalea.matdoes.dev/azalea/struct.Client.html#impl-ContainerClientExt-for-Client)
- [Attacking entities](https://azalea.matdoes.dev/azalea/struct.Client.html#method.attack)
- [Plugins](#plugins)

## Goals

- Support everything that a vanilla Minecraft client can do.
- Have an intuitive and easy to use API.
- Make it easy to have many bots working at the same time.
- Don't trigger anti-cheats.
- Support the latest Minecraft version.
- Be fast and efficient.

## Non-goals

- Supporting multiple versions of Minecraft at the same time[\*](https://github.com/azalea-rs/azalea-viaversion).
- Graphics[\*](https://github.com/urisinger/azalea-graphics).
- Bedrock edition.

## Documentation

The stable documentation for the main `azalea` crate is available at [docs.rs/azalea](https://docs.rs/azalea), and the unstable documentation is at [azalea.matdoes.dev](https://azalea.matdoes.dev).

## Matrix/Discord

If you'd like to chat about Azalea, you can join the Matrix space at [#azalea:matdoes.dev](https://matrix.to/#/#azalea:matdoes.dev) or the Discord server at [discord.gg/FaRey6ytmC](https://discord.gg/FaRey6ytmC). The channels are bridged so you don't need to join both.

## Real-world bots using Azalea

Here's an incomplete list of bots built using Azalea, primarily intended as a reference in addition to the existing documentation and examples:

- [ShayBox/ShaysBot](https://github.com/ShayBox/ShaysBot) - Pearl stasis bot featuring a Discord bot, an HTTP API, and more.
- [EnderKill98/statis-bot](https://github.com/EnderKill98/stasis-bot) - This bot can automatically detect thrown pearls and later walk there and pull them for you.
- [as1100k/aether](https://github.com/as1100k/aether) - Collection of Minecraft bots and plugins.
- [mat-1/potato-bot-2](https://github.com/mat-1/potato-bot-2) - Hardened Discord chat bridge created for the LiveOverflow SMP.
- [ErrorNoInternet/ErrorNoWatcher](https://github.com/ErrorNoInternet/ErrorNoWatcher) - A Minecraft bot with Lua scripting support.

You can see more projects built with Azalea in the [GitHub dependency graph](https://github.com/azalea-rs/azalea/network/dependents).

## Plugins

Azalea has support for Bevy plugins which can significantly alter its functionality.
Here are some plugins that you may find useful:

- [azalea-rs/azalea-viaversion](https://github.com/azalea-rs/azalea-viaversion) - Multi-version compatibility for your Azalea bots using ViaProxy.
- [azalea-rs/azalea-hax](https://github.com/azalea-rs/azalea-hax) - Anti-knockback.

If you've created your own plugin for Azalea, please create a PR to add it to this list :).

## Funding

Azalea is currently maintained primarily by one person as a hobby project. If you appreciate Azalea, consider [donating on Ko-fi](https://ko-fi.com/matdoesdev).

