// TODO

// pub struct ActiveEffects(HashMap<azalea_registry::MobEffect, MobEffectData>);

/// Returns the level of the given effect, or `None` if the effect is not
/// active. The lowest level is 0.
pub fn get_effect(effect: azalea_registry::MobEffect) -> Option<u32> {
    // TODO
    None
}

pub fn get_dig_speed_amplifier() -> Option<u32> {
    let effect_plus_one = u32::max(
        get_effect(azalea_registry::MobEffect::Haste)
            .map(|x| x + 1)
            .unwrap_or_default(),
        get_effect(azalea_registry::MobEffect::ConduitPower)
            .map(|x| x + 1)
            .unwrap_or_default(),
    );
    if effect_plus_one > 0 {
        Some(effect_plus_one - 1)
    } else {
        None
    }
}
