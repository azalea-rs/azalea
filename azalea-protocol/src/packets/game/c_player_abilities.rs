use std::io::{Cursor, Write};

use azalea_buf::{AzBuf, BufReadError};
use azalea_buf::{AzaleaRead, AzaleaWrite};
use azalea_core::bitset::FixedBitSet;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundPlayerAbilities {
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

impl AzaleaRead for PlayerAbilitiesFlags {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let set = FixedBitSet::<{ 4_usize.div_ceil(8) }>::azalea_read(buf)?;
        Ok(PlayerAbilitiesFlags {
            invulnerable: set.index(0),
            flying: set.index(1),
            can_fly: set.index(2),
            instant_break: set.index(3),
        })
    }
}

impl AzaleaWrite for PlayerAbilitiesFlags {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let mut set = FixedBitSet::<{ 4_usize.div_ceil(8) }>::new();
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
        set.azalea_write(buf)
    }
}
