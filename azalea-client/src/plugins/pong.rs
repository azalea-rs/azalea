use azalea_protocol::packets::{config, game};
use bevy_app::{App, Plugin};
use bevy_ecs::prelude::*;

use super::packet::{
    config::{ConfigPingEvent, SendConfigPacketEvent},
    game::GamePingEvent,
};
use crate::packet::game::SendGamePacketEvent;

/// A plugin that replies to [`ClientboundPing`] packets with
/// [`ServerboundPong`].
///
/// This works in both the `game` and `config` states.
///
/// [`ClientboundPing`]: azalea_protocol::packets::game::ClientboundPing
/// [`ServerboundPong`]: azalea_protocol::packets::game::ServerboundPong
pub struct PongPlugin;
impl Plugin for PongPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(reply_to_game_ping)
            .add_observer(reply_to_config_ping);
    }
}

pub fn reply_to_game_ping(ping: On<GamePingEvent>, mut commands: Commands) {
    commands.trigger(SendGamePacketEvent::new(
        ping.entity,
        game::ServerboundPong { id: ping.packet.id },
    ));
}

pub fn reply_to_config_ping(ping: On<ConfigPingEvent>, mut commands: Commands) {
    commands.trigger(SendConfigPacketEvent::new(
        ping.entity,
        config::ServerboundPong { id: ping.packet.id },
    ));
}
