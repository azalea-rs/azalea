use std::io::{Cursor, Write};

use azalea_buf::{AzBuf, AzaleaRead, AzaleaWrite, BufReadError};
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_registry::MobEffect;
use azalea_world::MinecraftEntityId;

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundGamePacket)]
pub struct ClientboundUpdateMobEffect {
    #[var]
    pub entity_id: MinecraftEntityId,
    pub mob_effect: MobEffect,
    #[var]
    pub effect_amplifier: u32,
    #[var]
    pub effect_duration_ticks: u32,
    pub flags: MobEffectFlags,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MobEffectFlags {
    pub ambient: bool,
    pub show_particles: bool,
    pub show_icon: bool,
}

impl AzaleaRead for MobEffectFlags {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let bits = u8::azalea_read(buf)?;
        Ok(MobEffectFlags {
            ambient: bits & 0x01 != 0,
            show_particles: bits & 0x02 != 0,
            show_icon: bits & 0x04 != 0,
        })
    }
}

impl AzaleaWrite for MobEffectFlags {
    fn azalea_write(&self, buf: &mut impl Write) -> std::io::Result<()> {
        let mut bits = 0;
        if self.ambient {
            bits |= 0x01;
        }
        if self.show_particles {
            bits |= 0x02;
        }
        if self.show_icon {
            bits |= 0x04;
        }
        bits.azalea_write(buf)
    }
}
