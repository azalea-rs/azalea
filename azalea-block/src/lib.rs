#![doc = include_str!("../README.md")]

mod behavior;
pub mod block_state;
pub mod fluid_state;
mod generated;
mod range;

use core::fmt::Debug;
use std::any::Any;

pub use behavior::BlockBehavior;
// re-exported for convenience
pub use block_state::BlockState;
pub use generated::{blocks, properties};
pub use range::BlockStates;

pub trait Block: Debug + Any {
    fn behavior(&self) -> BlockBehavior;
    /// Get the Minecraft ID for this block. For example `stone` or
    /// `grass_block`.
    fn id(&self) -> &'static str;
    /// Convert the block to a block state. This is lossless, as the block
    /// contains all the state data.
    fn as_block_state(&self) -> BlockState;
    /// Convert the block to an [`azalea_registry::Block`]. This is lossy, as
    /// `azalea_registry::Block` doesn't contain any state data.
    fn as_registry_block(&self) -> azalea_registry::Block;
}
impl dyn Block {
    pub fn downcast_ref<T: Block>(&self) -> Option<&T> {
        (self as &dyn Any).downcast_ref::<T>()
    }
}

pub trait Property {
    type Value;

    fn try_from_block_state(state: BlockState) -> Option<Self::Value>;
}
