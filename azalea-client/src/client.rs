use crate::{Account, Player};
use azalea_core::{resource_location::ResourceLocation, ChunkPos};
use azalea_entity::Entity;
use azalea_protocol::{
    connect::{GameConnection, HandshakeConnection},
    packets::{
        game::{
            clientbound_player_chat_packet::ClientboundPlayerChatPacket,
            clientbound_system_chat_packet::ClientboundSystemChatPacket,
            serverbound_custom_payload_packet::ServerboundCustomPayloadPacket,
            serverbound_keep_alive_packet::ServerboundKeepAlivePacket, GamePacket,
        },
        handshake::client_intention_packet::ClientIntentionPacket,
        login::{
            serverbound_hello_packet::ServerboundHelloPacket,
            serverbound_key_packet::{NonceOrSaltSignature, ServerboundKeyPacket},
            LoginPacket,
        },
        ConnectionProtocol, PROTOCOL_VERSION,
    },
    resolver, ServerAddress,
};
use azalea_world::World;
use owning_ref::OwningRef;
use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

#[derive(Default)]
pub struct ClientState {
    pub player: Player,
    pub world: Option<World>,
}

#[derive(Debug, Clone)]
pub enum Event {
    Login,
    Chat(ChatPacket),
}

#[derive(Debug, Clone)]
pub enum ChatPacket {
    System(ClientboundSystemChatPacket),
    Player(Box<ClientboundPlayerChatPacket>),
}

// impl ChatPacket {
//     pub fn message(&self) -> &str {
//         match self {
//             ChatPacket::System(p) => &p.content,
//             ChatPacket::Player(p) => &p.message,
//         }
//     }
// }

/// A player that you can control that is currently in a Minecraft server.
pub struct Client {
    event_receiver: UnboundedReceiver<Event>,
    pub conn: Arc<tokio::sync::Mutex<GameConnection>>,
    pub state: Arc<Mutex<ClientState>>,
    // game_loop
}

/// Whether we should ignore errors when decoding packets.
const IGNORE_ERRORS: bool = !cfg!(debug_assertions);

#[derive(Debug)]
struct HandleError(String);

impl Client {
    /// Connect to a Minecraft server with an account.
    pub async fn join(account: &Account, address: &ServerAddress) -> Result<Self, String> {
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
                public_key: None,
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
                                nonce_or_salt_signature: NonceOrSaltSignature::Nonce(
                                    e.encrypted_nonce,
                                ),
                                key_bytes: e.encrypted_public_key,
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

        let conn = Arc::new(tokio::sync::Mutex::new(conn));

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

        let game_loop_state = client.state.clone();

        tokio::spawn(Self::game_loop(conn, tx, game_loop_state));

        Ok(client)
    }

