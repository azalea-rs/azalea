///! Ping Minecraft servers.
use minecraft_protocol::{
    connect::HandshakeConnection,
    packets::{
        handshake::client_intention_packet::ClientIntentionPacket,
        status::{
            clientbound_status_response_packet::ClientboundStatusResponsePacket,
            serverbound_status_request_packet::ServerboundStatusRequestPacket, StatusPacket,
        },
        ConnectionProtocol, PROTOCOL_VERSION,
    },
    resolver, ServerAddress,
};

pub async fn ping_server(
    address: &ServerAddress,
) -> Result<ClientboundStatusResponsePacket, String> {
    let resolved_address = resolver::resolve_address(address).await?;

    let mut conn = HandshakeConnection::new(&resolved_address).await?;

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
    .await;
    let mut conn = conn.status();

    // send the empty status request packet
    conn.write(ServerboundStatusRequestPacket {}.get()).await;

    let packet = conn.read().await.unwrap();

    match packet {
        StatusPacket::ClientboundStatusResponsePacket(p) => Ok(*p),
        _ => Err("Invalid packet type".to_string()),
    }
}
