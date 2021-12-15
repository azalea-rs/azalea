pub mod client_intention_packet;

use std::f32::consts::E;

use async_trait::async_trait;
use tokio::io::BufReader;

use crate::connect::PacketFlow;

use super::ProtocolPacket;

#[derive(Clone, Debug)]
pub enum HandshakePacket
where
    Self: Sized,
{
    ClientIntentionPacket(client_intention_packet::ClientIntentionPacket),
}

#[async_trait]
impl ProtocolPacket for HandshakePacket {
    fn id(&self) -> u32 {
        match self {
            HandshakePacket::ClientIntentionPacket(_packet) => 0x00,
        }
    }

    fn write(&self, buf: &mut Vec<u8>) {
        match self {
            HandshakePacket::ClientIntentionPacket(packet) => packet.write(buf),
        }
    }

    /// Read a packet by its id, ConnectionProtocol, and flow
    async fn read<T: tokio::io::AsyncRead + std::marker::Unpin + std::marker::Send>(
        id: u32,
        flow: &PacketFlow,
        buf: &mut BufReader<T>,
    ) -> Result<HandshakePacket, String>
    where
        Self: Sized,
    {
        match flow {
            PacketFlow::ServerToClient => Err("HandshakePacket::read not implemented".to_string()),
            PacketFlow::ClientToServer => match id {
                0x00 => Ok(client_intention_packet::ClientIntentionPacket::read(buf).await?),
                _ => Err(format!("Unknown ClientToServer status packet id: {}", id)),
            },
        }
    }
}
