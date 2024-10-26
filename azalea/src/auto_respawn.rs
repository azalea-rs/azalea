use azalea_client::{
    packet_handling::{death_event_on_0_health, game::DeathEvent},
    respawn::{perform_respawn, PerformRespawnEvent},
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
    mut events: EventReader<DeathEvent>,
    mut perform_respawn_events: EventWriter<PerformRespawnEvent>,
) {
    for event in events.read() {
        perform_respawn_events.send(PerformRespawnEvent {
            entity: event.entity,
        });
    }
}
