pub mod game;
pub mod handshake;
pub mod login;
pub mod status;

use async_trait::async_trait;
use tokio::io::BufReader;

use crate::connect::PacketFlow;

pub const PROTOCOL_VERSION: u32 = 757;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConnectionProtocol {
    Handshake = -1,
    Game = 0,
    Status = 1,
    Login = 2,
}

#[derive(Clone, Debug)]
pub enum Packet {
    Game(game::GamePacket),
    Handshake(handshake::HandshakePacket),
    Login(login::LoginPacket),
    Status(Box<status::StatusPacket>),
}

/// An enum of packets for a certain protocol
#[async_trait]
pub trait ProtocolPacket
where
    Self: Sized,
{
    fn id(&self) -> u32;

    /// Read a packet by its id, ConnectionProtocol, and flow
    async fn read<T: tokio::io::AsyncRead + std::marker::Unpin + std::marker::Send>(
        id: u32,
        flow: &PacketFlow,
        buf: &mut T,
    ) -> Result<Self, String>
    where
        Self: Sized;

    fn write(&self, buf: &mut Vec<u8>);
}
