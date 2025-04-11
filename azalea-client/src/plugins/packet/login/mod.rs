// login packets aren't actually handled here because compression/encryption
// would make packet handling a lot messier

mod events;

use std::collections::HashSet;

use azalea_protocol::packets::{
    ConnectionProtocol, Packet,
    login::{
        ClientboundCookieRequest, ClientboundCustomQuery, ClientboundHello,
        ClientboundLoginCompression, ClientboundLoginDisconnect, ClientboundLoginFinished,
        ClientboundLoginPacket, ServerboundCookieResponse, ServerboundCustomQueryAnswer,
        ServerboundLoginAcknowledged, ServerboundLoginPacket,
    },
};
use bevy_ecs::prelude::*;
use derive_more::{Deref, DerefMut};
pub use events::*;
use tracing::{debug, error};

use super::as_system;
use crate::{
    Account, GameProfileComponent, InConfigState, connection::NetworkConnection,
    declare_packet_handlers, disconnect::DisconnectEvent,
};

pub fn process_packet(
    ecs: &mut World,
    player: Entity,
    packet: &ClientboundLoginPacket,
    state: &mut ConnectionProtocol,
    net_conn: Option<&mut NetworkConnection>,
) {
    let mut handler = LoginPacketHandler {
        player,
        ecs,
        state,
        net_conn,
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

/// A marker component for local players that are currently in the
/// `login` state.
#[derive(Component, Clone, Debug)]
pub struct InLoginState;

/// Plugins can add to this set if they want to handle a custom query packet
/// themselves. This component removed after the login state ends.
#[derive(Component, Default, Debug, Deref, DerefMut)]
pub struct IgnoreQueryIds(HashSet<u32>);

pub struct LoginPacketHandler<'a> {
    pub ecs: &'a mut World,
    pub player: Entity,
    pub state: &'a mut ConnectionProtocol,
    pub net_conn: Option<&'a mut NetworkConnection>,
}
impl LoginPacketHandler<'_> {
    pub fn hello(&mut self, p: &ClientboundHello) {
        debug!("Got encryption request {p:?}");

        as_system::<(Commands, Query<&Account>)>(self.ecs, |(mut commands, query)| {
            let Ok(account) = query.get(self.player) else {
                error!(
                    "Expected Account component to be present on player when receiving hello packet."
                );
                return;
            };
            commands.trigger_targets(
                ReceiveHelloEvent {
                    account: account.clone(),
                    packet: p.clone(),
                },
                self.player,
            );
        });
    }
    pub fn login_disconnect(&mut self, p: &ClientboundLoginDisconnect) {
        debug!("Got disconnect {:?}", p);

        as_system::<EventWriter<_>>(self.ecs, |mut events| {
            events.send(DisconnectEvent {
                entity: self.player,
                reason: Some(p.reason.clone()),
            });
        });
    }
    pub fn login_finished(&mut self, p: &ClientboundLoginFinished) {
        debug!(
            "Got profile {:?}. handshake is finished and we're now switching to the configuration state",
            p.game_profile
        );

        as_system::<Commands>(self.ecs, |mut commands| {
            commands.trigger(SendLoginPacketEvent::new(
                self.player,
                ServerboundLoginAcknowledged,
            ));

            commands
                .entity(self.player)
                .remove::<IgnoreQueryIds>()
                .remove::<InLoginState>()
                .insert(InConfigState)
                .insert(GameProfileComponent(p.game_profile.clone()));
        });

        // break (conn.config(), p.game_profile);
    }
    pub fn login_compression(&mut self, p: &ClientboundLoginCompression) {
        debug!("Got compression request {p:?}");

        if let Some(net_conn) = &mut self.net_conn {
            net_conn.set_compression_threshold(Some(p.compression_threshold as u32));
        }
    }
    pub fn custom_query(&mut self, p: &ClientboundCustomQuery) {
        debug!("Got custom query {p:?}");

        as_system::<(Commands, Query<&IgnoreQueryIds>)>(self.ecs, |(mut commands, query)| {
            let ignore_query_ids = query.get(self.player).ok().map(|x| x.0.clone());
            if let Some(ignore_query_ids) = ignore_query_ids {
                if ignore_query_ids.contains(&p.transaction_id) {
                    return;
                }
            }

            commands.trigger(SendLoginPacketEvent::new(
                self.player,
                ServerboundCustomQueryAnswer {
                    transaction_id: p.transaction_id,
                    data: None,
                },
            ));
        });
    }
    pub fn cookie_request(&mut self, p: &ClientboundCookieRequest) {
        debug!("Got cookie request {p:?}");

        as_system::<Commands>(self.ecs, |mut commands| {
            commands.trigger(SendLoginPacketEvent::new(
                self.player,
                ServerboundCookieResponse {
                    key: p.key.clone(),
                    // cookies aren't implemented
                    payload: None,
                },
            ));
        });
    }
}
