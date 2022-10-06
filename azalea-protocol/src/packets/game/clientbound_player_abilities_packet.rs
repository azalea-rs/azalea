use azalea_buf::{BufReadError, McBuf};
use azalea_buf::{McBufReadable, McBufWritable};
use azalea_protocol_macros::ClientboundGamePacket;
use std::io::{Cursor, Write};

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundPlayerAbilitiesPacket {
    pub flags: PlayerAbilitiesFlags,
    pub flying_speed: f32,
    /// Used for the fov
    pub walking_speed: f32,
}

#[derive(Clone, Debug)]
pub struct PlayerAbilitiesFlags {
    pub invulnerable: bool,
    pub flying: bool,
    pub can_fly: bool,
    pub instant_break: bool,
}

impl McBufReadable for PlayerAbilitiesFlags {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let byte = u8::read_from(buf)?;
        Ok(PlayerAbilitiesFlags {
            invulnerable: byte & 1 != 0,
            flying: byte & 2 != 0,
            can_fly: byte & 4 != 0,
            instant_break: byte & 8 != 0,
        })
    }
}

impl McBufWritable for PlayerAbilitiesFlags {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let mut byte = 0;
        if self.invulnerable {
            byte |= 0b1;
        }
        if self.flying {
            byte |= 0b10;
        }
        if self.can_fly {
            byte |= 0b100;
        }
        if self.instant_break {
            byte |= 0b1000;
        }
        u8::write_into(&byte, buf)
    }
}
