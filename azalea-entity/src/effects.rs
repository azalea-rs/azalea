use std::{
    collections::HashMap,
    io::{self, Cursor, Write},
};

use azalea_buf::{AzBuf, AzaleaRead, AzaleaWrite, BufReadError};
use azalea_core::bitset::FixedBitSet;
use azalea_registry::MobEffect;
use bevy_ecs::component::Component;

/// Data about an active mob effect.
#[derive(Clone, Debug, Default, PartialEq, AzBuf)]
pub struct MobEffectData {
    /// The effect's amplifier level, starting at 0 if present.
    #[var]
    pub amplifier: u32,
    #[var]
    pub duration_ticks: u32,

    pub flags: MobEffectFlags,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MobEffectFlags {
    pub ambient: bool,
    pub show_particles: bool,
    pub show_icon: bool,
    pub blend: bool,
}

impl AzaleaRead for MobEffectFlags {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let bitset = FixedBitSet::<8>::azalea_read(buf)?;
        let ambient = bitset.index(0);
        let show_particles = bitset.index(1);
        let show_icon = bitset.index(2);
        let blend = bitset.index(3);
        Ok(Self {
            ambient,
            show_particles,
            show_icon,
            blend,
        })
    }
}

impl AzaleaWrite for MobEffectFlags {
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        let mut bitset = FixedBitSet::<8>::new();
        if self.ambient {
            bitset.set(0);
        }
        if self.show_particles {
            bitset.set(1);
        }
        if self.show_icon {
            bitset.set(2);
        }
        if self.blend {
            bitset.set(3);
        }
        bitset.azalea_write(buf)
    }
}

/// An ECS component that stores the active mob effects on an entity.
#[derive(Component, Clone, Debug, Default)]
pub struct ActiveEffects(pub HashMap<MobEffect, MobEffectData>);
impl ActiveEffects {
    pub fn insert(&mut self, effect: MobEffect, data: MobEffectData) {
        self.0.insert(effect, data);
    }

    pub fn remove(&mut self, effect: MobEffect) -> Option<MobEffectData> {
        self.0.remove(&effect)
    }

    /// Get the amplifier level for the effect, starting at 0.
    pub fn get_level(&self, effect: MobEffect) -> Option<u32> {
        self.0.get(&effect).map(|data| data.amplifier)
    }

    pub fn get(&self, effect: MobEffect) -> Option<&MobEffectData> {
        self.0.get(&effect)
    }

    /// Returns the amplifier for dig speed (haste / conduit power), if present.
    pub fn get_dig_speed_amplifier(&self) -> Option<u32> {
        let haste_level = self
            .get_level(MobEffect::Haste)
            .map(|level| level + 1)
            .unwrap_or_default();
        let conduit_power_level = self
            .get_level(MobEffect::ConduitPower)
            .map(|level| level + 1)
            .unwrap_or_default();

        let effect_plus_one = u32::max(haste_level, conduit_power_level);
        if effect_plus_one > 0 {
            Some(effect_plus_one - 1)
        } else {
            None
        }
    }
}
