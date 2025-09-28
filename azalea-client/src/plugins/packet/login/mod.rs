// login packets aren't actually handled here because compression/encryption
// would make packet handling a lot messier

mod events;

use azalea_protocol::packets::{
    ConnectionProtocol,
    login::{
        ClientboundCookieRequest, ClientboundCustomQuery, ClientboundHello,
        ClientboundLoginCompression, ClientboundLoginDisconnect, ClientboundLoginFinished,
        ClientboundLoginPacket, ServerboundCookieResponse, ServerboundLoginAcknowledged,
    },
};
use bevy_ecs::prelude::*;
pub use events::*;
use tracing::{debug, error};

use super::as_system;
use crate::{
    Account, InConfigState, connection::RawConnection, disconnect::DisconnectEvent,
    packet::declare_packet_handlers, player::GameProfileComponent,
};

pub fn process_packet(ecs: &mut World, player: Entity, packet: &ClientboundLoginPacket) {
    let mut handler = LoginPacketHandler { player, ecs };

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

/// A marker component for local players that are currently in the
/// `login` state.
#[derive(Component, Clone, Debug)]
pub struct InLoginState;

pub struct LoginPacketHandler<'a> {
    pub ecs: &'a mut World,
    pub player: Entity,
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
            commands.trigger(ReceiveHelloEvent {
                entity: self.player,
                account: account.clone(),
                packet: p.clone(),
            });
        });
    }
    pub fn login_disconnect(&mut self, p: &ClientboundLoginDisconnect) {
        debug!("Got disconnect {:?}", p);

        as_system::<MessageWriter<_>>(self.ecs, |mut events| {
            events.write(DisconnectEvent {
                entity: self.player,
                reason: Some(p.reason.clone()),
            });
        });
    }
    pub fn login_finished(&mut self, p: &ClientboundLoginFinished) {
        debug!(
            "Got profile {:?}. login is finished and we're now switching to the config state",
            p.game_profile
        );

        as_system::<(Commands, Query<&mut RawConnection>)>(
            self.ecs,
            |(mut commands, mut query)| {
                commands.trigger(SendLoginPacketEvent::new(
                    self.player,
                    ServerboundLoginAcknowledged,
                ));

                commands
                    .entity(self.player)
                    .remove::<InLoginState>()
                    .insert(InConfigState)
                    .insert(GameProfileComponent(p.game_profile.clone()));

                let mut conn = query
                    .get_mut(self.player)
                    .expect("RawConnection component should be present when receiving packets");
                conn.state = ConnectionProtocol::Configuration;
            },
        );
    }
    pub fn login_compression(&mut self, p: &ClientboundLoginCompression) {
        debug!("Got compression request {p:?}");

        as_system::<Query<&mut RawConnection>>(self.ecs, |mut query| {
            let mut conn = query
                .get_mut(self.player)
                .expect("RawConnection component should be present when receiving packets");
            if let Some(net_conn) = &mut conn.net_conn() {
                net_conn.set_compression_threshold(Some(p.compression_threshold as u32));
            }
        })
    }
    pub fn custom_query(&mut self, p: &ClientboundCustomQuery) {
        debug!("Got custom query {p:?}");

        as_system::<MessageWriter<ReceiveCustomQueryEvent>>(self.ecs, |mut events| {
            events.write(ReceiveCustomQueryEvent {
                entity: self.player,
                packet: p.clone(),
                disabled: false,
            });
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
