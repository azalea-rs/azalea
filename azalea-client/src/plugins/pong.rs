use bevy_app::{App, Plugin};
use bevy_ecs::prelude::*;

use super::packet::{
    config::{ConfigPingEvent, SendConfigPacketEvent},
    game::PingEvent,
};
use crate::packet::game::SendPacketEvent;

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

pub fn reply_to_game_ping(trigger: Trigger<PingEvent>, mut commands: Commands) {
    commands.trigger(SendPacketEvent::new(
        trigger.target(),
        azalea_protocol::packets::game::ServerboundPong { id: trigger.0.id },
    ));
}

pub fn reply_to_config_ping(trigger: Trigger<ConfigPingEvent>, mut commands: Commands) {
    commands.trigger(SendConfigPacketEvent::new(
        trigger.target(),
        azalea_protocol::packets::config::ServerboundPong { id: trigger.0.id },
    ));
}
