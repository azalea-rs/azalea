use azalea_entity::EntityUpdateSet;
use bevy_app::{App, First, Plugin, PreUpdate, Update};
use bevy_ecs::prelude::*;

use crate::chat::ChatReceivedEvent;

use self::game::{
    AddPlayerEvent, DeathEvent, KeepAliveEvent, RemovePlayerEvent, UpdatePlayerEvent,
};

pub mod configuration;
pub mod game;

pub struct PacketHandlerPlugin;

impl Plugin for PacketHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            First,
            (game::send_packet_events, configuration::send_packet_events),
        )
        .add_systems(
            PreUpdate,
            (
                game::process_packet_events,
                configuration::process_packet_events,
            )
                // we want to index and deindex right after
                .before(EntityUpdateSet::Deindex),
        )
        .add_systems(Update, game::death_event_on_0_health)
        // we do this instead of add_event so we can handle the events ourselves
        .init_resource::<Events<game::PacketEvent>>()
        .init_resource::<Events<configuration::PacketEvent>>()
        .add_event::<AddPlayerEvent>()
        .add_event::<RemovePlayerEvent>()
        .add_event::<UpdatePlayerEvent>()
        .add_event::<ChatReceivedEvent>()
        .add_event::<DeathEvent>()
        .add_event::<KeepAliveEvent>();
    }
}
