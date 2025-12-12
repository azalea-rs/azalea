# `azalea-block`

Representation of Minecraft block states.

There are three block types, used for different things. You can (mostly) convert between them with `.into()`.

## `BlockState` struct

[`BlockState`] is a struct containing the numerical protocol ID of a block state. This is how blocks are stored in the world.

```
# use azalea_block::BlockState;
let block_state: BlockState = azalea_block::blocks::CobblestoneWall {
    east: azalea_block::properties::WallEast::Low,
    north: azalea_block::properties::WallNorth::Low,
    south: azalea_block::properties::WallSouth::Low,
    west: azalea_block::properties::WallWest::Low,
    up: false,
    waterlogged: false,
}
.into();
```
```
# use azalea_block::BlockState;
# use azalea_registry::builtin::BlockKind;
let block_state: BlockState = BlockKind::Jukebox.into();
```

## `BlockTrait`

The [`BlockTrait`] trait represents a type of a block. With [`BlockTrait`], you can get some extra things like the string block ID and some information about the block's behavior. Also, the structs that implement the trait contain the block attributes as fields so it's more convenient to get them. Note that this is often used as `Box<dyn BlockTrait>`.
If for some reason you don't want `BlockTrait`, set `default-features = false`.

```
# use azalea_block::{BlockTrait, BlockState};
# let block_state = BlockState::from(azalea_registry::builtin::BlockKind::Jukebox);
let block = Box::<dyn BlockTrait>::from(block_state);
```
```
# use azalea_block::{BlockTrait, BlockState};
# let block_state: BlockState = azalea_registry::builtin::BlockKind::Jukebox.into();
if let Some(jukebox) = Box::<dyn BlockTrait>::from(block_state).downcast_ref::<azalea_block::blocks::Jukebox>() {
    // ...
}
```


## `azalea_registry::builtin::BlockKind`

This one isn't from the `azalea-block` crate, but it's still very relevant. It's an enum that contains every block type as a variant *without* containing any state data (unlike `BlockState` and `BlockTrait`). Converting this into any other block type will use the default state for that block.

