use azalea_client::attack::{
    AttackEvent, AttackStrengthScale, TicksSinceLastAttack, get_attack_strength_delay,
};
use azalea_entity::Attributes;
use bevy_ecs::entity::Entity;

use crate::Client;

impl Client {
    /// Attack an entity in the world.
    ///
    /// This doesn't automatically look at the entity or perform any
    /// range/visibility checks, so it might trigger anticheats.
    pub fn attack(&self, entity: Entity) {
        self.ecs.write().write_message(AttackEvent {
            entity: self.entity,
            target: entity,
        });
    }

    /// Whether the player has an attack cooldown.
    ///
    /// Also see [`Client::attack_cooldown_remaining_ticks`].
    pub fn has_attack_cooldown(&self) -> bool {
        let Some(attack_strength_scale) = self.get_component::<AttackStrengthScale>() else {
            // they don't even have an AttackStrengthScale so they probably can't even
            // attack? whatever, just return false
            return false;
        };
        **attack_strength_scale < 1.0
    }

    /// Returns the number of ticks until we can attack at full strength again.
    ///
    /// Also see [`Client::has_attack_cooldown`].
    pub fn attack_cooldown_remaining_ticks(&self) -> usize {
        let ecs = self.ecs.read();

        let Some(attributes) = ecs.get::<Attributes>(self.entity) else {
            return 0;
        };
        let Some(ticks_since_last_attack) = ecs.get::<TicksSinceLastAttack>(self.entity) else {
            return 0;
        };

        let attack_strength_delay = get_attack_strength_delay(attributes);
        let remaining_ticks = attack_strength_delay - **ticks_since_last_attack as f32;

        remaining_ticks.max(0.).ceil() as usize
    }
}
