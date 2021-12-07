mod client_intention_packet;
pub use client_intention_packet::ClientIntentionPacket;
mod serverbound_status_request_packet;
pub use serverbound_status_request_packet::ServerboundStatusRequestPacket;
use tokio::io::AsyncRead;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConnectionProtocol {
    Handshaking = -1,
    Play = 0,
    Status = 1,
    Login = 2,
}

pub trait Packet {
    /// Get the id of the packet, this is always a byte.
    fn get_id(&self) -> u32;

    fn write(&self, buf: &mut Vec<u8>) -> ();
    fn parse<T: AsyncRead + std::marker::Unpin>(&self, buf: T) -> ();
}
