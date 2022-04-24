pub mod game;
pub mod handshake;
pub mod login;
pub mod status;

use crate::{
    connect::PacketFlow,
    mc_buf::{McBufReadable, McBufWritable, Readable, Writable},
};
use async_trait::async_trait;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use tokio::io::AsyncRead;

pub const PROTOCOL_VERSION: u32 = 758;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
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

    fn write(&self, buf: &mut Vec<u8>) -> Result<(), std::io::Error>;
}

#[async_trait]
impl McBufReadable for ConnectionProtocol {
    async fn read_into<R>(buf: &mut R) -> Result<Self, String>
    where
        R: AsyncRead + std::marker::Unpin + std::marker::Send,
    {
        ConnectionProtocol::from_i32(buf.read_varint().await?)
            .ok_or_else(|| "Invalid intention".to_string())
    }
}

impl McBufWritable for ConnectionProtocol {
    fn write_into(&self, buf: &mut Vec<u8>) -> Result<(), std::io::Error> {
        buf.write_varint(*self as i32)
    }
}
