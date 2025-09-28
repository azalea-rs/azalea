use azalea_client::{
    packet::{death_event_on_0_health, game::DeathEvent},
    respawn::{PerformRespawnEvent, perform_respawn},
};
use bevy_app::Update;
use bevy_ecs::prelude::*;

use crate::app::{App, Plugin};

/// A plugin that makes [`DeathEvent`]s send [`PerformRespawnEvent`]s.
#[derive(Clone, Default)]
pub struct AutoRespawnPlugin;
impl Plugin for AutoRespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            auto_respawn
                .before(perform_respawn)
                .after(death_event_on_0_health),
        );
    }
}

fn auto_respawn(
    mut events: MessageReader<DeathEvent>,
    mut perform_respawn_events: MessageWriter<PerformRespawnEvent>,
) {
    for event in events.read() {
        perform_respawn_events.write(PerformRespawnEvent {
            entity: event.entity,
        });
    }
}
