pub mod clientbound_status_response_packet;
pub mod serverbound_status_request_packet;

use async_trait::async_trait;
use tokio::io::BufReader;

use crate::connect::PacketFlow;

use super::{ConnectionProtocol, PacketTrait, ProtocolPacket};

#[derive(Clone, Debug)]
pub enum StatusPacket {
    ServerboundStatusRequestPacket(
        serverbound_status_request_packet::ServerboundStatusRequestPacket,
    ),
    ClientboundStatusResponsePacket(
        clientbound_status_response_packet::ClientboundStatusResponsePacket,
    ),
}

// #[async_trait]
// impl ProtocolPacket for StatusPacket {
impl StatusPacket {
    fn get_inner(self) -> impl PacketTrait {
        match self {
            StatusPacket::ServerboundStatusRequestPacket(packet) => packet,
            StatusPacket::ClientboundStatusResponsePacket(packet) => packet,
        }
    }
    // fn get_inner(&self) -> StatusPacket {
    //     match self {
    //         StatusPacket::ServerboundStatusRequestPacket(packet) => packet,
    //         StatusPacket::ClientboundStatusResponsePacket(packet) => packet,
    //     }
    // }

    fn id(&self) -> u32 {
        match self {
            StatusPacket::ServerboundStatusRequestPacket(_packet) => 0x00,
            StatusPacket::ClientboundStatusResponsePacket(_packet) => 0x00,
        }
    }

    fn write(&self, buf: &mut Vec<u8>) {
        match self {
            StatusPacket::ServerboundStatusRequestPacket(packet) => packet.write(buf),
            StatusPacket::ClientboundStatusResponsePacket(packet) => packet.write(buf),
        }
    }

    /// Read a packet by its id, ConnectionProtocol, and flow
    async fn read<
        T: tokio::io::AsyncRead + std::marker::Unpin + std::marker::Send,
        P: ProtocolPacket,
    >(
        id: u32,
        flow: &PacketFlow,
        buf: &mut BufReader<T>,
    ) -> Result<P, String>
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
