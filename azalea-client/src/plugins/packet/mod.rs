use azalea_entity::metadata::Health;
use bevy_app::{App, Plugin, Update};
use bevy_ecs::{
    prelude::*,
    system::{SystemParam, SystemState},
};

use self::game::DeathEvent;
use crate::chat::ChatReceivedEvent;

pub mod config;
pub mod game;
pub mod login;
pub mod relative_updates;

pub struct PacketPlugin;

pub fn death_event_on_0_health(
    query: Query<(Entity, &Health), Changed<Health>>,
    mut death_events: MessageWriter<DeathEvent>,
) {
    for (entity, health) in query.iter() {
        if **health == 0. {
            death_events.write(DeathEvent {
                entity,
                packet: None,
            });
        }
    }
}

impl Plugin for PacketPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            relative_updates::debug_detect_updates_received_on_local_entities,
        )
        .add_observer(game::handle_outgoing_packets_observer)
        .add_observer(config::handle_outgoing_packets_observer)
        .add_observer(login::handle_outgoing_packets_observer)
        .add_systems(Update, death_event_on_0_health)
        .add_message::<game::ReceiveGamePacketEvent>()
        .add_message::<config::ReceiveConfigPacketEvent>()
        .add_message::<login::ReceiveLoginPacketEvent>()
        //
        .add_message::<game::AddPlayerEvent>()
        .add_message::<game::RemovePlayerEvent>()
        .add_message::<game::UpdatePlayerEvent>()
        .add_message::<ChatReceivedEvent>()
        .add_message::<game::DeathEvent>()
        .add_message::<game::KeepAliveEvent>()
        .add_message::<game::ResourcePackEvent>()
        .add_message::<game::WorldLoadedEvent>()
        .add_message::<login::ReceiveCustomQueryEvent>();
    }
}

#[doc(hidden)]
macro_rules! __declare_packet_handlers {
    (
        $packetenum:ident,
        $packetvar:expr,
        $handler:ident,
        [$($packet:path),+ $(,)?]
    ) => {
        pastey::paste! {
           match $packetvar {
                $(
                    $packetenum::[< $packet:camel >](p) => $handler.$packet(p),
                )+
            }
        }
    };
}

pub(crate) use __declare_packet_handlers as declare_packet_handlers;

#[derive(Resource)]
struct CachedSystemState<T: SystemParam + 'static>(SystemState<T>);

pub(crate) fn as_system<T>(ecs: &mut World, f: impl FnOnce(T::Item<'_, '_>))
where
    T: SystemParam + 'static,
{
    // creating a new SystemState is expensive, so we save them as a Resource in the
    // ecs
    let mut system_state = match ecs.remove_resource::<CachedSystemState<T>>() {
        Some(s) => s.0,
        None => SystemState::<T>::new(ecs),
    };
    let values = system_state.get_mut(ecs);
    f(values);
    system_state.apply(ecs);
    ecs.insert_resource(CachedSystemState(system_state));
}
