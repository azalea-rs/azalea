use azalea_protocol::packets::game::serverbound_client_command_packet::{
    self, ServerboundClientCommandPacket,
};
use bevy_app::{App, Plugin};
use bevy_ecs::prelude::*;

use crate::LocalPlayer;

/// Tell the server that we're respawning.
#[derive(Debug, Clone)]
pub struct PerformRespawnEvent {
    pub entity: Entity,
}

/// A plugin that makes [`PerformRespawnEvent`] send the packet to respawn.
pub struct RespawnPlugin;
impl Plugin for RespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PerformRespawnEvent>()
            .add_system(perform_respawn);
    }
}

pub fn perform_respawn(
    mut events: EventReader<PerformRespawnEvent>,
    mut query: Query<&mut LocalPlayer>,
) {
    for event in events.iter() {
        if let Ok(local_player) = query.get_mut(event.entity) {
            local_player.write_packet(
                ServerboundClientCommandPacket {
                    action: serverbound_client_command_packet::Action::PerformRespawn,
                }
                .get(),
            );
        }
    }
}
