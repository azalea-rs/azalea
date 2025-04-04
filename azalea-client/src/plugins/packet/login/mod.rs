// login packets aren't actually handled here because compression/encryption
// would make packet handling a lot messier

mod events;

use std::collections::HashSet;

use azalea_protocol::packets::{
    Packet,
    login::{
        ClientboundCookieRequest, ClientboundCustomQuery, ClientboundHello,
        ClientboundLoginCompression, ClientboundLoginDisconnect, ClientboundLoginFinished,
        ClientboundLoginPacket, ServerboundCustomQueryAnswer, ServerboundLoginPacket,
    },
};
use bevy_ecs::{prelude::*, system::SystemState};
use derive_more::{Deref, DerefMut};
pub use events::*;
use tokio::sync::mpsc;
use tracing::error;

use super::as_system;
use crate::declare_packet_handlers;

/// Event for sending a login packet to the server.
#[derive(Event)]
pub struct SendLoginPacketEvent {
    pub entity: Entity,
    pub packet: ServerboundLoginPacket,
}
impl SendLoginPacketEvent {
    pub fn new(entity: Entity, packet: impl Packet<ServerboundLoginPacket>) -> Self {
        let packet = packet.into_variant();
        Self { entity, packet }
    }
}

#[derive(Component)]
pub struct LoginSendPacketQueue {
    pub tx: mpsc::UnboundedSender<ServerboundLoginPacket>,
}

/// A marker component for local players that are currently in the
/// `login` state.
#[derive(Component, Clone, Debug)]
pub struct InLoginState;

pub fn handle_send_packet_event(
    mut send_packet_events: EventReader<SendLoginPacketEvent>,
    mut query: Query<&mut LoginSendPacketQueue>,
) {
    for event in send_packet_events.read() {
        if let Ok(queue) = query.get_mut(event.entity) {
            let _ = queue.tx.send(event.packet.clone());
        } else {
            error!("Sent SendPacketEvent for entity that doesn't have a LoginSendPacketQueue");
        }
    }
}

/// Plugins can add to this set if they want to handle a custom query packet
/// themselves. This component removed after the login state ends.
#[derive(Component, Default, Debug, Deref, DerefMut)]
pub struct IgnoreQueryIds(HashSet<u32>);

pub fn process_packet_events(ecs: &mut World) {
    let mut events_owned = Vec::new();
    let mut system_state: SystemState<EventReader<ReceiveLoginPacketEvent>> = SystemState::new(ecs);
    let mut events = system_state.get_mut(ecs);
    for ReceiveLoginPacketEvent {
        entity: player_entity,
        packet,
    } in events.read()
    {
        // we do this so `ecs` isn't borrowed for the whole loop
        events_owned.push((*player_entity, packet.clone()));
    }
    for (player_entity, packet) in events_owned {
        let mut handler = LoginPacketHandler {
            player: player_entity,
            ecs,
        };

        declare_packet_handlers!(
            ClientboundLoginPacket,
            packet,
            handler,
            [
                hello,
                login_disconnect,
                login_finished,
                login_compression,
                custom_query,
                cookie_request
            ]
        );
    }
}

pub struct LoginPacketHandler<'a> {
    pub ecs: &'a mut World,
    pub player: Entity,
}
impl LoginPacketHandler<'_> {
    pub fn hello(&mut self, _p: ClientboundHello) {}
    pub fn login_disconnect(&mut self, _p: ClientboundLoginDisconnect) {}
    pub fn login_finished(&mut self, _p: ClientboundLoginFinished) {}
    pub fn login_compression(&mut self, _p: ClientboundLoginCompression) {
        // as_system::<Query<&mut RawConnection>>(self.ecs, |mut query| {
        //     if let Ok(mut raw_conn) = query.get_mut(self.player) {
        //         raw_conn.set_compression_threshold(p.compression_threshold);
        //     }
        // });
    }
    pub fn custom_query(&mut self, p: ClientboundCustomQuery) {
        as_system::<(EventWriter<SendLoginPacketEvent>, Query<&IgnoreQueryIds>)>(
            self.ecs,
            |(mut events, query)| {
                let ignore_query_ids = query.get(self.player).ok().map(|x| x.0.clone());
                if let Some(ignore_query_ids) = ignore_query_ids {
                    if ignore_query_ids.contains(&p.transaction_id) {
                        return;
                    }
                }

                events.send(SendLoginPacketEvent::new(
                    self.player,
                    ServerboundCustomQueryAnswer {
                        transaction_id: p.transaction_id,
                        data: None,
                    },
                ));
            },
        );
    }
    pub fn cookie_request(&mut self, _p: ClientboundCookieRequest) {}
}
