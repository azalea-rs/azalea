///! Ping Minecraft servers.
use azalea_protocol::{
    connect::{Connection, ConnectionError},
    packets::{
        handshake::client_intention_packet::ClientIntentionPacket,
        status::{
            clientbound_status_response_packet::ClientboundStatusResponsePacket,
            serverbound_status_request_packet::ServerboundStatusRequestPacket,
            ClientboundStatusPacket,
        },
        ConnectionProtocol, PROTOCOL_VERSION,
    },
    resolver, ServerAddress,
};
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PingError {
    #[error("{0}")]
    Resolver(#[from] resolver::ResolverError),
    #[error("{0}")]
    Connection(#[from] ConnectionError),
    #[error("{0}")]
    ReadPacket(#[from] azalea_protocol::read::ReadPacketError),
    #[error("{0}")]
    WritePacket(#[from] io::Error),
}

pub async fn ping_server(
    address: &ServerAddress,
) -> Result<ClientboundStatusResponsePacket, PingError> {
    let resolved_address = resolver::resolve_address(address).await?;

    let mut conn = Connection::new(&resolved_address).await?;

    // send the client intention packet and switch to the status state
    conn.write(
        ClientIntentionPacket {
            protocol_version: PROTOCOL_VERSION,
            hostname: address.host.clone(),
            port: address.port,
            intention: ConnectionProtocol::Status,
        }
        .get(),
    )
    .await?;
    let mut conn = conn.status();

    // send the empty status request packet
    conn.write(ServerboundStatusRequestPacket {}.get()).await?;

    let packet = conn.read().await?;

    match packet {
        ClientboundStatusPacket::ClientboundStatusResponsePacket(p) => Ok(p),
    }
}
