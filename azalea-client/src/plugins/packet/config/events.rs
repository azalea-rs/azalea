use std::sync::Arc;

use azalea_protocol::packets::{
    Packet,
    config::{ClientboundConfigPacket, ServerboundConfigPacket},
};
use bevy_ecs::prelude::*;
use tracing::{debug, error};

use crate::{InConfigState, connection::RawConnection};

#[derive(Event, Debug, Clone)]
pub struct ReceiveConfigPacketEvent {
    /// The client entity that received the packet.
    pub entity: Entity,
    /// The packet that was actually received.
    pub packet: Arc<ClientboundConfigPacket>,
}

/// An event for sending a packet to the server while we're in the
/// `configuration` state.
#[derive(Event, Clone)]
pub struct SendConfigPacketEvent {
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
    trigger: Trigger<SendConfigPacketEvent>,
    mut query: Query<(&mut RawConnection, Option<&InConfigState>)>,
) {
    let event = trigger.event();
    if let Ok((mut raw_conn, in_configuration_state)) = query.get_mut(event.sent_by) {
        if in_configuration_state.is_none() {
            error!(
                "Tried to send a configuration packet {:?} while not in configuration state",
                event.packet
            );
            return;
        }
        debug!("Sending config packet: {:?}", event.packet);
        if let Err(e) = raw_conn.write(event.packet.clone()) {
            error!("Failed to send packet: {e}");
        }
    }
}
/// A system that converts [`SendConfigPacketEvent`] events into triggers so
/// they get received by [`handle_outgoing_packets_observer`].
pub fn handle_outgoing_packets(
    mut commands: Commands,
    mut events: EventReader<SendConfigPacketEvent>,
) {
    for event in events.read() {
        commands.trigger(event.clone());
    }
}

/// A Bevy trigger that's sent when our client receives a [`ClientboundPing`]
/// packet in the config state.
///
/// See [`PingEvent`] for more information.
///
/// [`ClientboundPing`]: azalea_protocol::packets::config::ClientboundPing
/// [`PingEvent`]: crate::packet::game::PingEvent
#[derive(Event, Debug, Clone)]
pub struct ConfigPingEvent(pub azalea_protocol::packets::config::ClientboundPing);
