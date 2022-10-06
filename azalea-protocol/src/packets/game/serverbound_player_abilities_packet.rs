use crate::packets::BufReadError;
use azalea_buf::{McBufReadable, McBufWritable};
use azalea_protocol_macros::ServerboundGamePacket;
use std::io::Cursor;

#[derive(Clone, Debug, ServerboundGamePacket)]
pub struct ServerboundPlayerAbilitiesPacket {
    is_flying: bool,
}

impl McBufReadable for ServerboundPlayerAbilitiesPacket {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let byte = u8::read_from(buf)?;
        Ok(Self {
            is_flying: byte & 2 != 0,
        })
    }
}

impl McBufWritable for ServerboundPlayerAbilitiesPacket {
    fn write_into(&self, buf: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        let mut byte = 0;
        if self.is_flying {
            byte |= 2;
        }
        byte.write_into(buf)?;
        Ok(())
    }
}
