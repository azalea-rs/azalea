use azalea_buf::{BufReadError, McBuf};
use azalea_buf::{McBufReadable, McBufWritable};
use azalea_core::FixedBitSet;
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
        let set = FixedBitSet::<4>::read_from(buf)?;
        Ok(PlayerAbilitiesFlags {
            invulnerable: set.index(0),
            flying: set.index(1),
            can_fly: set.index(2),
            instant_break: set.index(3),
        })
    }
}

impl McBufWritable for PlayerAbilitiesFlags {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let mut set = FixedBitSet::<4>::new();
        if self.invulnerable {
            set.set(0);
        }
        if self.flying {
            set.set(1);
        }
        if self.can_fly {
            set.set(2);
        }
        if self.instant_break {
            set.set(3);
        }
        set.write_into(buf)
    }
}
