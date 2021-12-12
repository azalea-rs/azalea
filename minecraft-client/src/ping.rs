///! Ping Minecraft servers.

use minecraft_protocol::{
    connection::Connection,
    packets::{
        handshake::client_intention_packet::ClientIntentionPacket,
        status::{
            clientbound_status_response_packet::ClientboundStatusResponsePacket,
            serverbound_status_request_packet::ServerboundStatusRequestPacket,
        },
        ConnectionProtocol, Packet, PacketTrait,
    },
    resolver, ServerAddress,
};

pub async fn ping_server(
    address: &ServerAddress,
) -> Result<ClientboundStatusResponsePacket, String> {
    let resolved_address = resolver::resolve_address(address).await?;

    let mut conn = Connection::new(&resolved_address).await?;

    // send the client intention packet and switch to the status state
    conn.send_packet(
        ClientIntentionPacket {
            protocol_version: 757,
            hostname: address.host.clone(),
            port: address.port,
            intention: ConnectionProtocol::Status,
        }
        .get(),
    )
    .await;
    conn.switch_state(ConnectionProtocol::Status);

    // send the empty status request packet
    conn.send_packet(ServerboundStatusRequestPacket {}.get())
        .await;

    let packet = conn.read_packet().await.unwrap();

    Ok(match packet {
        Packet::ClientboundStatusResponsePacket(p) => p,
        _ => Err("Invalid packet type".to_string())?,
    })
}
