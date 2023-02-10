#![doc = include_str!("../README.md")]
#![feature(trait_upcasting)]

mod behavior;
mod blocks;

use azalea_buf::{BufReadError, McBufReadable, McBufVarReadable, McBufVarWritable, McBufWritable};
pub use behavior::BlockBehavior;
pub use blocks::*;
use std::io::{Cursor, Write};

impl BlockState {
    /// Transmutes a u32 to a block state.
    ///
    /// # Safety
    /// The `state_id` should be a valid block state.
    #[inline]
    pub unsafe fn from_u32_unchecked(state_id: u32) -> Self {
        BlockState { id: state_id }
    }

    #[inline]
    pub fn is_valid_state(state_id: u32) -> bool {
        state_id <= Self::max_state()
    }
}

impl TryFrom<u32> for BlockState {
    type Error = ();

    /// Safely converts a state id to a block state.
    fn try_from(state_id: u32) -> Result<Self, Self::Error> {
        if Self::is_valid_state(state_id) {
            Ok(unsafe { Self::from_u32_unchecked(state_id) })
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
        assert!(
            formatted.ends_with(", FloweringAzaleaBlock)"),
            "{}",
            formatted
        );

        let formatted = format!(
            "{:?}",
            BlockState::from(azalea_registry::Block::BigDripleafStem)
        );
        assert!(
            formatted.ends_with(", BigDripleafStemBlock { facing: North, waterlogged: false })"),
            "{}",
            formatted
        );
    }
}
