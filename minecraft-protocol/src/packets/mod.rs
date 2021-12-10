pub mod game;
pub mod handshake;
pub mod login;
pub mod status;

use async_trait::async_trait;
use tokio::io::{AsyncRead, BufReader};

use crate::connection::PacketFlow;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConnectionProtocol {
    Handshake = -1,
    Game = 0,
    Status = 1,
    Login = 2,
}

#[derive(Clone, Debug)]
pub enum Packet {
    // game

    // handshake
    ClientIntentionPacket(handshake::client_intention_packet::ClientIntentionPacket),

    // login

    // status
    ServerboundStatusRequestPacket(
        status::serverbound_status_request_packet::ServerboundStatusRequestPacket,
    ),
    ClientboundStatusResponsePacket(
        status::clientbound_status_response_packet::ClientboundStatusResponsePacket,
    ),
}

// TODO: do all this with macros so it's less repetitive
impl Packet {
    fn get_inner_packet(&self) -> &dyn PacketTrait {
        match self {
            Packet::ClientIntentionPacket(packet) => packet,
            Packet::ServerboundStatusRequestPacket(packet) => packet,
            Packet::ClientboundStatusResponsePacket(packet) => packet,
        }
    }

    pub fn id(&self) -> u32 {
        match self {
            Packet::ClientIntentionPacket(_packet) => 0x00,
            Packet::ServerboundStatusRequestPacket(_packet) => 0x00,
            Packet::ClientboundStatusResponsePacket(_packet) => 0x00,
        }
    }

    /// Read a packet by its id, ConnectionProtocol, and flow
    pub async fn read<T: tokio::io::AsyncRead + std::marker::Unpin + std::marker::Send>(
        id: u32,
        protocol: &ConnectionProtocol,
        flow: &PacketFlow,
        buf: &mut BufReader<T>,
    ) -> Result<Packet, String> {
        match protocol {
            ConnectionProtocol::Handshake => match id {
                0x00 => Ok(
                    handshake::client_intention_packet::ClientIntentionPacket::read(buf).await?,
                ),
                _ => Err(format!("Unknown packet id: {}", id)),
            },
            ConnectionProtocol::Game => Err("Game protocol not implemented yet".to_string()),
            ConnectionProtocol::Status => match flow {
                PacketFlow::ServerToClient => match id {
                    0x00 => Ok(
                        status::clientbound_status_response_packet::ClientboundStatusResponsePacket
                            ::read(buf)
                            .await?,
                    ),
                    _ => Err(format!("Unknown packet id: {}", id)),
                },
                PacketFlow::ClientToServer => match id {
                    0x00 => Ok(
                        status::serverbound_status_request_packet::ServerboundStatusRequestPacket
                            ::read(buf)
                            .await?,
                    ),
                    _ => Err(format!("Unknown packet id: {}", id)),
                },
            },
            ConnectionProtocol::Login => Err("Login protocol not implemented yet".to_string()),
        }
    }

    pub fn write(&self, buf: &mut Vec<u8>) {
        self.get_inner_packet().write(buf);
    }
}

#[async_trait]
pub trait PacketTrait {
    /// Return a version of the packet that you can actually use for stuff
    fn get(self) -> Packet;
    fn write(&self, buf: &mut Vec<u8>);
    async fn read<T: AsyncRead + std::marker::Unpin + std::marker::Send>(
        buf: &mut BufReader<T>,
    ) -> Result<Packet, String>
    where
        Self: Sized;
}
