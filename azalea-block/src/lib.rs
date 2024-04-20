#![doc = include_str!("../README.md")]
#![feature(trait_upcasting)]

mod behavior;
mod generated;
mod range;

pub use generated::{blocks, properties};

use azalea_buf::{BufReadError, McBufReadable, McBufVarReadable, McBufVarWritable, McBufWritable};
pub use behavior::BlockBehavior;
use core::fmt::Debug;
pub use range::BlockStates;
use std::{
    any::Any,
    io::{Cursor, Write},
};

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

/// A representation of a state a block can be in.
///
/// For example, a stone block only has one state but each possible stair
/// rotation is a different state.
#[derive(Copy, Clone, PartialEq, Eq, Default, Hash)]
pub struct BlockState {
    /// The protocol ID for the block state. IDs may change every
    /// version, so you shouldn't hard-code them or store them in databases.
    pub id: u32,
}

impl BlockState {
    pub const AIR: BlockState = BlockState { id: 0 };

    #[inline]
    pub fn is_valid_state(state_id: u32) -> bool {
        state_id <= Self::max_state()
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

    /// Safely converts a state id to a block state.
    fn try_from(state_id: u32) -> Result<Self, Self::Error> {
        if Self::is_valid_state(state_id) {
            Ok(BlockState { id: state_id })
        } else {
            Err(())
        }
    }
}

impl McBufReadable for BlockState {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let state_id = u32::var_read_from(buf)?;
        Self::try_from(state_id).map_err(|_| BufReadError::UnexpectedEnumVariant {
            id: state_id as i32,
        })
    }
}
impl McBufWritable for BlockState {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        u32::var_write_into(&self.id, buf)
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
                    level: crate::properties::WaterLevel::from(state.height as u32),
                })
            }
            azalea_registry::Fluid::Lava | azalea_registry::Fluid::FlowingLava => {
                BlockState::from(crate::blocks::Lava {
                    level: crate::properties::LavaLevel::from(state.height as u32),
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
        assert_eq!(BlockState::try_from(0).unwrap(), BlockState::AIR);

        assert!(BlockState::try_from(BlockState::max_state()).is_ok());
        assert!(BlockState::try_from(BlockState::max_state() + 1).is_err());
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
