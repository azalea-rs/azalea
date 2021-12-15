///! Connect to Minecraft servers.
use minecraft_protocol::{
    connect::HandshakeConnection,
    packets::{
        handshake::client_intention_packet::ClientIntentionPacket,
        login::{serverbound_hello_packet::ServerboundHelloPacket, LoginPacket},
        ConnectionProtocol, PROTOCOL_VERSION,
    },
    resolver, ServerAddress,
};

pub async fn join_server(address: &ServerAddress) -> Result<(), String> {
    let username = "bot".to_string();

    let resolved_address = resolver::resolve_address(address).await?;

    let mut conn = HandshakeConnection::new(&resolved_address).await?;

    // handshake
    conn.write(
        ClientIntentionPacket {
            protocol_version: PROTOCOL_VERSION,
            hostname: address.host.clone(),
            port: address.port,
            intention: ConnectionProtocol::Login,
        }
        .get(),
    )
    .await;
    let mut conn = conn.login();

    // login start
    conn.write(ServerboundHelloPacket { username }.get()).await;

    // encryption request
    let packet = conn.read().await.unwrap();
    let _encryption_request_packet = match packet {
        LoginPacket::ClientboundHelloPacket(p) => p,
        _ => return Err(format!("Invalid packet type: {:?}", packet)),
    };

    // TODO: client auth

    // TODO: encryption response

    Ok(())
}
