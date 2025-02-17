use std::io::Cursor;

use azalea_protocol::{
    packets::{
        config::{ClientboundConfigPacket, ServerboundConfigPacket},
        Packet,
    },
    read::deserialize_packet,
};
use bevy_ecs::prelude::*;
use tracing::{debug, error};

use crate::{raw_connection::RawConnection, InConfigState};

#[derive(Event, Debug, Clone)]
pub struct ReceiveConfigPacketEvent {
    /// The client entity that received the packet.
    pub entity: Entity,
    /// The packet that was actually received.
    pub packet: ClientboundConfigPacket,
}

/// An event for sending a packet to the server while we're in the
/// `configuration` state.
#[derive(Event)]
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

pub fn handle_send_packet_event(
    mut send_packet_events: EventReader<SendConfigPacketEvent>,
    mut query: Query<(&mut RawConnection, Option<&InConfigState>)>,
) {
    for event in send_packet_events.read() {
        if let Ok((raw_conn, in_configuration_state)) = query.get_mut(event.sent_by) {
            if in_configuration_state.is_none() {
                error!(
                    "Tried to send a configuration packet {:?} while not in configuration state",
                    event.packet
                );
                continue;
            }
            debug!("Sending packet: {:?}", event.packet);
            if let Err(e) = raw_conn.write_packet(event.packet.clone()) {
                error!("Failed to send packet: {e}");
            }
        }
    }
}

pub fn send_packet_events(
    query: Query<(Entity, &RawConnection), With<InConfigState>>,
    mut packet_events: ResMut<Events<ReceiveConfigPacketEvent>>,
) {
    // we manually clear and send the events at the beginning of each update
    // since otherwise it'd cause issues with events in process_packet_events
    // running twice
    packet_events.clear();
    for (player_entity, raw_conn) in &query {
        let packets_lock = raw_conn.incoming_packet_queue();
        let mut packets = packets_lock.lock();
        if !packets.is_empty() {
            for raw_packet in packets.iter() {
                let packet = match deserialize_packet::<ClientboundConfigPacket>(&mut Cursor::new(
                    raw_packet,
                )) {
                    Ok(packet) => packet,
                    Err(err) => {
                        error!("failed to read packet: {err:?}");
                        debug!("packet bytes: {raw_packet:?}");
                        continue;
                    }
                };
                packet_events.send(ReceiveConfigPacketEvent {
                    entity: player_entity,
                    packet,
                });
            }
            // clear the packets right after we read them
            packets.clear();
        }
    }
}
