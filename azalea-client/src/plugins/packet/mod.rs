use azalea_entity::{EntityUpdateSet, metadata::Health};
use bevy_app::{App, First, Plugin, PreUpdate, Update};
use bevy_ecs::{
    prelude::*,
    system::{SystemParam, SystemState},
};

use self::{
    game::{
        AddPlayerEvent, DeathEvent, InstanceLoadedEvent, KeepAliveEvent, RemovePlayerEvent,
        ResourcePackEvent, UpdatePlayerEvent,
    },
    login::{LoginPacketEvent, SendLoginPacketEvent},
};
use crate::{chat::ChatReceivedEvent, events::death_listener};

pub mod config;
pub mod game;
pub mod login;

pub struct PacketPlugin;

pub fn death_event_on_0_health(
    query: Query<(Entity, &Health), Changed<Health>>,
    mut death_events: EventWriter<DeathEvent>,
) {
    for (entity, health) in query.iter() {
        if **health == 0. {
            death_events.send(DeathEvent {
                entity,
                packet: None,
            });
        }
    }
}

impl Plugin for PacketPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            First,
            (
                game::emit_receive_packet_events,
                config::emit_receive_config_packet_events,
            ),
        )
        .add_systems(
            PreUpdate,
            (
                game::process_packet_events
                    // we want to index and deindex right after
                    .before(EntityUpdateSet::Deindex),
                config::process_packet_events,
                login::handle_send_packet_event,
                login::process_packet_events,
            ),
        )
        .add_systems(
            Update,
            (
                (
                    config::handle_send_packet_event,
                    game::handle_outgoing_packets,
                )
                    .chain(),
                death_event_on_0_health.before(death_listener),
            ),
        )
        // we do this instead of add_event so we can handle the events ourselves
        .init_resource::<Events<game::ReceivePacketEvent>>()
        .init_resource::<Events<config::ReceiveConfigPacketEvent>>()
        .add_event::<game::SendPacketEvent>()
        .add_event::<config::SendConfigPacketEvent>()
        .add_event::<AddPlayerEvent>()
        .add_event::<RemovePlayerEvent>()
        .add_event::<UpdatePlayerEvent>()
        .add_event::<ChatReceivedEvent>()
        .add_event::<DeathEvent>()
        .add_event::<KeepAliveEvent>()
        .add_event::<ResourcePackEvent>()
        .add_event::<InstanceLoadedEvent>()
        .add_event::<LoginPacketEvent>()
        .add_event::<SendLoginPacketEvent>();
    }
}

#[macro_export]
macro_rules! declare_packet_handlers {
    (
        $packetenum:ident,
        $packetvar:expr,
        $handler:ident,
        [$($packet:path),+ $(,)?]
    ) => {
        paste::paste! {
           match $packetvar {
                $(
                    $packetenum::[< $packet:camel >](p) => $handler.$packet(p),
                )+
            }
        }
    };
}

pub(crate) fn as_system<T>(ecs: &mut World, f: impl FnOnce(T::Item<'_, '_>))
where
    T: SystemParam + 'static,
{
    let mut system_state = SystemState::<T>::new(ecs);
    let values = system_state.get_mut(ecs);
    f(values);
    system_state.apply(ecs);
}
