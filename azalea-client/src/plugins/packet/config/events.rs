use std::io::Cursor;

use azalea_protocol::{
    packets::{
        Packet,
        config::{ClientboundConfigPacket, ServerboundConfigPacket},
    },
    read::deserialize_packet,
};
use bevy_ecs::prelude::*;
use tracing::{debug, error};

use crate::{InConfigState, raw_connection::RawConnection};

#[derive(Event, Debug, Clone)]
pub struct ReceiveConfigPacketEvent {
    /// The client entity that received the packet.
    pub entity: Entity,
    /// The packet that was actually received.
    pub packet: ClientboundConfigPacket,
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
    if let Ok((raw_conn, in_configuration_state)) = query.get_mut(event.sent_by) {
        if in_configuration_state.is_none() {
            error!(
                "Tried to send a configuration packet {:?} while not in configuration state",
                event.packet
            );
            return;
        }
        debug!("Sending packet: {:?}", event.packet);
        if let Err(e) = raw_conn.write_packet(event.packet.clone()) {
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

pub fn emit_receive_config_packet_events(
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
            let mut packets_read = 0;
            for raw_packet in packets.iter() {
                packets_read += 1;
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

                let should_interrupt = packet_interrupts(&packet);

                packet_events.send(ReceiveConfigPacketEvent {
                    entity: player_entity,
                    packet,
                });

                if should_interrupt {
                    break;
                }
            }
            packets.drain(0..packets_read);
        }
    }
}

/// Whether the given packet should make us stop deserializing the received
/// packets until next update.
///
/// This is used for packets that can switch the client state.
fn packet_interrupts(packet: &ClientboundConfigPacket) -> bool {
    matches!(
        packet,
        ClientboundConfigPacket::FinishConfiguration(_)
            | ClientboundConfigPacket::Disconnect(_)
            | ClientboundConfigPacket::Transfer(_)
    )
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