    async fn game_loop(
        conn: Arc<tokio::sync::Mutex<GameConnection>>,
        tx: UnboundedSender<Event>,
        state: Arc<Mutex<ClientState>>,
    ) {
        loop {
            let r = conn.lock().await.read().await;
            match r {
                Ok(packet) => match Self::handle(&packet, &tx, &state, &conn).await {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Error handling packet: {:?}", e);
                        if IGNORE_ERRORS {
                            continue;
                        } else {
                            panic!("Error handling packet: {:?}", e);
                        }
                    }
                },
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
        conn: &Arc<tokio::sync::Mutex<GameConnection>>,
    ) -> Result<(), HandleError> {
        match packet {
            GamePacket::ClientboundLoginPacket(p) => {
                println!("Got login packet {:?}", p);

                {
                    let mut state = state.lock()?;

                    // // write p into login.txt
                    // std::io::Write::write_all(
                    //     &mut std::fs::File::create("login.txt").unwrap(),
                    //     format!("{:#?}", p).as_bytes(),
                    // )
                    // .unwrap();

                    state.player.entity.id = p.player_id;

                    // TODO: have registry_holder be a struct because this sucks rn
                    // best way would be to add serde support to azalea-nbt

                    let registry_holder = p
                        .registry_holder
                        .as_compound()
                        .expect("Registry holder is not a compound")
                        .get("")
                        .expect("No \"\" tag")
                        .as_compound()
                        .expect("\"\" tag is not a compound");
                    let dimension_types = registry_holder
                        .get("minecraft:dimension_type")
                        .expect("No dimension_type tag")
                        .as_compound()
                        .expect("dimension_type is not a compound")
                        .get("value")
                        .expect("No dimension_type value")
                        .as_list()
                        .expect("dimension_type value is not a list");
                    let dimension_type = dimension_types
                        .iter()
                        .find(|t| {
                            t.as_compound()
                                .expect("dimension_type value is not a compound")
                                .get("name")
                                .expect("No name tag")
                                .as_string()
                                .expect("name is not a string")
                                == p.dimension_type.to_string()
                        })
                        .unwrap_or_else(|| {
                            panic!("No dimension_type with name {}", p.dimension_type)
                        })
                        .as_compound()
                        .unwrap()
                        .get("element")
                        .expect("No element tag")
                        .as_compound()
                        .expect("element is not a compound");
                    let height = (*dimension_type
                        .get("height")
                        .expect("No height tag")
                        .as_int()
                        .expect("height tag is not an int"))
                    .try_into()
                    .expect("height is not a u32");
                    let min_y = *dimension_type
                        .get("min_y")
                        .expect("No min_y tag")
                        .as_int()
                        .expect("min_y tag is not an int");

                    state.world = Some(World::new(16, height, min_y));
                }

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
            GamePacket::ClientboundDeclareCommandsPacket(_p) => {
                println!("Got declare commands packet");
            }
            GamePacket::ClientboundPlayerAbilitiesPacket(p) => {
                println!("Got player abilities packet {:?}", p);
            }
            GamePacket::ClientboundSetCarriedItemPacket(p) => {
                println!("Got set carried item packet {:?}", p);
            }
            GamePacket::ClientboundUpdateTagsPacket(_p) => {
                println!("Got update tags packet");
            }
            GamePacket::ClientboundDisconnectPacket(p) => {
                println!("Got disconnect packet {:?}", p);
            }
            GamePacket::ClientboundUpdateRecipesPacket(_p) => {
                println!("Got update recipes packet");
            }
            GamePacket::ClientboundEntityEventPacket(_p) => {
                // println!("Got entity event packet {:?}", p);
            }
            GamePacket::ClientboundRecipePacket(_p) => {
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
                state
                    .lock()?
                    .world
                    .as_mut()
                    .unwrap()
                    .update_view_center(&ChunkPos::new(p.x, p.z));
            }
            GamePacket::ClientboundLevelChunkWithLightPacket(p) => {
                println!("Got chunk with light packet {} {}", p.x, p.z);
                let pos = ChunkPos::new(p.x, p.z);
                // let chunk = Chunk::read_with_world_height(&mut p.chunk_data);
                // println("chunk {:?}")
                state
                    .lock()?
                    .world
                    .as_mut()
                    .expect("World doesn't exist! We should've gotten a login packet by now.")
                    .replace_with_packet_data(&pos, &mut p.chunk_data.data.as_slice())
                    .unwrap();
            }
            GamePacket::ClientboundLightUpdatePacket(p) => {
                println!("Got light update packet {:?}", p);
            }
            GamePacket::ClientboundAddEntityPacket(p) => {
                println!("Got add entity packet {:?}", p);
                let entity = Entity::from(p);
                state
                    .lock()?
                    .world
                    .as_mut()
                    .expect("World doesn't exist! We should've gotten a login packet by now.")
                    .add_entity(entity);
            }
            GamePacket::ClientboundSetEntityDataPacket(_p) => {
                // println!("Got set entity data packet {:?}", p);
            }
            GamePacket::ClientboundUpdateAttributesPacket(_p) => {
                // println!("Got update attributes packet {:?}", p);
            }
            GamePacket::ClientboundEntityVelocityPacket(_p) => {
                // println!("Got entity velocity packet {:?}", p);
            }
            GamePacket::ClientboundSetEntityLinkPacket(p) => {
                println!("Got set entity link packet {:?}", p);
            }
            GamePacket::ClientboundAddPlayerPacket(p) => {
                println!("Got add player packet {:?}", p);
            }
            GamePacket::ClientboundInitializeBorderPacket(p) => {
                println!("Got initialize border packet {:?}", p);
            }
            GamePacket::ClientboundSetTimePacket(p) => {
                println!("Got set time packet {:?}", p);
            }
            GamePacket::ClientboundSetDefaultSpawnPositionPacket(p) => {
                println!("Got set default spawn position packet {:?}", p);
            }
            GamePacket::ClientboundContainerSetContentPacket(p) => {
                println!("Got container set content packet {:?}", p);
            }
            GamePacket::ClientboundSetHealthPacket(p) => {
                println!("Got set health packet {:?}", p);
            }
            GamePacket::ClientboundSetExperiencePacket(p) => {
                println!("Got set experience packet {:?}", p);
            }
            GamePacket::ClientboundTeleportEntityPacket(_p) => {
                // println!("Got teleport entity packet {:?}", p);
                // let state_lock = state.lock()?;

                // let entity = state_lock
                //     .world
                //     .unwrap()
                //     .entity_by_id(p.id)
                //     .ok_or("Teleporting entity that doesn't exist.".to_string())?;
                // state_lock
                //     .world
                //     .as_mut()
                //     .expect("World doesn't exist! We should've gotten a login packet by now.")
                //     .move_entity(&mut entity, new_pos)
            }
            GamePacket::ClientboundUpdateAdvancementsPacket(p) => {
                println!("Got update advancements packet {:?}", p);
            }
            GamePacket::ClientboundRotateHeadPacket(_p) => {
                // println!("Got rotate head packet {:?}", p);
            }
            GamePacket::ClientboundMoveEntityPosPacket(_p) => {
                // println!("Got move entity pos packet {:?}", p);
            }
            GamePacket::ClientboundMoveEntityPosRotPacket(_p) => {
                // println!("Got move entity pos rot packet {:?}", p);
            }
            GamePacket::ClientboundMoveEntityRotPacket(p) => {
                println!("Got move entity rot packet {:?}", p);
            }
            GamePacket::ClientboundKeepAlivePacket(p) => {
                println!("Got keep alive packet {:?}", p);
                conn.lock()
                    .await
                    .write(ServerboundKeepAlivePacket { id: p.id }.get())
                    .await;
            }
            GamePacket::ClientboundRemoveEntitiesPacket(p) => {
                println!("Got remove entities packet {:?}", p);
            }
            GamePacket::ClientboundPlayerChatPacket(p) => {
                println!("Got player chat packet {:?}", p);
                tx.send(Event::Chat(ChatPacket::Player(Box::new(p.clone()))))
                    .unwrap();
            }
            GamePacket::ClientboundSystemChatPacket(p) => {
                println!("Got system chat packet {:?}", p);
                tx.send(Event::Chat(ChatPacket::System(p.clone()))).unwrap();
            }
            GamePacket::ClientboundSoundPacket(p) => {
                println!("Got sound packet {:?}", p);
            }
            GamePacket::ClientboundLevelEventPacket(p) => {
                println!("Got level event packet {:?}", p);
            }
            GamePacket::ClientboundBlockUpdatePacket(p) => {
                println!("Got block update packet {:?}", p);
                // TODO: update world
            }
            GamePacket::ClientboundAnimatePacket(p) => {
                println!("Got animate packet {:?}", p);
            }
            GamePacket::ClientboundSectionBlocksUpdatePacket(p) => {
                println!("Got section blocks update packet {:?}", p);
                // TODO: update world
            }
            GamePacket::ClientboundGameEventPacket(p) => {
                println!("Got game event packet {:?}", p);
            }
            GamePacket::ClientboundLevelParticlesPacket(p) => {
                println!("Got level particles packet {:?}", p);
            }
            GamePacket::ClientboundServerDataPacket(p) => {
                println!("Got server data packet {:?}", p);
            }
            GamePacket::ClientboundSetEquipmentPacket(p) => {
                println!("Got set equipment packet {:?}", p);
            }
            _ => panic!("Unexpected packet {:?}", packet),
        }

        Ok(())
    }

    pub async fn next(&mut self) -> Option<Event> {
        self.event_receiver.recv().await
    }

    /// Gets the `World` the client is in.
    ///
    /// This is basically a shortcut for `let world = client.state.lock().unwrap().world.as_ref().unwrap()`.
    /// If the client hasn't received a login packet yet, this will panic.
    pub fn world(&self) -> OwningRef<std::sync::MutexGuard<ClientState>, World> {
        let state_lock: std::sync::MutexGuard<ClientState> = self.state.lock().unwrap();
        let state_lock_ref = OwningRef::new(state_lock);
        state_lock_ref.map(|state| state.world.as_ref().expect("World doesn't exist!"))
    }
}

impl<T> From<std::sync::PoisonError<T>> for HandleError {
    fn from(e: std::sync::PoisonError<T>) -> Self {
        HandleError(e.to_string())
    }
}

impl From<String> for HandleError {
    fn from(e: String) -> Self {
        HandleError(e)
    }
}
