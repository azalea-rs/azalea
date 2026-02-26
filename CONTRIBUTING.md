Thanks for your interest in contributing to Azalea!

This document is a work-in-progress.

## How you can help

First, you're advised to join the Matrix space at [#azalea:matdoes.dev](https://matrix.to/#/#azalea:matdoes.dev) or the Discord server at [discord.gg/FaRey6ytmC](https://discord.gg/FaRey6ytmC) (bridged, you don't need to join both). If you're unsure of how to do something related to Azalea, you should go and ask there.

If there's a feature you want, implement it! You should search the GitHub issues first to make sure that it's not already being worked on, though.

After you're done, please submit a PR. It'll be reviewed/merged when the maintainer has time.

Note that not all features are within scope for Azalea, but many features can still be implemented as a plugin which you may publish separately (and which may be advertised in Azalea's README).

## Things to watch out for

If you're working with low-level physics or packet related code, it's quite easy to accidentally make a change that causes Azalea to start flagging anticheats. If you're unsure about a change, you're advised to reference the decompiled vanilla Minecraft source code and test with an anticheat such as [GrimAC](https://modrinth.com/plugin/grimac) (Grim is preferred as it's relatively strict and is used on some popular anarchy servers).

The second major thing to watch out for is accidentally introducing performance regressions. Certain parts of Azalea are highly performance sensitive (notably, the pathfinder), so most changes in these areas should be benchmarked to avoid accidentally hurting performance.

You're encouraged to write relevant tests and benchmarks.

## Profiling

Please see [the chapter about profiling in the Rust performance book](https://nnethercote.github.io/perf-book/profiling.html).

The usual profiling setup is something like this:

```sh
cargo install flamegraph
RUSTFLAGS="-C force-frame-pointers=yes" cargo r -r --example testbot
# wait a few seconds so chunks being loaded doesn't affect the flamegraph, and
# then run this in a separate window:
flamegraph -p $(pidof testbot) --deterministic
# wait about 15 seconds, then ctrl+c, and view the flamegraph.svg
```

## AI Policy

Please avoid using generative AI to make contributions to Azalea. We do not enjoy working with code that wasn't written by people.
