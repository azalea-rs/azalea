mod game;
mod handshake;
mod login;
mod status;

use async_trait::async_trait;
use tokio::io::AsyncRead;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConnectionProtocol {
    Handshake = -1,
    Game = 0,
    Status = 1,
    Login = 2,
}

pub enum Packet<'a> {
    // game
    // handshake
    ClientIntentionPacket(&'a handshake::client_intention_packet::ClientIntentionPacket<'a>),
    // login
    // status
    ServerboundStatusRequestPacket(
        &'a status::serverbound_status_request_packet::ServerboundStatusRequestPacket,
    ),
    ClientboundStatusRequestPacket(
        &'a status::clientbound_status_response_packet::ClientboundStatusRequestPacket,
    ),
}

pub trait PacketTrait {
    /// Return a version of the packet that you can actually use for stuff
    fn get(&self) -> Packet;
    fn write(&self, buf: &mut Vec<u8>) -> ();
    fn parse<T: AsyncRead + std::marker::Unpin>(
        buf: &mut T,
        // is using a static lifetime here a good idea? idk
    ) -> Result<Packet<'_>, String>;
}
