# Azalea Protocol

Sent and receive Minecraft packets. You should probably use `azalea` or `azalea-client` instead.

The goal is to **only** support the latest Minecraft version in order to ease development.

This is not yet complete, search for `TODO` in the code for things that need to be done.

Unfortunately, compiling the crate requires Rust nightly because specialization is not stable yet.

## Adding a new packet

Adding new packets is usually pretty easy, but you'll want to have Minecraft's decompiled source code which you can obtain with tools such as [DecompilerMC](https://github.com/hube12/DecompilerMC).

1. Find the packet in Minecraft's source code. Minecraft's packets are in the `net/minecraft/network/protocol/<state>` directory. The state for your packet is usually `game`.
2. Add a new file in the [`packets/<state>`](./src/packets/game) directory with the snake_cased version of the name Minecraft uses.
3. Copy the code from a similar packet and change the struct name.
4. Add the fields from Minecraft's source code from either the read or write methods.
If it's a `varint`, use `#[var] pub <name>: i32` (or u32 if it makes more sense).
If it's a `varlong`, use `#[var] pub <name>: i64` (or u64).
If it's a byte, use i8 or u8.
Etc.. You can look at [wiki.vg](https://wiki.vg/Protocol) if you're not sure about how a packet is structured, but be aware that wiki.vg uses different names for most things.
5. Add the packet to the `mod.rs` file in the same directory. You will have to look at [wiki.vg](https://wiki.vg/Protocol) to determine the packet id here.
6. That's it! Format your code, submit a pull request, and wait for it to be reviewed.
