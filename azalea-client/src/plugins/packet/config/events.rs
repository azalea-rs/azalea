use std::sync::Arc;

use azalea_protocol::packets::{
    Packet,
    config::{ClientboundConfigPacket, ServerboundConfigPacket},
};
use bevy_ecs::prelude::*;
use tracing::{debug, error};

use crate::{InConfigState, connection::RawConnection};

#[derive(Message, Debug, Clone)]
pub struct ReceiveConfigPacketEvent {
    /// The client entity that received the packet.
    pub entity: Entity,
    /// The packet that was actually received.
    pub packet: Arc<ClientboundConfigPacket>,
}

/// An event for sending a packet to the server while we're in the
/// `configuration` state.
#[derive(EntityEvent, Clone)]
pub struct SendConfigPacketEvent {
    #[event_target]
    pub sent_by: Entity,
    pub packet: ServerboundConfigPacket,
}
impl SendConfigPacketEvent {
    pub fn new(sent_by: Entity, packet: impl Packet<ServerboundConfigPacket>) -> Self {
        let packet = packet.into_variant();
        Self { sent_by, packet }
    }
}

pub fn handle_outgoing_packets_observer(
    send_config_packet: On<SendConfigPacketEvent>,
    mut query: Query<(&mut RawConnection, Option<&InConfigState>)>,
) {
    if let Ok((mut raw_conn, in_configuration_state)) = query.get_mut(send_config_packet.sent_by) {
        if in_configuration_state.is_none() {
            error!(
                "Tried to send a configuration packet {:?} while not in configuration state",
                send_config_packet.packet
            );
            return;
        }
        debug!("Sending config packet: {:?}", send_config_packet.packet);
        if let Err(e) = raw_conn.write(send_config_packet.packet.clone()) {
            error!("Failed to send packet: {e}");
        }
    }
}

/// A Bevy trigger that's sent when our client receives a [`ClientboundPing`]
/// packet in the config state.
///
/// Also see [`GamePingEvent`].
///
/// [`ClientboundPing`]: azalea_protocol::packets::config::ClientboundPing
/// [`GamePingEvent`]: crate::packet::game::GamePingEvent
#[derive(Event, Debug, Clone)]
pub struct ConfigPingEvent {
    pub entity: Entity,
    pub packet: azalea_protocol::packets::config::ClientboundPing,
}
