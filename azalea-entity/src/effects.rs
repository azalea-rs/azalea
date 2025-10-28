use std::collections::HashMap;

use azalea_registry::MobEffect;
use bevy_ecs::component::Component;

/// Data about an active mob effect that the client knows about.
#[derive(Clone, Debug, Default)]
pub struct MobEffectData {
    pub amplifier: u32,
    pub duration_ticks: u32,
    pub flags: u8,
}
impl MobEffectData {
    pub fn is_ambient(&self) -> bool {
        self.flags & 0x01 != 0
    }
    pub fn should_show_particles(&self) -> bool {
        self.flags & 0x02 != 0
    }
    pub fn should_show_icon(&self) -> bool {
        self.flags & 0x04 != 0
    }
}

/// Component storing the active mob effects on an entity.
#[derive(Component, Clone, Debug, Default)]
pub struct ActiveEffects(pub HashMap<MobEffect, MobEffectData>);
impl ActiveEffects {
    pub fn insert(&mut self, effect: MobEffect, data: MobEffectData) {
        self.0.insert(effect, data);
    }

    pub fn remove(&mut self, effect: MobEffect) -> Option<MobEffectData> {
        self.0.remove(&effect)
    }

    pub fn get_level(&self, effect: MobEffect) -> Option<u32> {
        self.0.get(&effect).map(|data| data.amplifier)
    }

    pub fn get(&self, effect: MobEffect) -> Option<&MobEffectData> {
        self.0.get(&effect)
    }
}

/// Returns the level (amplifier) of the given effect, or `None` if the effect
/// is not active. The lowest level is 0.
pub fn get_effect(active_effects: &ActiveEffects, effect: MobEffect) -> Option<u32> {
    active_effects.get_level(effect)
}

/// Returns the amplifier for dig speed (haste / conduit power), if present.
pub fn get_dig_speed_amplifier(active_effects: &ActiveEffects) -> Option<u32> {
    let effect_plus_one = u32::max(
        get_effect(active_effects, MobEffect::Haste)
            .map(|level| level + 1)
            .unwrap_or_default(),
        get_effect(active_effects, MobEffect::ConduitPower)
            .map(|level| level + 1)
            .unwrap_or_default(),
    );
    if effect_plus_one > 0 {
        Some(effect_plus_one - 1)
    } else {
        None
    }
}
