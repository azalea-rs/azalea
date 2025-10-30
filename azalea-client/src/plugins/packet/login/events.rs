use std::sync::Arc;

use azalea_protocol::packets::{
    Packet,
    login::{
        ClientboundCustomQuery, ClientboundHello, ClientboundLoginPacket, ServerboundLoginPacket,
    },
};
use bevy_ecs::prelude::*;
use tracing::{debug, error};

use super::InLoginState;
use crate::{Account, connection::RawConnection};

#[derive(Message, Debug, Clone)]
pub struct ReceiveLoginPacketEvent {
    /// The client entity that received the packet.
    pub entity: Entity,
    /// The packet that was actually received.
    pub packet: Arc<ClientboundLoginPacket>,
}

#[derive(EntityEvent, Debug, Clone)]
pub struct ReceiveHelloEvent {
    pub entity: Entity,
    pub account: Account,
    pub packet: ClientboundHello,
}

#[derive(Message, Debug, Clone)]
pub struct ReceiveCustomQueryEvent {
    /// The client entity that received the packet.
    pub entity: Entity,
    pub packet: ClientboundCustomQuery,
    /// A system can set this to `true` to make Azalea not reply to the query.
    /// You must make sure you modify this before the
    /// [`reply_to_custom_queries`] system runs.
    ///
    /// [`reply_to_custom_queries`]: crate::login::reply_to_custom_queries
    pub disabled: bool,
}

/// Event for sending a login packet to the server.
#[derive(EntityEvent, Debug, Clone)]
pub struct SendLoginPacketEvent {
    #[event_target]
    pub sent_by: Entity,
    pub packet: ServerboundLoginPacket,
}
impl SendLoginPacketEvent {
    pub fn new(entity: Entity, packet: impl Packet<ServerboundLoginPacket>) -> Self {
        let packet = packet.into_variant();
        Self {
            sent_by: entity,
            packet,
        }
    }
}

pub fn handle_outgoing_packets_observer(
    trigger: On<SendLoginPacketEvent>,
    mut query: Query<(&mut RawConnection, Option<&InLoginState>)>,
) {
    let event = trigger.event();
    if let Ok((mut raw_conn, in_login_state)) = query.get_mut(event.sent_by) {
        if in_login_state.is_none() {
            error!(
                "Tried to send a login packet {:?} while not in login state",
                event.packet
            );
            return;
        }
        debug!("Sending login packet: {:?}", event.packet);
        if let Err(e) = raw_conn.write(event.packet.clone()) {
            error!("Failed to send packet: {e}");
        }
    }
}
