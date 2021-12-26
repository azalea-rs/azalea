pub mod clientbound_login_packet;

use super::ProtocolPacket;
use crate::connect::PacketFlow;
use async_trait::async_trait;

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
        id: u32,
        flow: &PacketFlow,
        buf: &mut T,
    ) -> Result<GamePacket, String>
    where
        Self: Sized,
    {
        Ok(match flow {
            PacketFlow::ServerToClient => match id {
                0x26 => clientbound_login_packet::ClientboundLoginPacket::read(buf).await?,

                _ => return Err(format!("Unknown ServerToClient game packet id: {}", id)),
            },
            PacketFlow::ClientToServer => match id {
                // 0x00 => serverbound_hello_packet::ServerboundHelloPacket::read(buf).await?,
                _ => return Err(format!("Unknown ClientToServer game packet id: {}", id)),
            },
        })
    }
}
