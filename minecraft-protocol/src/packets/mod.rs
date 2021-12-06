mod client_intention_packet;
pub use client_intention_packet::ClientIntentionPacket;
mod serverbound_status_request_packet;
pub use serverbound_status_request_packet::ServerboundStatusRequestPacket;

use crate::mc_buf;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConnectionProtocol {
    Handshaking = -1,
    Play = 0,
    Status = 1,
    Login = 2,
}

pub trait Packet {
    /// Get the id of the packet, this is always a byte.
    fn get_id(&self) -> u8;

    fn write(&self, friendly_byte_buf: &mut Vec<u8>) -> ();
}
