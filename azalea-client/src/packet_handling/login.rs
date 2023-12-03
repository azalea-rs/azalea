// login packets aren't actually handled here because compression/encryption
// would make packet handling a lot messier

use std::sync::Arc;

use azalea_protocol::packets::login::{ClientboundLoginPacket, ServerboundLoginPacket};
use bevy_ecs::prelude::*;
use tokio::sync::mpsc;
use tracing::error;

// this struct is defined here anyways though so it's consistent with the other
// ones

#[derive(Event, Debug, Clone)]
pub struct LoginPacketEvent {
    /// The client entity that received the packet.
    pub entity: Entity,
    /// The packet that was actually received.
    pub packet: Arc<ClientboundLoginPacket>,
}

/// Event for sending a login packet to the server.
#[derive(Event)]
pub struct SendLoginPacketEvent {
    pub entity: Entity,
    pub packet: ServerboundLoginPacket,
}

#[derive(Component)]
pub struct LoginSendPacketQueue {
    pub tx: mpsc::UnboundedSender<ServerboundLoginPacket>,
}

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
