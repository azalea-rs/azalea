pub mod game;
pub mod handshake;
pub mod login;
pub mod status;

use async_trait::async_trait;
use tokio::io::{AsyncRead, BufReader};

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
    Status(status::StatusPacket),
}

#[async_trait]
pub trait ProtocolPacket {
    fn get_inner<P: PacketTrait>(&self) -> &P;

    fn id(&self) -> u32;

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
        Self: Sized;

    fn write(&self, buf: &mut Vec<u8>);
}

// impl Packet {
//     fn get_inner_packet(&self) -> &dyn PacketTrait {
//         match self {
//             Packet::ClientIntentionPacket(packet) => packet,
//             Packet::ServerboundStatusRequestPacket(packet) => packet,
//             Packet::ClientboundStatusResponsePacket(packet) => packet,
//             Packet::ServerboundHelloPacket(packet) => packet,
//             Packet::ClientboundHelloPacket(packet) => packet,
//         }
//     }

//     pub fn id(&self) -> u32 {
//         match self {
//             Packet::ClientIntentionPacket(_packet) => 0x00,
//             Packet::ServerboundStatusRequestPacket(_packet) => 0x00,
//             Packet::ClientboundStatusResponsePacket(_packet) => 0x00,
//             Packet::ServerboundHelloPacket(_packet) => 0x00,
//             Packet::ClientboundHelloPacket(_packet) => 0x01,
//         }
//     }

//     /// Read a packet by its id, ConnectionProtocol, and flow
//     pub async fn read<T: tokio::io::AsyncRead + std::marker::Unpin + std::marker::Send>(
//         id: u32,
//         protocol: &ConnectionProtocol,
//         flow: &PacketFlow,
//         buf: &mut BufReader<T>,
//     ) -> Result<Packet, String> {
//         match protocol {
//             ConnectionProtocol::Handshake => match flow {
//                 PacketFlow::ClientToServer => match id {
//                     0x00 => Ok(
//                         handshake::client_intention_packet::ClientIntentionPacket::read(buf).await?,
//                     ),
//                     _ => Err(format!("Unknown ClientToServer handshake packet id: {}", id)),
//                 }
//                 PacketFlow::ServerToClient => Err("ServerToClient handshake packets not implemented".to_string()),
//             },

//             ConnectionProtocol::Game => Err("Game protocol not implemented yet".to_string()),

//             ConnectionProtocol::Status => match flow {
//                 PacketFlow::ServerToClient => match id {
//                     0x00 => Ok(
//                         status::clientbound_status_response_packet::ClientboundStatusResponsePacket
//                             ::read(buf)
//                             .await?,
//                     ),
//                     _ => Err(format!("Unknown ServerToClient status packet id: {}", id)),
//                 },
//                 PacketFlow::ClientToServer => match id {
//                     0x00 => Ok(
//                         status::serverbound_status_request_packet::ServerboundStatusRequestPacket
//                             ::read(buf)
//                             .await?,
//                     ),
//                     _ => Err(format!("Unknown ClientToServer status packet id: {}", id)),
//                 },
//             },

//             ConnectionProtocol::Login => match flow {
//                 PacketFlow::ServerToClient => match id {
//                     0x01 => Ok(
//                         login::clientbound_hello_packet::ClientboundHelloPacket::read(buf).await?,
//                     ),
//                     _ => Err(format!("Unknown ServerToClient login packet id: {}", id)),
//                 },
//                 PacketFlow::ClientToServer => match id {
//                     0x00 => Ok(
//                         login::serverbound_hello_packet::ServerboundHelloPacket::read(buf).await?,
//                     ),
//                     _ => Err(format!("Unknown ClientToServer login packet id: {}", id)),
//                 },
//             },
//         }
//     }

//     pub fn write(&self, buf: &mut Vec<u8>) {
//         self.get_inner_packet().write(buf);
//     }
// }

#[async_trait]
pub trait PacketTrait {
    /// Return a version of the packet that you can actually use for stuff
    fn get<P: ProtocolPacket>(self) -> P;
    fn write(&self, buf: &mut Vec<u8>);
    async fn read<T: AsyncRead + std::marker::Unpin + std::marker::Send, P: ProtocolPacket>(
        buf: &mut BufReader<T>,
    ) -> Result<P, String>
    where
        Self: Sized;
}
