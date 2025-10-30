use std::{
    fmt::{self, Debug},
    io::{self, Cursor, Write},
};

use azalea_buf::{AzaleaRead, AzaleaReadVar, AzaleaWrite, AzaleaWriteVar, BufReadError};

use crate::BlockTrait;

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
    id: BlockStateIntegerRepr,
}

impl BlockState {
    /// A shortcut for getting the air block state, since it always has an ID of
    /// 0.
    ///
    /// This does not include the other types of air like cave air.
    pub const AIR: BlockState = BlockState { id: 0 };

    /// Create a new BlockState and panic if the block is not a valid state.
    ///
    /// You should probably use [`BlockState::try_from`] instead.
    #[inline]
    pub(crate) const fn new_const(id: BlockStateIntegerRepr) -> Self {
        assert!(Self::is_valid_state(id));
        Self { id }
    }

    /// Whether the block state is possible to exist in vanilla Minecraft.
    ///
    /// It's equivalent to checking that the state ID is not greater than
    /// [`Self::MAX_STATE`].
    #[inline]
    pub const fn is_valid_state(state_id: BlockStateIntegerRepr) -> bool {
        state_id <= Self::MAX_STATE
    }

    /// Returns true if the block is air.
    ///
    /// This only checks for normal air, not other types like cave air.
    #[inline]
    pub fn is_air(&self) -> bool {
        self == &Self::AIR
    }

    /// Returns the protocol ID for the block state.
    ///
    /// These IDs may change across Minecraft versions, so you shouldn't
    /// hard-code them or store them in databases.
    #[inline]
    pub const fn id(&self) -> BlockStateIntegerRepr {
        self.id
    }
}

impl TryFrom<u32> for BlockState {
    type Error = ();

    /// Safely converts a u32 state ID to a block state.
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

    /// Safely converts a u16 state ID to a block state.
    fn try_from(state_id: u16) -> Result<Self, Self::Error> {
        let state_id = state_id as BlockStateIntegerRepr;
        if Self::is_valid_state(state_id) {
            Ok(BlockState { id: state_id })
        } else {
            Err(())
        }
    }
}
impl From<BlockState> for u32 {
    /// See [`BlockState::id`].
    fn from(value: BlockState) -> Self {
        value.id as u32
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
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        u32::azalea_write_var(&(self.id as u32), buf)
    }
}

impl Debug for BlockState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BlockState(id: {}, {:?})",
            self.id,
            Box::<dyn BlockTrait>::from(*self)
        )
    }
}

impl From<BlockState> for azalea_registry::Block {
    fn from(value: BlockState) -> Self {
        Box::<dyn BlockTrait>::from(value).as_registry_block()
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
        let block: Box<dyn BlockTrait> = Box::<dyn BlockTrait>::from(BlockState::AIR);
        assert_eq!(block.id(), "air");

        let block: Box<dyn BlockTrait> =
            Box::<dyn BlockTrait>::from(BlockState::from(azalea_registry::Block::FloweringAzalea));
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
