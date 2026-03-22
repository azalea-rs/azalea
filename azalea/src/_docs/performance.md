# Azalea performance guide

Azalea is designed to have a reasonable trade-off between performance and ergonomics.
In some cases, performance is left on the table in exchange for simpler or more flexible interfaces.
This guide is for those who want to learn more about Azalea's performance characteristics, or for those who find that Azalea's default performance does not meet their needs.

Typically, one Azalea bot with 8 chunk render distance (the default) will idle on about 20mb-40mb of memory on a normal world.
If you're using a swarm, the world information (chunks, entities, registries) is shared between bots.
This means that each new bot in a swarm should be relatively cheap -- usually up to 1mb extra per bot -- though it may be more if they're spread apart.

As for CPU usage, Azalea is meant to be able to run on weak servers, but it may struggle at large swarm sizes, especially if you're using the pathfinder.
By default, swarms should be able to handle up to a few hundred bots, depending on your hardware.
Azalea uses threads whenever it's beneficial, so performance should also scale with your number of CPU cores.

<!-- note: we use higher headers here because otherwise they wouldn't all show up in the sidebar -->

# Improved performance in debug mode

This one is already mentioned in the `azalea` crate documentation, but is repeated here for completeness.

As you likely know, Rust programs perform significantly better when run in release mode (like, `cargo run --release`), but this comes at the cost of slower compile times.
To have nearly the same performance as release mode while having similar incremental compile times as debug mode, you can ask Rust to compile only your dependencies as if they were in release mode.
To do this, add the following lines to your `Cargo.toml`:

```toml
[profile.dev]
opt-level = 1
[profile.dev.package."*"]
opt-level = 3
```

For maximum performance, it is still recommended to compile your bot with release mode when you don't need a short feedback loop.

# General optimizations

## Compilation options

An easy win is to enable LTO (not thin LTO) by putting the following lines in your `Cargo.toml`:
```toml
[profile.release]
lto = true
```
In certain cases (i.e. pathfinding) this can make Azalea about 20% faster, at the cost of substantially slower compile times.

There are a few other options you can try setting, like `RUSTFLAGS="-C target-cpu=native"` and running with `panic = "abort"`, but these usually won't have a significant impact.

If you're willing to go through the trouble, PGO is also usually another 10% win.
[`cargo-pgo`](https://github.com/Kobzol/cargo-pgo) streamlines the process of creating PGO builds.

## Using a different allocator

Using [mimalloc v3](https://docs.rs/mimalloc/latest/mimalloc/) or [snmalloc](https://docs.rs/snmalloc-rs/latest/snmalloc_rs/) as your allocator is almost always another easy performance win.
Note that using these may increase your total memory usage.

# Some advice

If you're not profiling and benchmarking, then you're going in blind.
This means that you may inadvertently make your code slower, and you may waste time and miss potentially big optimization opportunities.

Benchmarking just means having a way to measure speedups.
For instance, this can be a simple timer or an averaged CPU usage measurement.

Profiling is to help you identify the slow parts of your code.
The recommended tool for this is [`cargo-flamegraph`](https://github.com/flamegraph-rs/flamegraph), just make sure to enable `force-frame-pointers` and `debuginfo`.

Some specific suggestions will not be mentioned in this guide because they're already mentioned in the relevant Azalea documentation.

I would also recommend reading Nethercote's [Rust Performance Book](https://nnethercote.github.io/perf-book/title-page.html) for more performance tips.

# Azalea-specific optimizations

## Update Azalea

Azalea (and its dependencies) are often updated with new performance improvements.
If you're on an old version of Azalea, then your bot may become slightly faster after updating or by switching to the unstable Git version.

## Disabling packet events

Azalea clones received packets to emit an `Event::Packet` for every packet that every client receives, but this can be wasteful if you're not actually using that event.
To avoid this cost, disable default features for Azalea and then enable the default ones except for `packet-event`.
This can be done with the following command:
```sh
cargo add azalea --no-default-features --features=log,online-mode,serde
```

If you want to disable `packet-event` but still have a way to watch for packets, you can do this by making a plugin and watching for ECS events such as `ReceiveGamePacketEvent`.

## Lower your render distance

If the client doesn't need a high view distance, then you can reduce the number of stored chunks/entities and received packets by lowering it.
This can be done by having something like the following in your handler function:
```rust,no_run
# use azalea::ClientInformation;
# async fn handle(bot: azalea::Client, event: azalea::Event, state: azalea::NoState) {
#    match event {
azalea::Event::Init => bot.set_client_information(ClientInformation {
    view_distance: 2,
    ..Default::default()
}),
# _ => {}
# }
# }
```

## Disabling plugins

This can be somewhat risky and isn't technically officially supported, but disabling default Azalea plugins like `PhysicsPlugin` can help with performance if you don't need them.
Look at the source code for `DefaultPlugins` and `DefaultBotPlugins` to find the full list of plugins that can be disabled.

If you want to go even further, forking Azalea and commenting out code within packet handlers or other places may be worthwhile (but will probably make things unstable and will make it a pain to update versions).

## Write your bot as a plugin

It's rare for user code to be a bottleneck, but if you do happen to be interacting with the bot a lot, then implementing those parts of your code as a Bevy plugin might be a good idea.
Doing this can help avoid unnecessary clones and locks, and it'll allow for your code to run in parallel with Azalea's internal systems.

Note that while it's somewhat possible to access a `Client` from within a plugin (which also requires creating a new thread or Tokio task), you should avoid this as it will negate most of the performance benefits that come from using a plugin.
Looking at the source code for how things are implemented in `Client` is a good idea, though.

## Don't use Azalea

At a certain point, it may be worth considering if you even need a bot library.
If Azalea's event loop is still too slow, and if your bot could be implemented by manually reading and writing packets, then perhaps you should think about using `azalea-protocol` directly.
There are some examples for how you can go about doing this in the documentation for that crate.
Hopefully, though, Azalea's performance will be good enough and you won't need to do this.
