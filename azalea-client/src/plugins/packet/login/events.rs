use std::sync::Arc;

use azalea_protocol::packets::login::{ClientboundHello, ClientboundLoginPacket};
use bevy_ecs::prelude::*;

use crate::Account;

#[derive(Event, Debug, Clone)]
pub struct ReceiveLoginPacketEvent {
    /// The client entity that received the packet.
    pub entity: Entity,
    /// The packet that was actually received.
    pub packet: Arc<ClientboundLoginPacket>,
}

#[derive(Event)]
pub struct ReceiveHelloEvent {
    pub account: Account,
    pub packet: ClientboundHello,
}
