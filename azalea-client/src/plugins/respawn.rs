use azalea_protocol::packets::game::s_client_command::{self, ServerboundClientCommand};
use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::*;

use crate::packet::game::SendGamePacketEvent;

/// Tell the server that we're respawning.
#[derive(Message, Debug, Clone)]
pub struct PerformRespawnEvent {
    pub entity: Entity,
}

/// A plugin that makes [`PerformRespawnEvent`] send the packet to respawn.
pub struct RespawnPlugin;
impl Plugin for RespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<PerformRespawnEvent>()
            .add_systems(Update, perform_respawn);
    }
}

pub fn perform_respawn(mut events: MessageReader<PerformRespawnEvent>, mut commands: Commands) {
    for event in events.read() {
        commands.trigger(SendGamePacketEvent::new(
            event.entity,
            ServerboundClientCommand {
                action: s_client_command::Action::PerformRespawn,
            },
        ));
    }
}
