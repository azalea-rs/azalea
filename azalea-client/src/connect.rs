use crate::Player;
use azalea_core::resource_location::ResourceLocation;
use azalea_protocol::{
    connect::{GameConnection, HandshakeConnection},
    packets::{
        game::{serverbound_custom_payload_packet::ServerboundCustomPayloadPacket, GamePacket},
        handshake::client_intention_packet::ClientIntentionPacket,
        login::{
            serverbound_hello_packet::ServerboundHelloPacket,
            serverbound_key_packet::ServerboundKeyPacket, LoginPacket,
        },
        ConnectionProtocol, PROTOCOL_VERSION,
    },
    resolver, ServerAddress,
};
use std::sync::Arc;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tokio::sync::Mutex;

///! Connect to Minecraft servers.

/// Something that can join Minecraft servers.
pub struct Account {
    username: String,
}

#[derive(Default)]
pub struct ClientState {
    // placeholder
    pub player: Player,
}

/// A player that you can control that is currently in a Minecraft server.
pub struct Client {
    event_receiver: UnboundedReceiver<Event>,
    conn: Arc<Mutex<GameConnection>>,
    state: Arc<Mutex<ClientState>>,
    // game_loop
}

#[derive(Debug, Clone)]
pub enum Event {
    Login,
}

/// Whether we should ignore errors when decoding packets.
const IGNORE_ERRORS: bool = false;

impl Client {
    async fn join(account: &Account, address: &ServerAddress) -> Result<Self, String> {
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
        conn.write(
            ServerboundHelloPacket {
                username: account.username.clone(),
            }
            .get(),
        )
        .await;

        let conn = loop {
            let packet_result = conn.read().await;
            match packet_result {
                Ok(packet) => match packet {
                    LoginPacket::ClientboundHelloPacket(p) => {
                        println!("Got encryption request");
                        let e = azalea_crypto::encrypt(&p.public_key, &p.nonce).unwrap();

                        // TODO: authenticate with the server here (authenticateServer)

                        conn.write(
                            ServerboundKeyPacket {
                                nonce: e.encrypted_nonce,
                                shared_secret: e.encrypted_public_key,
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
                    LoginPacket::ClientboundLoginDisconnectPacket(p) => {
                        println!("Got disconnect {:?}", p);
                    }
                    LoginPacket::ClientboundCustomQueryPacket(p) => {
                        println!("Got custom query {:?}", p);
                    }
                    _ => panic!("Unexpected packet {:?}", packet),
                },
                Err(e) => {
                    panic!("Error: {:?}", e);
                }
            }
        };

        let conn = Arc::new(Mutex::new(conn));

        let (tx, rx) = mpsc::unbounded_channel();

        // we got the GameConnection, so the server is now connected :)
        let client = Client {
            event_receiver: rx,
            conn: conn.clone(),
            state: Arc::new(Mutex::new(ClientState::default())),
        };
        // let client = Arc::new(Mutex::new(client));
        // let weak_client = Arc::<_>::downgrade(&client);

        // just start up the game loop and we're ready!
        // tokio::spawn(Self::game_loop(conn, tx, handler, state))

        let game_loop_conn = conn.clone();
        let game_loop_state = client.state.clone();

        tokio::spawn(async move { Self::game_loop(game_loop_conn, tx, game_loop_state).await });

        Ok(client)
    }

    async fn game_loop(
        conn: Arc<Mutex<GameConnection>>,
        tx: UnboundedSender<Event>,
        state: Arc<Mutex<ClientState>>,
    ) {
        loop {
            let r = conn.lock().await.read().await;
            match r {
                Ok(packet) => Self::handle(&packet, &tx, &state, &conn).await,
                Err(e) => {
                    if IGNORE_ERRORS {
                        println!("Error: {:?}", e);
                        if e == "length wider than 21-bit" {
                            panic!();
                        }
                    } else {
                        panic!("Error: {:?}", e);
                    }
                }
            };
        }
    }

    async fn handle(
        packet: &GamePacket,
        tx: &UnboundedSender<Event>,
        state: &Arc<Mutex<ClientState>>,
        conn: &Arc<Mutex<GameConnection>>,
    ) {
        match packet {
            GamePacket::ClientboundLoginPacket(p) => {
                println!("Got login packet {:?}", p);

                state.lock().await.player.entity.id = p.player_id;
                conn.lock()
                    .await
                    .write(
                        ServerboundCustomPayloadPacket {
                            identifier: ResourceLocation::new("brand").unwrap(),
                            // they don't have to know :)
                            data: "vanilla".into(),
                        }
                        .get(),
                    )
                    .await;

                tx.send(Event::Login).unwrap();
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
                println!("Got disconnect packet {:?}", p);
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
                println!("Got chunk with light packet {} {}", p.x, p.z);
                // p.chunk_data
            }
            GamePacket::ClientboundLightUpdatePacket(p) => {
                println!("Got light update packet {:?}", p);
            }
            GamePacket::ClientboundAddMobPacket(p) => {
                println!("Got add mob packet {:?}", p);
            }
            GamePacket::ClientboundAddEntityPacket(p) => {
                println!("Got add entity packet {:?}", p);
            }
            GamePacket::ClientboundSetEntityDataPacket(p) => {
                println!("Got set entity data packet {:?}", p);
            }
            GamePacket::ClientboundUpdateAttributesPacket(p) => {
                println!("Got update attributes packet {:?}", p);
            }
            GamePacket::ClientboundEntityVelocityPacket(p) => {
                println!("Got entity velocity packet {:?}", p);
            }
            GamePacket::ClientboundSetEntityLinkPacket(p) => {
                println!("Got set entity link packet {:?}", p);
            }
            _ => panic!("Unexpected packet {:?}", packet),
        }
        println!();
    }

    pub async fn next(&mut self) -> Option<Event> {
        self.event_receiver.recv().await
    }
}

impl Account {
    pub fn offline(username: &str) -> Self {
        Self {
            username: username.to_string(),
        }
    }

    pub async fn join(&self, address: &ServerAddress) -> Result<Client, String> {
        Client::join(self, address).await
    }
}
