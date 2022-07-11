use crate::{Account, Player};
use azalea_auth::game_profile::GameProfile;
use azalea_core::{ChunkPos, PositionDelta, PositionDeltaTrait, ResourceLocation, Vec3};
use azalea_protocol::{
    connect::{GameConnection, HandshakeConnection},
    packets::{
        game::{
            clientbound_player_chat_packet::ClientboundPlayerChatPacket,
            clientbound_system_chat_packet::ClientboundSystemChatPacket,
            serverbound_accept_teleportation_packet::ServerboundAcceptTeleportationPacket,
            serverbound_custom_payload_packet::ServerboundCustomPayloadPacket,
            serverbound_keep_alive_packet::ServerboundKeepAlivePacket,
            serverbound_move_player_packet_pos_rot::ServerboundMovePlayerPacketPosRot, GamePacket,
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
use azalea_world::entity::Entity;
use azalea_world::Dimension;
use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};
use tokio::{
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    time::{self},
};

#[derive(Debug, Clone)]
pub enum Event {
    Login,
    Chat(ChatPacket),
    /// A game tick, happens 20 times per second.
    GameTick,
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
#[derive(Clone)]
pub struct Client {
    game_profile: GameProfile,
    pub conn: Arc<tokio::sync::Mutex<GameConnection>>,
    pub player: Arc<Mutex<Player>>,
    pub dimension: Arc<Mutex<Option<Dimension>>>,
    // game_loop
}

/// Whether we should ignore errors when decoding packets.
const IGNORE_ERRORS: bool = !cfg!(debug_assertions);

#[derive(Debug)]
struct HandleError(String);

impl Client {
    /// Connect to a Minecraft server with an account.
    pub async fn join(
        account: &Account,
        address: &ServerAddress,
    ) -> Result<(Self, UnboundedReceiver<Event>), String> {
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

        let (conn, game_profile) = loop {
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
                        break (conn.game(), p.game_profile);
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
            game_profile,
            conn,
            player: Arc::new(Mutex::new(Player::default())),
            dimension: Arc::new(Mutex::new(None)),
        };

        // just start up the game loop and we're ready!

        // if you get an error right here that means you're doing something with locks wrong
        // read the error to see where the issue is
        // you might be able to just drop the lock or put it in its own scope to fix
        tokio::spawn(Self::protocol_loop(client.clone(), tx.clone()));
        tokio::spawn(Self::game_tick_loop(client.clone(), tx));

        Ok((client, rx))
    }

    async fn protocol_loop(client: Client, tx: UnboundedSender<Event>) {
        loop {
            let r = client.conn.lock().await.read().await;
            match r {
                Ok(packet) => match Self::handle(&packet, &client, &tx).await {
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
        client: &Client,
        tx: &UnboundedSender<Event>,
    ) -> Result<(), HandleError> {
        match packet {
            GamePacket::ClientboundLoginPacket(p) => {
                println!("Got login packet {:?}", p);

                {
                    // // write p into login.txt
                    // std::io::Write::write_all(
                    //     &mut std::fs::File::create("login.txt").unwrap(),
                    //     format!("{:#?}", p).as_bytes(),
                    // )
                    // .unwrap();

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

                    let mut dimension_lock = client.dimension.lock().unwrap();
                    // the 16 here is our render distance
                    // i'll make this an actual setting later
                    *dimension_lock = Some(Dimension::new(16, height, min_y));

                    let entity =
                        Entity::new(p.player_id, client.game_profile.uuid, Vec3::default());
                    dimension_lock
                        .as_mut()
                        .expect(
                            "Dimension doesn't exist! We should've gotten a login packet by now.",
                        )
                        .add_entity(entity);

                    let mut player_lock = client.player.lock().unwrap();

                    player_lock.set_entity_id(p.player_id);
                }

                client
                    .conn
                    .lock()
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

                let (new_pos, y_rot, x_rot) = {
                    let player_lock = client.player.lock().unwrap();
                    let player_entity_id = player_lock.entity_id;
                    drop(player_lock);

                    let mut dimension_lock = client.dimension.lock().unwrap();
                    let dimension = dimension_lock.as_mut().unwrap();

                    let player_entity = dimension
                        .mut_entity_by_id(player_entity_id)
                        .expect("Player entity doesn't exist");

                    let delta_movement = &player_entity.delta;

                    let is_x_relative = p.relative_arguments.x;
                    let is_y_relative = p.relative_arguments.y;
                    let is_z_relative = p.relative_arguments.z;

                    let (delta_x, new_pos_x) = if is_x_relative {
                        player_entity.old_pos.x += p.x;
                        (delta_movement.x(), player_entity.pos().x + p.x)
                    } else {
                        player_entity.old_pos.x = p.x;
                        (0.0, p.x)
                    };
                    let (delta_y, new_pos_y) = if is_y_relative {
                        player_entity.old_pos.y += p.y;
                        (delta_movement.y(), player_entity.pos().y + p.y)
                    } else {
                        player_entity.old_pos.y = p.y;
                        (0.0, p.y)
                    };
                    let (delta_z, new_pos_z) = if is_z_relative {
                        player_entity.old_pos.z += p.z;
                        (delta_movement.z(), player_entity.pos().z + p.z)
                    } else {
                        player_entity.old_pos.z = p.z;
                        (0.0, p.z)
                    };

                    let mut y_rot = p.y_rot;
                    let mut x_rot = p.x_rot;
                    if p.relative_arguments.x_rot {
                        y_rot += player_entity.x_rot;
                    }
                    if p.relative_arguments.y_rot {
                        x_rot += player_entity.y_rot;
                    }

                    player_entity.delta = PositionDelta {
                        xa: delta_x,
                        ya: delta_y,
                        za: delta_z,
                    };
                    player_entity.set_rotation(y_rot, x_rot);
                    // TODO: minecraft sets "xo", "yo", and "zo" here but idk what that means
                    // so investigate that ig
                    let new_pos = Vec3 {
                        x: new_pos_x,
                        y: new_pos_y,
                        z: new_pos_z,
                    };
                    dimension
                        .move_entity(player_entity_id, new_pos)
                        .expect("The player entity should always exist");

                    (new_pos, y_rot, x_rot)
                };

                let mut conn_lock = client.conn.lock().await;
                conn_lock
                    .write(ServerboundAcceptTeleportationPacket { id: p.id }.get())
                    .await;
                conn_lock
                    .write(
                        ServerboundMovePlayerPacketPosRot {
                            x: new_pos.x,
                            y: new_pos.y,
                            z: new_pos.z,
                            y_rot,
                            x_rot,
                            // this is always false
                            on_ground: false,
                        }
                        .get(),
                    )
                    .await;
            }
            GamePacket::ClientboundPlayerInfoPacket(p) => {
                println!("Got player info packet {:?}", p);
            }
            GamePacket::ClientboundSetChunkCacheCenterPacket(p) => {
                println!("Got chunk cache center packet {:?}", p);
                client
                    .dimension
                    .lock()?
                    .as_mut()
                    .unwrap()
                    .update_view_center(&ChunkPos::new(p.x, p.z));
            }
            GamePacket::ClientboundLevelChunkWithLightPacket(p) => {
                println!("Got chunk with light packet {} {}", p.x, p.z);
                let pos = ChunkPos::new(p.x, p.z);
                // let chunk = Chunk::read_with_world_height(&mut p.chunk_data);
                // println("chunk {:?}")
                client
                    .dimension
                    .lock()?
                    .as_mut()
                    .expect("Dimension doesn't exist! We should've gotten a login packet by now.")
                    .replace_with_packet_data(&pos, &mut p.chunk_data.data.as_slice())
                    .unwrap();
            }
            GamePacket::ClientboundLightUpdatePacket(p) => {
                println!("Got light update packet {:?}", p);
            }
            GamePacket::ClientboundAddEntityPacket(p) => {
                println!("Got add entity packet {:?}", p);
                let entity = Entity::from(p);
                client
                    .dimension
                    .lock()?
                    .as_mut()
                    .expect("Dimension doesn't exist! We should've gotten a login packet by now.")
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
                let entity = Entity::from(p);
                client
                    .dimension
                    .lock()?
                    .as_mut()
                    .expect("Dimension doesn't exist! We should've gotten a login packet by now.")
                    .add_entity(entity);
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
            GamePacket::ClientboundTeleportEntityPacket(p) => {
                let mut dimension_lock = client.dimension.lock()?;
                let dimension = dimension_lock.as_mut().unwrap();

                dimension.move_entity(
                    p.id,
                    Vec3 {
                        x: p.x,
                        y: p.y,
                        z: p.z,
                    },
                )?;
            }
            GamePacket::ClientboundUpdateAdvancementsPacket(p) => {
                println!("Got update advancements packet {:?}", p);
            }
            GamePacket::ClientboundRotateHeadPacket(_p) => {
                // println!("Got rotate head packet {:?}", p);
            }
            GamePacket::ClientboundMoveVec3Packet(p) => {
                let mut dimension_lock = client.dimension.lock()?;
                let dimension = dimension_lock.as_mut().unwrap();

                dimension.move_entity_with_delta(p.entity_id, &p.delta)?;
            }
            GamePacket::ClientboundMoveVec3RotPacket(p) => {
                let mut dimension_lock = client.dimension.lock()?;
                let dimension = dimension_lock.as_mut().unwrap();

                dimension.move_entity_with_delta(p.entity_id, &p.delta)?;
            }
            GamePacket::ClientboundMoveEntityRotPacket(p) => {
                println!("Got move entity rot packet {:?}", p);
            }
            GamePacket::ClientboundKeepAlivePacket(p) => {
                println!("Got keep alive packet {:?}", p);
                client
                    .conn
                    .lock()
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
            GamePacket::ClientboundUpdateMobEffectPacket(p) => {
                println!("Got update mob effect packet {:?}", p);
            }
            _ => panic!("Unexpected packet {:?}", packet),
        }

        Ok(())
    }

    /// Runs game_tick every 50 milliseconds.
    async fn game_tick_loop(client: Client, tx: UnboundedSender<Event>) {
        let mut game_tick_interval = time::interval(time::Duration::from_millis(50));
        // TODO: Minecraft bursts up to 10 ticks and then skips, we should too
        game_tick_interval.set_missed_tick_behavior(time::MissedTickBehavior::Burst);
        loop {
            game_tick_interval.tick().await;
            Self::game_tick(&client, &tx).await;
        }
    }

    /// Runs every 50 milliseconds.
    async fn game_tick(client: &Client, tx: &UnboundedSender<Event>) {
        if client.dimension.lock().unwrap().is_none() {
            return;
        }
        tx.send(Event::GameTick).unwrap();
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
