use azalea_entity::metadata::Health;
use bevy_app::{App, Plugin, Update};
use bevy_ecs::{
    prelude::*,
    system::{SystemParam, SystemState},
};

use self::game::DeathEvent;
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
        app.add_observer(game::handle_outgoing_packets_observer)
            .add_observer(config::handle_outgoing_packets_observer)
            .add_observer(login::handle_outgoing_packets_observer)
            .add_systems(
                Update,
                (
                    (
                        config::handle_outgoing_packets,
                        game::handle_outgoing_packets,
                        login::handle_outgoing_packets,
                    )
                        .chain(),
                    death_event_on_0_health.before(death_listener),
                ),
            )
            .add_event::<game::ReceiveGamePacketEvent>()
            .add_event::<config::ReceiveConfigPacketEvent>()
            .add_event::<login::ReceiveLoginPacketEvent>()
            //
            .add_event::<game::SendPacketEvent>()
            .add_event::<config::SendConfigPacketEvent>()
            .add_event::<login::SendLoginPacketEvent>()
            //
            .add_event::<game::AddPlayerEvent>()
            .add_event::<game::RemovePlayerEvent>()
            .add_event::<game::UpdatePlayerEvent>()
            .add_event::<ChatReceivedEvent>()
            .add_event::<game::DeathEvent>()
            .add_event::<game::KeepAliveEvent>()
            .add_event::<game::ResourcePackEvent>()
            .add_event::<game::InstanceLoadedEvent>()
            .add_event::<login::ReceiveCustomQueryEvent>();
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
