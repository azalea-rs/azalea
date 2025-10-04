use azalea_core::tick::GameTick;
use azalea_physics::PhysicsSystems;
use azalea_world::InstanceName;
use bevy_app::{App, Plugin};
use bevy_ecs::prelude::*;

use crate::{mining::MiningSystems, movement::send_position, tick_broadcast::send_tick_broadcast};

/// Counts the number of game ticks elapsed on the **local client** since the
/// `login` packet was received.
#[derive(Component, Clone, Debug, Default)]
pub struct TicksConnected(pub u64);

/// Inserts the counter-increment system into the `GameTick` schedule **before**
/// physics, mining and movement.
pub struct TickCounterPlugin;

impl Plugin for TickCounterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            GameTick,
            increment_counter
                .before(PhysicsSystems)
                .before(MiningSystems)
                .before(send_position)
                .before(send_tick_broadcast),
        );
    }
}

/// Increment the [`GameTickCounter`] on every entity that lives in an instance.
fn increment_counter(mut query: Query<&mut TicksConnected, With<InstanceName>>) {
    for mut counter in &mut query {
        counter.0 += 1;
    }
}
