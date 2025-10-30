use azalea_core::{game_type::GameMode, tick::GameTick};
use azalea_entity::{
    Attributes, Crouching, Physics, indexing::EntityIdIndex, metadata::Sprinting,
    update_bounding_box,
};
use azalea_physics::PhysicsSystems;
use azalea_protocol::packets::game::s_interact::{self, ServerboundInteract};
use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::*;
use derive_more::{Deref, DerefMut};
use tracing::warn;

use super::packet::game::SendGamePacketEvent;
use crate::{
    Client, interact::SwingArmEvent, local_player::LocalGameMode, movement::MoveEventsSystems,
    respawn::perform_respawn,
};

pub struct AttackPlugin;
impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<AttackEvent>()
            .add_systems(
                Update,
                handle_attack_event
                    .before(update_bounding_box)
                    .before(MoveEventsSystems)
                    .after(perform_respawn),
            )
            .add_systems(
                GameTick,
                (
                    increment_ticks_since_last_attack,
                    update_attack_strength_scale.after(PhysicsSystems),
                    handle_attack_queued
                        .before(super::tick_end::game_tick_packet)
                        .after(super::movement::send_sprinting_if_needed)
                        .before(super::movement::send_position),
                )
                    .chain(),
            );
    }
}

impl Client {
    /// Attack an entity in the world.
    ///
    /// This doesn't automatically look at the entity or perform any
    /// range/visibility checks, so it might trigger anticheats.
    pub fn attack(&self, entity: Entity) {
        self.ecs.lock().write_message(AttackEvent {
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
        *attack_strength_scale < 1.0
    }

    /// Returns the number of ticks until we can attack at full strength again.
    ///
    /// Also see [`Client::has_attack_cooldown`].
    pub fn attack_cooldown_remaining_ticks(&self) -> usize {
        let mut ecs = self.ecs.lock();
        let Ok((attributes, ticks_since_last_attack)) = ecs
            .query::<(&Attributes, &TicksSinceLastAttack)>()
            .get(&ecs, self.entity)
        else {
            return 0;
        };

        let attack_strength_delay = get_attack_strength_delay(attributes);
        let remaining_ticks = attack_strength_delay - **ticks_since_last_attack as f32;

        remaining_ticks.max(0.).ceil() as usize
    }
}

/// A component that indicates that this client will be attacking the given
/// entity next tick.
#[derive(Component, Clone, Debug)]
pub struct AttackQueued {
    pub target: Entity,
}
#[allow(clippy::type_complexity)]
pub fn handle_attack_queued(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut TicksSinceLastAttack,
        &mut Physics,
        &mut Sprinting,
        &AttackQueued,
        &LocalGameMode,
        &Crouching,
        &EntityIdIndex,
    )>,
) {
    for (
        client_entity,
        mut ticks_since_last_attack,
        mut physics,
        mut sprinting,
        attack_queued,
        game_mode,
        crouching,
        entity_id_index,
    ) in &mut query
    {
        let target_entity = attack_queued.target;
        let Some(target_entity_id) = entity_id_index.get_by_ecs_entity(target_entity) else {
            warn!("tried to attack entity {target_entity} which isn't in our EntityIdIndex");
            continue;
        };

        commands.entity(client_entity).remove::<AttackQueued>();

        commands.trigger(SendGamePacketEvent::new(
            client_entity,
            ServerboundInteract {
                entity_id: target_entity_id,
                action: s_interact::ActionType::Attack,
                using_secondary_action: **crouching,
            },
        ));
        commands.trigger(SwingArmEvent {
            entity: client_entity,
        });

        // we can't attack if we're in spectator mode but it still sends the attack
        // packet
        if game_mode.current == GameMode::Spectator {
            continue;
        };

        ticks_since_last_attack.0 = 0;

        physics.velocity = physics.velocity.multiply(0.6, 1.0, 0.6);
        **sprinting = false;
    }
}

/// Queues up an attack packet for next tick by inserting the [`AttackQueued`]
/// component to our client.
#[derive(Message)]
pub struct AttackEvent {
    /// Our client entity that will send the packets to attack.
    pub entity: Entity,
    /// The entity that will be attacked.
    pub target: Entity,
}
pub fn handle_attack_event(mut events: MessageReader<AttackEvent>, mut commands: Commands) {
    for event in events.read() {
        commands.entity(event.entity).insert(AttackQueued {
            target: event.target,
        });
    }
}

#[derive(Default, Bundle)]
pub struct AttackBundle {
    pub ticks_since_last_attack: TicksSinceLastAttack,
    pub attack_strength_scale: AttackStrengthScale,
}

#[derive(Default, Component, Clone, Deref, DerefMut)]
pub struct TicksSinceLastAttack(pub u32);
pub fn increment_ticks_since_last_attack(mut query: Query<&mut TicksSinceLastAttack>) {
    for mut ticks_since_last_attack in query.iter_mut() {
        **ticks_since_last_attack += 1;
    }
}

#[derive(Default, Component, Clone, Deref, DerefMut)]
pub struct AttackStrengthScale(pub f32);
pub fn update_attack_strength_scale(
    mut query: Query<(&TicksSinceLastAttack, &Attributes, &mut AttackStrengthScale)>,
) {
    for (ticks_since_last_attack, attributes, mut attack_strength_scale) in query.iter_mut() {
        // look 0.5 ticks into the future because that's what vanilla does
        **attack_strength_scale =
            get_attack_strength_scale(ticks_since_last_attack.0, attributes, 0.5);
    }
}

/// Returns how long it takes for the attack cooldown to reset (in ticks).
pub fn get_attack_strength_delay(attributes: &Attributes) -> f32 {
    ((1. / attributes.attack_speed.calculate()) * 20.) as f32
}

pub fn get_attack_strength_scale(
    ticks_since_last_attack: u32,
    attributes: &Attributes,
    in_ticks: f32,
) -> f32 {
    let attack_strength_delay = get_attack_strength_delay(attributes);
    let attack_strength = (ticks_since_last_attack as f32 + in_ticks) / attack_strength_delay;
    attack_strength.clamp(0., 1.)
}
