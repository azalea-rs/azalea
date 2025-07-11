use azalea_core::tick::GameTick;
use azalea_physics::PhysicsSet;
use azalea_world::InstanceName;

use bevy_app::{App, Plugin};
use bevy_ecs::prelude::*;

use crate::{local_player::TicksAlive, mining::MiningSet, movement::send_position, tick_broadcast::send_tick_broadcast};

/// Inserts the counter-increment system into the `GameTick` schedule **before**
/// physics, mining and movement.
pub struct TickCounterPlugin;

impl Plugin for TickCounterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            GameTick,
            increment_counter
                .before(PhysicsSet)
                .before(MiningSet)
                .before(send_position)
                .before(send_tick_broadcast),
        );
    }
}

/// Increment the [`GameTickCounter`] on every entity that lives in an instance.
fn increment_counter(mut query: Query<&mut TicksAlive, With<InstanceName>>) {
    for mut counter in &mut query {
        counter.0 += 1;
    }
}
