pub mod clientbound_hello_packet;
pub mod serverbound_hello_packet;

use async_trait::async_trait;
use tokio::io::BufReader;

use crate::connect::PacketFlow;

use super::ProtocolPacket;

#[derive(Clone, Debug)]
pub enum LoginPacket
where
    Self: Sized,
{
    ServerboundHelloPacket(serverbound_hello_packet::ServerboundHelloPacket),
    ClientboundHelloPacket(clientbound_hello_packet::ClientboundHelloPacket),
}

#[async_trait]
impl ProtocolPacket for LoginPacket {
    fn id(&self) -> u32 {
        match self {
            LoginPacket::ServerboundStatusRequestPacket(_packet) => 0x00,
            LoginPacket::ClientboundStatusResponsePacket(_packet) => 0x00,
        }
    }

    fn write(&self, buf: &mut Vec<u8>) {
        match self {
            LoginPacket::ServerboundStatusRequestPacket(packet) => packet.write(buf),
            LoginPacket::ClientboundStatusResponsePacket(packet) => packet.write(buf),
        }
    }

    /// Read a packet by its id, ConnectionProtocol, and flow
    async fn read<T: tokio::io::AsyncRead + std::marker::Unpin + std::marker::Send>(
        id: u32,
        flow: &PacketFlow,
        buf: &mut BufReader<T>,
    ) -> Result<LoginPacket, String>
    where
        Self: Sized,
    {
        match flow {
            PacketFlow::ServerToClient => match id {
                0x00 => Ok(
                    clientbound_status_response_packet::ClientboundStatusResponsePacket::read(buf)
                        .await?,
                ),
                _ => Err(format!("Unknown ServerToClient status packet id: {}", id)),
            },
            PacketFlow::ClientToServer => match id {
                0x00 => Ok(
                    serverbound_status_request_packet::ServerboundStatusRequestPacket::read(buf)
                        .await?,
                ),
                _ => Err(format!("Unknown ClientToServer status packet id: {}", id)),
            },
        }
    }
}
