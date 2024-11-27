use azalea_protocol::packets::game::s_client_command::{self, ServerboundClientCommand};
use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::*;

use crate::packet_handling::game::{handle_send_packet_event, SendPacketEvent};

/// Tell the server that we're respawning.
#[derive(Event, Debug, Clone)]
pub struct PerformRespawnEvent {
    pub entity: Entity,
}

/// A plugin that makes [`PerformRespawnEvent`] send the packet to respawn.
pub struct RespawnPlugin;
impl Plugin for RespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PerformRespawnEvent>()
            .add_systems(Update, perform_respawn.before(handle_send_packet_event));
    }
}

pub fn perform_respawn(
    mut events: EventReader<PerformRespawnEvent>,
    mut send_packets: EventWriter<SendPacketEvent>,
) {
    for event in events.read() {
        send_packets.send(SendPacketEvent::new(
            event.entity,
            ServerboundClientCommand {
                action: s_client_command::Action::PerformRespawn,
            },
        ));
    }
}
