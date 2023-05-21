use crate::app::{App, Plugin};
use azalea_client::packet_handling::DeathEvent;
use azalea_client::respawn::{perform_respawn, PerformRespawnEvent};
use bevy_ecs::prelude::*;

/// A plugin that makes [`DeathEvent`]s send [`PerformRespawnEvent`]s.
#[derive(Clone, Default)]
pub struct AutoRespawnPlugin;
impl Plugin for AutoRespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(auto_respawn.before(perform_respawn));
    }
}

fn auto_respawn(
    mut events: EventReader<DeathEvent>,
    mut perform_respawn_events: EventWriter<PerformRespawnEvent>,
) {
    for event in events.iter() {
        perform_respawn_events.send(PerformRespawnEvent {
            entity: event.entity,
        });
    }
}
