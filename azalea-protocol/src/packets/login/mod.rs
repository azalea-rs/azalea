pub mod clientbound_custom_query_packet;
pub mod clientbound_game_profile_packet;
pub mod clientbound_hello_packet;
pub mod clientbound_login_compression_packet;
pub mod serverbound_hello_packet;

use super::ProtocolPacket;
use crate::connect::PacketFlow;
use async_trait::async_trait;

#[derive(Clone, Debug)]
pub enum LoginPacket
where
    Self: Sized,
{
    ClientboundCustomQueryPacket(clientbound_custom_query_packet::ClientboundCustomQueryPacket),
    ClientboundGameProfilePacket(clientbound_game_profile_packet::ClientboundGameProfilePacket),
    ClientboundHelloPacket(clientbound_hello_packet::ClientboundHelloPacket),
    ClientboundLoginCompressionPacket(
        clientbound_login_compression_packet::ClientboundLoginCompressionPacket,
    ),
    ServerboundHelloPacket(serverbound_hello_packet::ServerboundHelloPacket),
}

#[async_trait]
impl ProtocolPacket for LoginPacket {
    fn id(&self) -> u32 {
        match self {
            LoginPacket::ClientboundCustomQueryPacket(_packet) => 0x04,
            LoginPacket::ClientboundGameProfilePacket(_packet) => 0x02,
            LoginPacket::ClientboundHelloPacket(_packet) => 0x01,
            LoginPacket::ClientboundLoginCompressionPacket(_packet) => 0x03,
            LoginPacket::ServerboundHelloPacket(_packet) => 0x00,
        }
    }

    fn write(&self, buf: &mut Vec<u8>) {
        match self {
            LoginPacket::ClientboundCustomQueryPacket(packet) => packet.write(buf),
            LoginPacket::ClientboundGameProfilePacket(packet) => packet.write(buf),
            LoginPacket::ClientboundHelloPacket(packet) => packet.write(buf),
            LoginPacket::ClientboundLoginCompressionPacket(packet) => packet.write(buf),
            LoginPacket::ServerboundHelloPacket(packet) => packet.write(buf),
        }
    }

    /// Read a packet by its id, ConnectionProtocol, and flow
    async fn read<T: tokio::io::AsyncRead + std::marker::Unpin + std::marker::Send>(
        id: u32,
        flow: &PacketFlow,
        buf: &mut T,
    ) -> Result<LoginPacket, String>
    where
        Self: Sized,
    {
        Ok(match flow {
            PacketFlow::ServerToClient => match id {
                0x01 => clientbound_hello_packet::ClientboundHelloPacket::read(buf).await?,
                0x02 => {
                    clientbound_game_profile_packet::ClientboundGameProfilePacket::read(buf).await?
                }
                0x04 => {
                    clientbound_custom_query_packet::ClientboundCustomQueryPacket::read(buf).await?
                }
                0x03 => {
                    clientbound_login_compression_packet::ClientboundLoginCompressionPacket::read(
                        buf,
                    )
                    .await?
                }
                _ => return Err(format!("Unknown ServerToClient status packet id: {}", id)),
            },
            PacketFlow::ClientToServer => match id {
                0x00 => serverbound_hello_packet::ServerboundHelloPacket::read(buf).await?,
                _ => return Err(format!("Unknown ClientToServer status packet id: {}", id)),
            },
        })
    }
}
