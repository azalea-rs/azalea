# Azalea Block

Representation of Minecraft block states.

There's two main things here, the `BlockState` enum and the `Block` trait.
`BlockState` is a simple enum with every possible block state as variant, and `Block` is a heavier trait which lets you access information about a block more easily.

Every block is a struct that implements `Block`. You can freely convert between `BlockState` and `Block` with .into().

If you don't want the `Block` trait, set default-features to false.
