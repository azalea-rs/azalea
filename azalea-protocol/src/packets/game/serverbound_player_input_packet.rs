use std::io::Cursor;

use azalea_buf::BufReadError;
use azalea_buf::{McBufReadable, McBufWritable};
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, ServerboundGamePacket)]
pub struct ServerboundPlayerInputPacket {
    pub xxa: f32,
    pub zza: f32,
    pub is_jumping: bool,
    pub is_shift_key_down: bool,
}

impl McBufReadable for ServerboundPlayerInputPacket {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let xxa = f32::read_from(buf)?;
        let zza = f32::read_from(buf)?;
        let byte = u8::read_from(buf)?;
        let is_jumping = byte & 1 != 0;
        let is_shift_key_down = byte & 2 != 0;
        Ok(Self {
            xxa,
            zza,
            is_jumping,
            is_shift_key_down,
        })
    }
}

impl McBufWritable for ServerboundPlayerInputPacket {
    fn write_into(&self, buf: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        self.xxa.write_into(buf)?;
        self.zza.write_into(buf)?;
        let mut byte = 0;
        if self.is_jumping {
            byte |= 1;
        }
        if self.is_shift_key_down {
            byte |= 2;
        }
        byte.write_into(buf)?;
        Ok(())
    }
}
