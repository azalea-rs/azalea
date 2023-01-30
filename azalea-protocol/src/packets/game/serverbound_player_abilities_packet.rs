use crate::packets::BufReadError;
use azalea_buf::{McBufReadable, McBufWritable};
use azalea_core::FixedBitSet;
use azalea_protocol_macros::ServerboundGamePacket;
use std::io::Cursor;

#[derive(Clone, Debug, ServerboundGamePacket)]
pub struct ServerboundPlayerAbilitiesPacket {
    pub is_flying: bool,
}

impl McBufReadable for ServerboundPlayerAbilitiesPacket {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let set = FixedBitSet::<2>::read_from(buf)?;
        Ok(Self {
            is_flying: set.index(1),
        })
    }
}

impl McBufWritable for ServerboundPlayerAbilitiesPacket {
    fn write_into(&self, buf: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        let mut set = FixedBitSet::<2>::new();
        if self.is_flying {
            set.set(1);
        }
        set.write_into(buf)
    }
}
