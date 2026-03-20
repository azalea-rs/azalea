use std::{
    collections::HashMap,
    io::{self, Cursor, Write},
};

use azalea_buf::{AzBuf, BufReadError};
use azalea_core::{attribute_modifier_operation::AttributeModifierOperation, bitset::FixedBitSet};
use azalea_inventory::components::AttributeModifier;
use azalea_registry::{
    builtin::{Attribute, MobEffect},
    identifier::Identifier,
};

/// Data about an active mob effect.
#[derive(AzBuf, Clone, Debug, Default, PartialEq)]
pub struct MobEffectData {
    /// The effect's amplifier level, starting at 0 if present.
    #[var]
    pub amplifier: i32,
    /// The effect's duration in ticks.
    #[var]
    pub duration: i32,

    pub flags: MobEffectFlags,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MobEffectFlags {
    pub ambient: bool,
    pub show_particles: bool,
    pub show_icon: bool,
    pub blend: bool,
}

impl AzBuf for MobEffectFlags {
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

/// The active mob effects on an entity.
#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::component::Component))]
#[derive(Clone, Debug, Default)]
pub struct ActiveEffects(pub HashMap<MobEffect, MobEffectData>);
impl ActiveEffects {
    pub fn insert(&mut self, effect: MobEffect, data: MobEffectData) -> Option<MobEffectData> {
        self.0.insert(effect, data)
    }

    pub fn remove(&mut self, effect: MobEffect) -> Option<MobEffectData> {
        self.0.remove(&effect)
    }

    /// Get the amplifier level for the effect, starting at 0.
    pub fn get_level(&self, effect: MobEffect) -> Option<i32> {
        self.0.get(&effect).map(|data| data.amplifier)
    }

    pub fn get(&self, effect: MobEffect) -> Option<&MobEffectData> {
        self.0.get(&effect)
    }

    /// Returns the amplifier for dig speed (haste / conduit power), if present.
    pub fn get_dig_speed_amplifier(&self) -> Option<i32> {
        let haste_level = self
            .get_level(MobEffect::Haste)
            .map(|level| level + 1)
            .unwrap_or_default();
        let conduit_power_level = self
            .get_level(MobEffect::ConduitPower)
            .map(|level| level + 1)
            .unwrap_or_default();

        let effect_plus_one = i32::max(haste_level, conduit_power_level);
        if effect_plus_one > 0 {
            Some(effect_plus_one - 1)
        } else {
            None
        }
    }
}

pub fn attribute_modifier_for_effect(id: MobEffect) -> Option<(Attribute, AttributeTemplate)> {
    Some(match id {
        MobEffect::Speed => (
            Attribute::MovementSpeed,
            AttributeTemplate::new(
                "effect.speed",
                0.2f32 as f64,
                AttributeModifierOperation::AddMultipliedTotal,
            ),
        ),
        MobEffect::Slowness => (
            Attribute::MovementSpeed,
            AttributeTemplate::new(
                "effect.slowness",
                -0.15f32 as f64,
                AttributeModifierOperation::AddMultipliedTotal,
            ),
        ),
        MobEffect::Haste => (
            Attribute::AttackSpeed,
            AttributeTemplate::new(
                "effect.haste",
                0.1f32 as f64,
                AttributeModifierOperation::AddMultipliedTotal,
            ),
        ),
        MobEffect::MiningFatigue => (
            Attribute::AttackSpeed,
            AttributeTemplate::new(
                "effect.mining_fatigue",
                -0.1f32 as f64,
                AttributeModifierOperation::AddMultipliedTotal,
            ),
        ),
        MobEffect::Strength => (
            Attribute::AttackDamage,
            AttributeTemplate::new("effect.strength", 3.0, AttributeModifierOperation::AddValue),
        ),
        MobEffect::JumpBoost => (
            Attribute::SafeFallDistance,
            AttributeTemplate::new(
                "effect.jump_boost",
                1.0,
                AttributeModifierOperation::AddValue,
            ),
        ),
        MobEffect::Invisibility => (
            Attribute::WaypointTransmitRange,
            AttributeTemplate::new(
                "effect.waypoint_transmit_range_hide",
                -1.0,
                AttributeModifierOperation::AddMultipliedTotal,
            ),
        ),
        MobEffect::Weakness => (
            Attribute::AttackDamage,
            AttributeTemplate::new(
                "effect.weakness",
                -4.0,
                AttributeModifierOperation::AddValue,
            ),
        ),
        MobEffect::HealthBoost => (
            Attribute::MaxHealth,
            AttributeTemplate::new(
                "effect.health_boost",
                4.0,
                AttributeModifierOperation::AddValue,
            ),
        ),
        MobEffect::Absorption => (
            Attribute::MaxAbsorption,
            AttributeTemplate::new(
                "effect.absorption",
                4.0,
                AttributeModifierOperation::AddValue,
            ),
        ),
        MobEffect::Luck => (
            Attribute::Luck,
            AttributeTemplate::new("effect.luck", 1.0, AttributeModifierOperation::AddValue),
        ),
        MobEffect::Unluck => (
            Attribute::Luck,
            AttributeTemplate::new("effect.unluck", -1.0, AttributeModifierOperation::AddValue),
        ),
        _ => return None,
    })
}

pub struct AttributeTemplate(AttributeModifier);
impl AttributeTemplate {
    pub fn new(id: &str, amount: f64, operation: AttributeModifierOperation) -> Self {
        Self(AttributeModifier {
            id: Identifier::from(id),
            amount,
            operation,
        })
    }
    pub fn create(self, amplifier: i32) -> AttributeModifier {
        AttributeModifier {
            id: self.0.id,
            amount: self.0.amount * (amplifier + 1) as f64,
            operation: self.0.operation,
        }
    }
}
