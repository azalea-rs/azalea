pub mod clientbound_login_packet;

use super::ProtocolPacket;
use crate::connect::PacketFlow;
use async_trait::async_trait;
use tokio::io::BufReader;

#[derive(Clone, Debug)]
pub enum GamePacket
where
    Self: Sized,
{
    ClientboundLoginPacket(clientbound_login_packet::ClientboundLoginPacket),
}

#[async_trait]
impl ProtocolPacket for GamePacket {
    fn id(&self) -> u32 {
        match self {
            GamePacket::ClientboundLoginPacket(_packet) => 0x00,
        }
    }

    fn write(&self, _buf: &mut Vec<u8>) {}

    /// Read a packet by its id, ConnectionProtocol, and flow
    async fn read<T: tokio::io::AsyncRead + std::marker::Unpin + std::marker::Send>(
        _id: u32,
        flow: &PacketFlow,
        _buf: &mut T,
    ) -> Result<GamePacket, String>
    where
        Self: Sized,
    {
        match flow {
            PacketFlow::ServerToClient => Err("HandshakePacket::read not implemented".to_string()),
            PacketFlow::ClientToServer => Err("HandshakePacket::read not implemented".to_string()),
        }
    }
}
