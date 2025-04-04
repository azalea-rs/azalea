use std::sync::Arc;

use azalea_protocol::packets::login::ClientboundLoginPacket;
use bevy_ecs::prelude::*;

/// An event that's sent when we receive a login packet from the server. Note
/// that if you want to handle this in a system, you must add
/// `.before(azalea::packet::login::process_packet_events)` to it
/// because that system clears the events.
#[derive(Event, Debug, Clone)]
pub struct LoginPacketEvent {
    /// The client entity that received the packet.
    pub entity: Entity,
    /// The packet that was actually received.
    pub packet: Arc<ClientboundLoginPacket>,
}

#[derive(Event, Debug, Clone)]
pub struct ReceiveLoginPacketEvent {
    /// The client entity that received the packet.
    pub entity: Entity,
    /// The packet that was actually received.
    pub packet: ClientboundLoginPacket,
}
