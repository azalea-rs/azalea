#![doc = include_str!("../README.md")]
#![feature(trait_upcasting)]

mod behavior;
mod generated;
mod range;

use core::fmt::Debug;
use std::{
    any::Any,
    io::{Cursor, Write},
};

use azalea_buf::{AzaleaRead, AzaleaReadVar, AzaleaWrite, AzaleaWriteVar, BufReadError};
pub use behavior::BlockBehavior;
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

/// The type that's used internally to represent a block state ID.
///
/// This should be either `u16` or `u32`. If you choose to modify it, you must
/// also change it in `azalea-block-macros/src/lib.rs`.
///
/// This does not affect protocol serialization, it just allows you to make the
/// internal type smaller if you want.
pub type BlockStateIntegerRepr = u16;

/// A representation of a state a block can be in.
///
/// For example, a stone block only has one state but each possible stair
/// rotation is a different state.
///
/// Note that this type is internally either a `u16` or `u32`, depending on
/// [`BlockStateIntegerRepr`].
#[derive(Copy, Clone, PartialEq, Eq, Default, Hash)]
pub struct BlockState {
    /// The protocol ID for the block state. IDs may change every
    /// version, so you shouldn't hard-code them or store them in databases.
    pub id: BlockStateIntegerRepr,
}

impl BlockState {
    pub const AIR: BlockState = BlockState { id: 0 };

    #[inline]
    pub fn is_valid_state(state_id: BlockStateIntegerRepr) -> bool {
        state_id <= Self::MAX_STATE
    }

    /// Returns true if the block is air. This only checks for normal air, not
    /// other types like cave air.
    #[inline]
    pub fn is_air(&self) -> bool {
        self == &Self::AIR
    }
}

impl TryFrom<u32> for BlockState {
    type Error = ();

    /// Safely converts a u32 state id to a block state.
    fn try_from(state_id: u32) -> Result<Self, Self::Error> {
        let state_id = state_id as BlockStateIntegerRepr;
        if Self::is_valid_state(state_id) {
            Ok(BlockState { id: state_id })
        } else {
            Err(())
        }
    }
}
impl TryFrom<u16> for BlockState {
    type Error = ();

    /// Safely converts a u16 state id to a block state.
    fn try_from(state_id: u16) -> Result<Self, Self::Error> {
        let state_id = state_id as BlockStateIntegerRepr;
        if Self::is_valid_state(state_id) {
            Ok(BlockState { id: state_id })
        } else {
            Err(())
        }
    }
}

impl AzaleaRead for BlockState {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let state_id = u32::azalea_read_var(buf)?;
        Self::try_from(state_id).map_err(|_| BufReadError::UnexpectedEnumVariant {
            id: state_id as i32,
        })
    }
}
impl AzaleaWrite for BlockState {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        u32::azalea_write_var(&(self.id as u32), buf)
    }
}

impl std::fmt::Debug for BlockState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BlockState(id: {}, {:?})",
            self.id,
            Box::<dyn Block>::from(*self)
        )
    }
}

#[derive(Clone, Debug)]
pub struct FluidState {
    pub fluid: azalea_registry::Fluid,
    pub height: u8,
}

impl Default for FluidState {
    fn default() -> Self {
        Self {
            fluid: azalea_registry::Fluid::Empty,
            height: 0,
        }
    }
}

impl From<BlockState> for FluidState {
    fn from(state: BlockState) -> Self {
        if state
            .property::<crate::properties::Waterlogged>()
            .unwrap_or_default()
        {
            Self {
                fluid: azalea_registry::Fluid::Water,
                height: 15,
            }
        } else {
            let block = Box::<dyn Block>::from(state);
            if let Some(water) = block.downcast_ref::<crate::blocks::Water>() {
                Self {
                    fluid: azalea_registry::Fluid::Water,
                    height: water.level as u8,
                }
            } else if let Some(lava) = block.downcast_ref::<crate::blocks::Lava>() {
                Self {
                    fluid: azalea_registry::Fluid::Lava,
                    height: lava.level as u8,
                }
            } else {
                Self {
                    fluid: azalea_registry::Fluid::Empty,
                    height: 0,
                }
            }
        }
    }
}

impl From<FluidState> for BlockState {
    fn from(state: FluidState) -> Self {
        match state.fluid {
            azalea_registry::Fluid::Empty => BlockState::AIR,
            azalea_registry::Fluid::Water | azalea_registry::Fluid::FlowingWater => {
                BlockState::from(crate::blocks::Water {
                    level: crate::properties::WaterLevel::from(
                        state.height as BlockStateIntegerRepr,
                    ),
                })
            }
            azalea_registry::Fluid::Lava | azalea_registry::Fluid::FlowingLava => {
                BlockState::from(crate::blocks::Lava {
                    level: crate::properties::LavaLevel::from(
                        state.height as BlockStateIntegerRepr,
                    ),
                })
            }
        }
    }
}

impl From<BlockState> for azalea_registry::Block {
    fn from(value: BlockState) -> Self {
        Box::<dyn Block>::from(value).as_registry_block()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_u32() {
        assert_eq!(
            BlockState::try_from(0 as BlockStateIntegerRepr).unwrap(),
            BlockState::AIR
        );

        assert!(BlockState::try_from(BlockState::MAX_STATE).is_ok());
        assert!(BlockState::try_from(BlockState::MAX_STATE + 1).is_err());
    }

    #[test]
    fn test_from_blockstate() {
        let block: Box<dyn Block> = Box::<dyn Block>::from(BlockState::AIR);
        assert_eq!(block.id(), "air");

        let block: Box<dyn Block> =
            Box::<dyn Block>::from(BlockState::from(azalea_registry::Block::FloweringAzalea));
        assert_eq!(block.id(), "flowering_azalea");
    }

    #[test]
    fn test_debug_blockstate() {
        let formatted = format!(
            "{:?}",
            BlockState::from(azalea_registry::Block::FloweringAzalea)
        );
        assert!(formatted.ends_with(", FloweringAzalea)"), "{}", formatted);

        let formatted = format!(
            "{:?}",
            BlockState::from(azalea_registry::Block::BigDripleafStem)
        );
        assert!(
            formatted.ends_with(", BigDripleafStem { facing: North, waterlogged: false })"),
            "{}",
            formatted
        );
    }
}
