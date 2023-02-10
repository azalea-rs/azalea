Representation of Minecraft block states.

There's three block types, used for different things. You can (mostly) convert between them with `.into()`.

## BlockState struct

[`BlockState`] is a struct containing the numerical protocol ID of a block state. This is how blocks are stored in the world.

```
# use azalea_block::BlockState;
let block_state: BlockState = azalea_block::CobblestoneWallBlock {
    east: azalea_block::EastWall::Low,
    north: azalea_block::NorthWall::Low,
    south: azalea_block::SouthWall::Low,
    west: azalea_block::WestWall::Low,
    up: false,
    waterlogged: false,
}
.into();
```
```
# use azalea_block::BlockState;
let block_state: BlockState = azalea_registry::Block::Jukebox.into();
```

## Block trait

The [`Block`] trait represents a type of a block. With the the [`Block`] trait, you can get some extra things like the string block ID and some information about the block's behavior. Also, the structs that implement the trait contain the block attributes as fields so it's more convenient to get them. Note that this is often used as `Box<dyn Block>`.
If for some reason you don't want the `Block` trait, set default-features to false.

```
# use azalea_block::{Block, BlockState};
# let block_state = BlockState::from(azalea_registry::Block::Jukebox);
let block = Box::<dyn Block>::from(block_state);
```
```
# use azalea_block::{Block, BlockState};
# let block_state: BlockState = azalea_registry::Block::Jukebox.into();
if let Some(jukebox) = Box::<dyn Block>::from(block_state).downcast_ref::<azalea_block::JukeboxBlock>() {
    // ...
}
```


## azalea_registry::Block enum

This one technically isn't from the `azalea-block` crate, but it's still very relevant. It's an enum that contains every block type as a variant *without* containing any state data (unlike `BlockState` and the `Block` trait). Converting this into any other block type will use the default state for that block.

