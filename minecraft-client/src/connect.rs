///! Connect to Minecraft servers.
use minecraft_protocol::{
    connect::Connection,
    packets::{
        handshake::client_intention_packet::ClientIntentionPacket,
        login::serverbound_hello_packet::ServerboundHelloPacket,
        status::clientbound_status_response_packet::ClientboundStatusResponsePacket,
        ConnectionProtocol, Packet, PROTOCOL_VERSION,
    },
    resolver, ServerAddress,
};

pub async fn join_server(address: &ServerAddress) -> Result<(), String> {
    let username = "bot".to_string();

    let resolved_address = resolver::resolve_address(address).await?;

    let mut conn = Connection::new(&resolved_address).await?;

    // handshake
    conn.send_packet(
        ClientIntentionPacket {
            protocol_version: PROTOCOL_VERSION,
            hostname: address.host.clone(),
            port: address.port,
            intention: ConnectionProtocol::Login,
        }
        .get(),
    )
    .await;
    conn.switch_state(ConnectionProtocol::Login);

    // login start
    conn.send_packet(ServerboundHelloPacket { username }.get())
        .await;

    // encryption request
    let packet = conn.read_packet().await.unwrap();
    let encryption_request_packet = match packet {
        Packet::ClientboundHelloPacket(p) => p,
        _ => Err(format!("Invalid packet type: {:?}", packet))?,
    };

    // TODO: client auth

    // TODO: encryption response

    Ok(())
}
