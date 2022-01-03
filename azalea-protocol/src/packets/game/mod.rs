pub mod clientbound_change_difficulty_packet;
pub mod clientbound_custom_payload_packet;
pub mod clientbound_login_packet;
pub mod clientbound_update_view_distance_packet;

use super::ProtocolPacket;
use crate::connect::PacketFlow;
use async_trait::async_trait;

#[derive(Clone, Debug)]
pub enum GamePacket
where
    Self: Sized,
{
    ClientboundLoginPacket(clientbound_login_packet::ClientboundLoginPacket),
    ClientboundUpdateViewDistancePacket(
        clientbound_update_view_distance_packet::ClientboundUpdateViewDistancePacket,
    ),
    ClientboundCustomPayloadPacket(
        clientbound_custom_payload_packet::ClientboundCustomPayloadPacket,
    ),
    ClientboundChangeDifficultyPacket(
        clientbound_change_difficulty_packet::ClientboundChangeDifficultyPacket,
    ),
}

#[async_trait]
impl ProtocolPacket for GamePacket {
    fn id(&self) -> u32 {
        match self {
            GamePacket::ClientboundChangeDifficultyPacket(_packet) => 0x0e,
            GamePacket::ClientboundCustomPayloadPacket(_packet) => 0x18,
            GamePacket::ClientboundLoginPacket(_packet) => 0x26,
            GamePacket::ClientboundUpdateViewDistancePacket(_packet) => 0x4a,
        }
    }

    fn write(&self, buf: &mut Vec<u8>) -> Result<(), std::io::Error> {
        match self {
            GamePacket::ClientboundChangeDifficultyPacket(packet) => packet.write(buf),
            GamePacket::ClientboundCustomPayloadPacket(packet) => packet.write(buf),
            GamePacket::ClientboundLoginPacket(packet) => packet.write(buf),
            GamePacket::ClientboundUpdateViewDistancePacket(packet) => packet.write(buf),
        }
    }

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
                0x0e => clientbound_change_difficulty_packet::ClientboundChangeDifficultyPacket
                    ::read(buf)
                    .await?,
                0x18 => clientbound_custom_payload_packet::ClientboundCustomPayloadPacket::read(buf).await?,
                0x26 => clientbound_login_packet::ClientboundLoginPacket::read(buf).await?,
                0x4a => clientbound_update_view_distance_packet::ClientboundUpdateViewDistancePacket
                    ::read(buf)
                    .await?,
                // _ => return Err(format!("Unknown ServerToClient game packet id: {}", id)),
                _ => panic!("Unknown ServerToClient game packet id: {}", id),
            },
            PacketFlow::ClientToServer => match id {
                // 0x00 => serverbound_hello_packet::ServerboundHelloPacket::read(buf).await?,
                _ => return Err(format!("Unknown ClientToServer game packet id: {}", id)),
            },
        })
    }
}
