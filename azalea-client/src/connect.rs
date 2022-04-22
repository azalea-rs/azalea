///! Connect to Minecraft servers.
use azalea_protocol::{
    connect::HandshakeConnection,
    packets::{
        game::GamePacket,
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

    // login
    conn.write(ServerboundHelloPacket { username }.get()).await;

    let mut conn = loop {
        let packet_result = conn.read().await;
        match packet_result {
            Ok(packet) => match packet {
                LoginPacket::ClientboundHelloPacket(p) => {
                    println!("Got encryption request {:?} {:?}", p.nonce, p.public_key);
                }
                LoginPacket::ClientboundLoginCompressionPacket(p) => {
                    println!("Got compression request {:?}", p.compression_threshold);
                    conn.set_compression_threshold(p.compression_threshold);
                }
                LoginPacket::ClientboundGameProfilePacket(p) => {
                    println!("Got profile {:?}", p.game_profile);
                    break conn.game();
                }
                _ => panic!("unhandled packet"),
            },
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    };

    // game
    loop {
        let packet_result = conn.read().await;
        match packet_result {
            Ok(packet) => match packet {
                GamePacket::ClientboundLoginPacket(p) => {
                    println!("Got login packet {:?}", p);
                }
                GamePacket::ClientboundUpdateViewDistancePacket(p) => {
                    println!("Got view distance packet {:?}", p);
                }
                GamePacket::ClientboundCustomPayloadPacket(p) => {
                    println!("Got custom payload packet {:?}", p);
                }
                GamePacket::ClientboundChangeDifficultyPacket(p) => {
                    println!("Got difficulty packet {:?}", p);
                }
                GamePacket::ClientboundDeclareCommandsPacket(p) => {
                    println!("Got declare commands packet {:?}", p);
                }
                GamePacket::ClientboundPlayerAbilitiesPacket(p) => {
                    println!("Got player abilities packet {:?}", p);
                }
                GamePacket::ClientboundSetCarriedItemPacket(p) => {
                    println!("Got set carried item packet {:?}", p);
                }
                GamePacket::ClientboundUpdateTagsPacket(p) => {
                    println!("Got update tags packet {:?}", p);
                }
            },
            Err(e) => {
                panic!("Error: {:?}", e);
            }
        }
    }

    Ok(())
}
