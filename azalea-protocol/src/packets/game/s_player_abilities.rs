use std::io::Cursor;

use azalea_buf::{AzaleaRead, AzaleaWrite};
use azalea_core::bitset::FixedBitSet;
use azalea_protocol_macros::ServerboundGamePacket;

use crate::packets::BufReadError;

#[derive(Clone, Debug, ServerboundGamePacket)]
pub struct ServerboundPlayerAbilities {
    pub is_flying: bool,
}

impl AzaleaRead for ServerboundPlayerAbilities {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let set = FixedBitSet::<{ 2_usize.div_ceil(8) }>::azalea_read(buf)?;
        Ok(Self {
            is_flying: set.index(1),
        })
    }
}

impl AzaleaWrite for ServerboundPlayerAbilities {
    fn azalea_write(&self, buf: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        let mut set = FixedBitSet::<{ 2_usize.div_ceil(8) }>::new();
        if self.is_flying {
            set.set(1);
        }
        set.azalea_write(buf)
    }
}
