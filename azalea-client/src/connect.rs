///! Connect to Minecraft servers.
use azalea_protocol::{
    connect::HandshakeConnection,
    packets::{
        game::GamePacket,
        handshake::client_intention_packet::ClientIntentionPacket,
        login::{
            serverbound_hello_packet::ServerboundHelloPacket,
            serverbound_key_packet::ServerboundKeyPacket, LoginPacket,
        },
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
                    let e = azalea_auth::encryption::encrypt(&p.public_key, &p.nonce).unwrap();

                    // TODO: authenticate with the server here (authenticateServer)
                    println!("Sending encryption response {:?}", e);

                    conn.write(
                        ServerboundKeyPacket {
                            nonce: e.encrypted_nonce.into(),
                            shared_secret: e.encrypted_public_key.into(),
                        }
                        .get(),
                    )
                    .await;
                    conn.set_encryption_key(e.secret_key);
                }
                LoginPacket::ClientboundLoginCompressionPacket(p) => {
                    println!("Got compression request {:?}", p.compression_threshold);
                    conn.set_compression_threshold(p.compression_threshold);
                }
                LoginPacket::ClientboundGameProfilePacket(p) => {
                    println!("Got profile {:?}", p.game_profile);
                    break conn.game();
                }
                LoginPacket::ServerboundHelloPacket(p) => {
                    println!("Got hello {:?}", p);
                }
                LoginPacket::ClientboundLoginDisconnectPacket(p) => {
                    println!("Got disconnect {:?}", p);
                }
                LoginPacket::ClientboundCustomQueryPacket(p) => {
                    println!("Got custom query {:?}", p);
                }
                LoginPacket::ServerboundKeyPacket(_) => todo!(),
            },
            Err(e) => {
                panic!("Error: {:?}", e);
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
                    println!("Got declare commands packet");
                }
                GamePacket::ClientboundPlayerAbilitiesPacket(p) => {
                    println!("Got player abilities packet {:?}", p);
                }
                GamePacket::ClientboundSetCarriedItemPacket(p) => {
                    println!("Got set carried item packet {:?}", p);
                }
                GamePacket::ClientboundUpdateTagsPacket(p) => {
                    println!("Got update tags packet");
                }
                GamePacket::ClientboundDisconnectPacket(p) => {
                    println!("Got login disconnect packet {:?}", p);
                }
                GamePacket::ClientboundUpdateRecipesPacket(p) => {
                    println!("Got update recipes packet");
                }
                GamePacket::ClientboundEntityEventPacket(p) => {
                    println!("Got entity event packet {:?}", p);
                }
                GamePacket::ClientboundRecipePacket(p) => {
                    println!("Got recipe packet");
                }
                GamePacket::ClientboundPlayerPositionPacket(p) => {
                    // TODO: reply with teleport confirm
                    println!("Got player position packet {:?}", p);
                }
                GamePacket::ClientboundPlayerInfoPacket(p) => {
                    println!("Got player info packet {:?}", p);
                }
                GamePacket::ClientboundSetChunkCacheCenterPacket(p) => {
                    println!("Got chunk cache center packet {:?}", p);
                }
                GamePacket::ClientboundLevelChunkWithLightPacket(p) => {
                    println!("Got chunk with light packet");
                }
                GamePacket::ClientboundLightUpdatePacket(p) => {
                    println!("Got light update packet {:?}", p);
                }
            },
            Err(e) => {
                panic!("Error: {:?}", e);
            }
        }
        println!();
    }

    Ok(())
}
