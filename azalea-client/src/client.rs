use crate::{movement::WalkDirection, plugins::Plugins, Account, Player};
use azalea_auth::game_profile::GameProfile;
use azalea_chat::Component;
use azalea_core::{ChunkPos, ResourceLocation, Vec3};
use azalea_protocol::{
    connect::{Connection, ConnectionError, ReadConnection, WriteConnection},
    packets::{
        game::{
            clientbound_player_chat_packet::ClientboundPlayerChatPacket,
            clientbound_system_chat_packet::ClientboundSystemChatPacket,
            serverbound_accept_teleportation_packet::ServerboundAcceptTeleportationPacket,
            serverbound_client_information_packet::ServerboundClientInformationPacket,
            serverbound_custom_payload_packet::ServerboundCustomPayloadPacket,
            serverbound_keep_alive_packet::ServerboundKeepAlivePacket,
            serverbound_move_player_pos_rot_packet::ServerboundMovePlayerPosRotPacket,
            ClientboundGamePacket, ServerboundGamePacket,
        },
        handshake::client_intention_packet::ClientIntentionPacket,
        login::{
            serverbound_custom_query_packet::ServerboundCustomQueryPacket,
            serverbound_hello_packet::ServerboundHelloPacket,
            serverbound_key_packet::{NonceOrSaltSignature, ServerboundKeyPacket},
            ClientboundLoginPacket,
        },
        ConnectionProtocol, PROTOCOL_VERSION,
    },
    read::ReadPacketError,
    resolver, ServerAddress,
};
use azalea_world::{
    entity::{metadata, Entity, EntityData, EntityMetadata},
    World,
};
use log::{debug, error, info, warn};
use parking_lot::{Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::{
    fmt::Debug,
    io::{self, Cursor},
    sync::Arc,
};
use thiserror::Error;
use tokio::{
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
    time::{self},
};

pub type ClientInformation = ServerboundClientInformationPacket;

/// Events are sent before they're processed, so for example game ticks happen
/// at the beginning of a tick before anything has happened.
#[derive(Debug, Clone)]
pub enum Event {
    /// Happens right after the bot switches into the Game state, but before
    /// it's actually spawned. This can be useful for setting the client
    /// information with `Client::set_client_information`, so the packet
    /// doesn't have to be sent twice.
    Initialize,
    Login,
    Chat(ChatPacket),
    /// Happens 20 times per second, but only when the world is loaded.
    Tick,
    Packet(Box<ClientboundGamePacket>),
}

/// A chat packet, either a system message or a chat message.
#[derive(Debug, Clone)]
pub enum ChatPacket {
    System(ClientboundSystemChatPacket),
    Player(Box<ClientboundPlayerChatPacket>),
}

impl ChatPacket {
    /// Get the message shown in chat for this packet.
    pub fn message(&self) -> Component {
        match self {
            ChatPacket::System(p) => p.content.clone(),
            ChatPacket::Player(p) => p.message(false),
        }
    }
}

/// A player that you control that is currently in a Minecraft server.
#[derive(Clone)]
pub struct Client {
    game_profile: GameProfile,
    pub read_conn: Arc<tokio::sync::Mutex<ReadConnection<ClientboundGamePacket>>>,
    pub write_conn: Arc<tokio::sync::Mutex<WriteConnection<ServerboundGamePacket>>>,
    pub player: Arc<RwLock<Player>>,
    pub world: Arc<RwLock<World>>,
    pub physics_state: Arc<Mutex<PhysicsState>>,
    pub client_information: Arc<RwLock<ClientInformation>>,
    /// Plugins are a way for other crates to add custom functionality to the
    /// client and keep state. If you're not making a plugin and you're using
    /// the `azalea` crate. you can ignore this field.
    pub plugins: Arc<Plugins>,
    tasks: Arc<Mutex<Vec<JoinHandle<()>>>>,
}

#[derive(Default)]
pub struct PhysicsState {
    /// Minecraft only sends a movement packet either after 20 ticks or if the player moved enough. This is that tick counter.
    pub position_remainder: u32,
    pub was_sprinting: bool,
    // Whether we're going to try to start sprinting this tick. Equivalent to
    // holding down ctrl for a tick.
    pub trying_to_sprint: bool,

    pub move_direction: WalkDirection,
    pub forward_impulse: f32,
    pub left_impulse: f32,
}

/// Whether we should ignore errors when decoding packets.
const IGNORE_ERRORS: bool = !cfg!(debug_assertions);

#[derive(Error, Debug)]
pub enum JoinError {
    #[error("{0}")]
    Resolver(#[from] resolver::ResolverError),
    #[error("{0}")]
    Connection(#[from] ConnectionError),
    #[error("{0}")]
    ReadPacket(#[from] azalea_protocol::read::ReadPacketError),
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    SessionServer(#[from] azalea_auth::sessionserver::SessionServerError),
    #[error("The given address could not be parsed into a ServerAddress")]
    InvalidAddress,
}

#[derive(Error, Debug)]
pub enum HandleError {
    #[error("{0}")]
    Poison(String),
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl Client {
    /// Connect to a Minecraft server.
    ///
    /// To change the render distance and other settings, use
    /// [`Client::set_client_information`]. To watch for events like packets
    /// sent by the server, use the `rx` variable this function returns.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use azalea_client::Client;
    ///
    /// #[tokio::main]
    /// async fn main() -> Box<dyn std::error::Error> {
    ///     let account = Account::offline("bot");
    ///     let (client, rx) = Client::join(&account, "localhost").await?;
    ///     client.chat("Hello, world!").await?;
    ///     client.shutdown().await?;
    /// }
    /// ```
    pub async fn join(
        account: &Account,
        address: impl TryInto<ServerAddress>,
    ) -> Result<(Self, UnboundedReceiver<Event>), JoinError> {
        let address: ServerAddress = address.try_into().map_err(|_| JoinError::InvalidAddress)?;

        let resolved_address = resolver::resolve_address(&address).await?;

        let mut conn = Connection::new(&resolved_address).await?;

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
        .await?;
        let mut conn = conn.login();

        // login
        conn.write(
            ServerboundHelloPacket {
                username: account.username.clone(),
                public_key: None,
                profile_id: None,
            }
            .get(),
        )
        .await?;

        let (conn, game_profile) = loop {
            let packet = conn.read().await?;
            match packet {
                ClientboundLoginPacket::Hello(p) => {
                    debug!("Got encryption request");
                    let e = azalea_crypto::encrypt(&p.public_key, &p.nonce).unwrap();

                    if let Some(access_token) = &account.access_token {
                        conn.authenticate(
                            access_token,
                            &account
                                .uuid
                                .expect("Uuid must be present if access token is present."),
                            e.secret_key,
                            p,
                        )
                        .await?;
                    }

                    conn.write(
                        ServerboundKeyPacket {
                            nonce_or_salt_signature: NonceOrSaltSignature::Nonce(e.encrypted_nonce),
                            key_bytes: e.encrypted_public_key,
                        }
                        .get(),
                    )
                    .await?;

                    conn.set_encryption_key(e.secret_key);
                }
                ClientboundLoginPacket::LoginCompression(p) => {
                    debug!("Got compression request {:?}", p.compression_threshold);
                    conn.set_compression_threshold(p.compression_threshold);
                }
                ClientboundLoginPacket::GameProfile(p) => {
                    debug!("Got profile {:?}", p.game_profile);
                    break (conn.game(), p.game_profile);
                }
                ClientboundLoginPacket::LoginDisconnect(p) => {
                    debug!("Got disconnect {:?}", p);
                }
                ClientboundLoginPacket::CustomQuery(p) => {
                    debug!("Got custom query {:?}", p);
                    conn.write(
                        ServerboundCustomQueryPacket {
                            transaction_id: p.transaction_id,
                            data: None,
                        }
                        .get(),
                    )
                    .await?;
                }
            }
        };

        let (read_conn, write_conn) = conn.into_split();

        let read_conn = Arc::new(tokio::sync::Mutex::new(read_conn));
        let write_conn = Arc::new(tokio::sync::Mutex::new(write_conn));

        let (tx, rx) = mpsc::unbounded_channel();

        // we got the GameConnection, so the server is now connected :)
        let client = Client {
            game_profile,
            read_conn,
            write_conn,
            player: Arc::new(RwLock::new(Player::default())),
            world: Arc::new(RwLock::new(World::default())),
            physics_state: Arc::new(Mutex::new(PhysicsState::default())),
            client_information: Arc::new(RwLock::new(ClientInformation::default())),
            // The plugins can be modified by the user by replacing the plugins
            // field right after this. No Mutex so the user doesn't need to .lock().
            plugins: Arc::new(Plugins::new()),
            tasks: Arc::new(Mutex::new(Vec::new())),
        };

        tx.send(Event::Initialize).unwrap();

        // just start up the game loop and we're ready!

        // if you get an error right here that means you're doing something with locks wrong
        // read the error to see where the issue is
        // you might be able to just drop the lock or put it in its own scope to fix
        {
            let mut tasks = client.tasks.lock();
            tasks.push(tokio::spawn(Self::protocol_loop(
                client.clone(),
                tx.clone(),
            )));
            tasks.push(tokio::spawn(Self::game_tick_loop(client.clone(), tx)));
        }

        Ok((client, rx))
    }

    /// Write a packet directly to the server.
    pub async fn write_packet(&self, packet: ServerboundGamePacket) -> Result<(), std::io::Error> {
        self.write_conn.lock().await.write(packet).await?;
        Ok(())
    }

    /// Disconnect from the server, ending all tasks.
    pub async fn shutdown(self) -> Result<(), std::io::Error> {
        self.write_conn.lock().await.shutdown().await?;
        let tasks = self.tasks.lock();
        for task in tasks.iter() {
            task.abort();
        }
        Ok(())
    }

    async fn protocol_loop(client: Client, tx: UnboundedSender<Event>) {
        loop {
            let r = client.read_conn.lock().await.read().await;
            match r {
                Ok(packet) => match Self::handle(&packet, &client, &tx).await {
                    Ok(_) => {}
                    Err(e) => {
                        error!("Error handling packet: {}", e);
                        if IGNORE_ERRORS {
                            continue;
                        } else {
                            panic!("Error handling packet: {e}");
                        }
                    }
                },
                Err(e) => {
                    if let ReadPacketError::ConnectionClosed = e {
                        info!("Connection closed");
                        if let Err(e) = client.shutdown().await {
                            error!("Error shutting down connection: {:?}", e);
                        }
                        return;
                    }
                    if IGNORE_ERRORS {
                        warn!("{}", e);
                        match e {
                            ReadPacketError::FrameSplitter { .. } => panic!("Error: {e:?}"),
                            _ => continue,
                        }
                    } else {
                        panic!("{}", e);
                    }
                }
            };
        }
    }

    async fn handle(
        packet: &ClientboundGamePacket,
        client: &Client,
        tx: &UnboundedSender<Event>,
    ) -> Result<(), HandleError> {
        tx.send(Event::Packet(Box::new(packet.clone()))).unwrap();
        match packet {
            ClientboundGamePacket::Login(p) => {
                debug!("Got login packet {:?}", p);

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

                    let mut world_lock = client.world.write();
                    // the 16 here is our render distance
                    // i'll make this an actual setting later
                    *world_lock = World::new(16, height, min_y);

                    let entity = EntityData::new(
                        client.game_profile.uuid,
                        Vec3::default(),
                        EntityMetadata::Player(metadata::Player::default()),
                    );
                    world_lock.add_entity(p.player_id, entity);

                    let mut player_lock = client.player.write();

                    player_lock.set_entity_id(p.player_id);
                }

                // send the client information that we have set
                let client_information_packet: ClientInformation =
                    client.client_information.read().clone();
                log::debug!(
                    "Sending client information because login: {:?}",
                    client_information_packet
                );
                client.write_packet(client_information_packet.get()).await?;

                // brand
                client
                    .write_packet(
                        ServerboundCustomPayloadPacket {
                            identifier: ResourceLocation::new("brand").unwrap(),
                            // they don't have to know :)
                            data: "vanilla".into(),
                        }
                        .get(),
                    )
                    .await?;

                tx.send(Event::Login).unwrap();
            }
            ClientboundGamePacket::SetChunkCacheRadius(p) => {
                debug!("Got set chunk cache radius packet {:?}", p);
            }
            ClientboundGamePacket::CustomPayload(p) => {
                debug!("Got custom payload packet {:?}", p);
            }
            ClientboundGamePacket::ChangeDifficulty(p) => {
                debug!("Got difficulty packet {:?}", p);
            }
            ClientboundGamePacket::Commands(_p) => {
                debug!("Got declare commands packet");
            }
            ClientboundGamePacket::PlayerAbilities(p) => {
                debug!("Got player abilities packet {:?}", p);
            }
            ClientboundGamePacket::SetCarriedItem(p) => {
                debug!("Got set carried item packet {:?}", p);
            }
            ClientboundGamePacket::UpdateTags(_p) => {
                debug!("Got update tags packet");
            }
            ClientboundGamePacket::Disconnect(p) => {
                debug!("Got disconnect packet {:?}", p);
            }
            ClientboundGamePacket::UpdateRecipes(_p) => {
                debug!("Got update recipes packet");
            }
            ClientboundGamePacket::EntityEvent(_p) => {
                // debug!("Got entity event packet {:?}", p);
            }
            ClientboundGamePacket::Recipe(_p) => {
                debug!("Got recipe packet");
            }
            ClientboundGamePacket::PlayerPosition(p) => {
                // TODO: reply with teleport confirm
                debug!("Got player position packet {:?}", p);

                let (new_pos, y_rot, x_rot) = {
                    let player_entity_id = {
                        let player_lock = client.player.write();
                        player_lock.entity_id
                    };

                    let mut world_lock = client.world.write();

                    let mut player_entity = world_lock
                        .entity_mut(player_entity_id)
                        .expect("Player entity doesn't exist");

                    let delta_movement = player_entity.delta;

                    let is_x_relative = p.relative_arguments.x;
                    let is_y_relative = p.relative_arguments.y;
                    let is_z_relative = p.relative_arguments.z;

                    let (delta_x, new_pos_x) = if is_x_relative {
                        player_entity.last_pos.x += p.x;
                        (delta_movement.x, player_entity.pos().x + p.x)
                    } else {
                        player_entity.last_pos.x = p.x;
                        (0.0, p.x)
                    };
                    let (delta_y, new_pos_y) = if is_y_relative {
                        player_entity.last_pos.y += p.y;
                        (delta_movement.y, player_entity.pos().y + p.y)
                    } else {
                        player_entity.last_pos.y = p.y;
                        (0.0, p.y)
                    };
                    let (delta_z, new_pos_z) = if is_z_relative {
                        player_entity.last_pos.z += p.z;
                        (delta_movement.z, player_entity.pos().z + p.z)
                    } else {
                        player_entity.last_pos.z = p.z;
                        (0.0, p.z)
                    };

                    let mut y_rot = p.y_rot;
                    let mut x_rot = p.x_rot;
                    if p.relative_arguments.x_rot {
                        x_rot += player_entity.x_rot;
                    }
                    if p.relative_arguments.y_rot {
                        y_rot += player_entity.y_rot;
                    }

                    player_entity.delta = Vec3 {
                        x: delta_x,
                        y: delta_y,
                        z: delta_z,
                    };
                    player_entity.set_rotation(y_rot, x_rot);
                    // TODO: minecraft sets "xo", "yo", and "zo" here but idk what that means
                    // so investigate that ig
                    let new_pos = Vec3 {
                        x: new_pos_x,
                        y: new_pos_y,
                        z: new_pos_z,
                    };
                    world_lock
                        .set_entity_pos(player_entity_id, new_pos)
                        .expect("The player entity should always exist");

                    (new_pos, y_rot, x_rot)
                };

                client
                    .write_packet(ServerboundAcceptTeleportationPacket { id: p.id }.get())
                    .await?;
                client
                    .write_packet(
                        ServerboundMovePlayerPosRotPacket {
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
                    .await?;
            }
            ClientboundGamePacket::PlayerInfo(p) => {
                debug!("Got player info packet {:?}", p);
            }
            ClientboundGamePacket::SetChunkCacheCenter(p) => {
                debug!("Got chunk cache center packet {:?}", p);
                client
                    .world
                    .write()
                    .update_view_center(&ChunkPos::new(p.x, p.z));
            }
            ClientboundGamePacket::LevelChunkWithLight(p) => {
                debug!("Got chunk with light packet {} {}", p.x, p.z);
                let pos = ChunkPos::new(p.x, p.z);
                // let chunk = Chunk::read_with_world_height(&mut p.chunk_data);
                // debug("chunk {:?}")
                if let Err(e) = client
                    .world
                    .write()
                    .replace_with_packet_data(&pos, &mut Cursor::new(&p.chunk_data.data))
                {
                    error!("Couldn't set chunk data: {}", e);
                }
            }
            ClientboundGamePacket::LightUpdate(_p) => {
                // debug!("Got light update packet {:?}", p);
            }
            ClientboundGamePacket::AddEntity(p) => {
                debug!("Got add entity packet {:?}", p);
                let entity = EntityData::from(p);
                client.world.write().add_entity(p.id, entity);
            }
            ClientboundGamePacket::SetEntityData(p) => {
                debug!("Got set entity data packet {:?}", p);
                let mut world = client.world.write();
                if let Some(mut entity) = world.entity_mut(p.id) {
                    entity.apply_metadata(&p.packed_items.0);
                } else {
                    warn!("Server sent an entity data packet for an entity id ({}) that we don't know about", p.id);
                }
            }
            ClientboundGamePacket::UpdateAttributes(_p) => {
                // debug!("Got update attributes packet {:?}", p);
            }
            ClientboundGamePacket::SetEntityMotion(_p) => {
                // debug!("Got entity velocity packet {:?}", p);
            }
            ClientboundGamePacket::SetEntityLink(p) => {
                debug!("Got set entity link packet {:?}", p);
            }
            ClientboundGamePacket::AddPlayer(p) => {
                debug!("Got add player packet {:?}", p);
                let entity = EntityData::from(p);
                client.world.write().add_entity(p.id, entity);
            }
            ClientboundGamePacket::InitializeBorder(p) => {
                debug!("Got initialize border packet {:?}", p);
            }
            ClientboundGamePacket::SetTime(p) => {
                debug!("Got set time packet {:?}", p);
            }
            ClientboundGamePacket::SetDefaultSpawnPosition(p) => {
                debug!("Got set default spawn position packet {:?}", p);
            }
            ClientboundGamePacket::ContainerSetContent(p) => {
                debug!("Got container set content packet {:?}", p);
            }
            ClientboundGamePacket::SetHealth(p) => {
                debug!("Got set health packet {:?}", p);
            }
            ClientboundGamePacket::SetExperience(p) => {
                debug!("Got set experience packet {:?}", p);
            }
            ClientboundGamePacket::TeleportEntity(p) => {
                let mut world_lock = client.world.write();

                world_lock
                    .set_entity_pos(
                        p.id,
                        Vec3 {
                            x: p.x,
                            y: p.y,
                            z: p.z,
                        },
                    )
                    .map_err(|e| HandleError::Other(e.into()))?;
            }
            ClientboundGamePacket::UpdateAdvancements(p) => {
                debug!("Got update advancements packet {:?}", p);
            }
            ClientboundGamePacket::RotateHead(_p) => {
                // debug!("Got rotate head packet {:?}", p);
            }
            ClientboundGamePacket::MoveEntityPos(p) => {
                let mut world_lock = client.world.write();

                world_lock
                    .move_entity_with_delta(p.entity_id, &p.delta)
                    .map_err(|e| HandleError::Other(e.into()))?;
            }
            ClientboundGamePacket::MoveEntityPosRot(p) => {
                let mut world_lock = client.world.write();

                world_lock
                    .move_entity_with_delta(p.entity_id, &p.delta)
                    .map_err(|e| HandleError::Other(e.into()))?;
            }
            ClientboundGamePacket::MoveEntityRot(_p) => {
                // debug!("Got move entity rot packet {:?}", p);
            }
            ClientboundGamePacket::KeepAlive(p) => {
                debug!("Got keep alive packet {:?}", p);
                client
                    .write_packet(ServerboundKeepAlivePacket { id: p.id }.get())
                    .await?;
            }
            ClientboundGamePacket::RemoveEntities(p) => {
                debug!("Got remove entities packet {:?}", p);
            }
            ClientboundGamePacket::PlayerChat(p) => {
                // debug!("Got player chat packet {:?}", p);
                tx.send(Event::Chat(ChatPacket::Player(Box::new(p.clone()))))
                    .unwrap();
            }
            ClientboundGamePacket::SystemChat(p) => {
                debug!("Got system chat packet {:?}", p);
                tx.send(Event::Chat(ChatPacket::System(p.clone()))).unwrap();
            }
            ClientboundGamePacket::Sound(p) => {
                debug!("Got sound packet {:?}", p);
            }
            ClientboundGamePacket::LevelEvent(p) => {
                debug!("Got level event packet {:?}", p);
            }
            ClientboundGamePacket::BlockUpdate(p) => {
                debug!("Got block update packet {:?}", p);
                let mut world = client.world.write();
                world.set_block_state(&p.pos, p.block_state);
            }
            ClientboundGamePacket::Animate(p) => {
                debug!("Got animate packet {:?}", p);
            }
            ClientboundGamePacket::SectionBlocksUpdate(p) => {
                debug!("Got section blocks update packet {:?}", p);
                let mut world = client.world.write();
                for state in &p.states {
                    world.set_block_state(&(p.section_pos + state.pos.clone()), state.state);
                }
            }
            ClientboundGamePacket::GameEvent(p) => {
                debug!("Got game event packet {:?}", p);
            }
            ClientboundGamePacket::LevelParticles(p) => {
                debug!("Got level particles packet {:?}", p);
            }
            ClientboundGamePacket::ServerData(p) => {
                debug!("Got server data packet {:?}", p);
            }
            ClientboundGamePacket::SetEquipment(p) => {
                debug!("Got set equipment packet {:?}", p);
            }
            ClientboundGamePacket::UpdateMobEffect(p) => {
                debug!("Got update mob effect packet {:?}", p);
            }
            ClientboundGamePacket::AddExperienceOrb(_) => {}
            ClientboundGamePacket::AwardStats(_) => {}
            ClientboundGamePacket::BlockChangedAck(_) => {}
            ClientboundGamePacket::BlockDestruction(_) => {}
            ClientboundGamePacket::BlockEntityData(_) => {}
            ClientboundGamePacket::BlockEvent(_) => {}
            ClientboundGamePacket::BossEvent(_) => {}
            ClientboundGamePacket::ChatPreview(_) => {}
            ClientboundGamePacket::CommandSuggestions(_) => {}
            ClientboundGamePacket::ContainerSetData(_) => {}
            ClientboundGamePacket::ContainerSetSlot(_) => {}
            ClientboundGamePacket::Cooldown(_) => {}
            ClientboundGamePacket::CustomChatCompletions(_) => {}
            ClientboundGamePacket::CustomSound(_) => {}
            ClientboundGamePacket::DeleteChat(_) => {}
            ClientboundGamePacket::Explode(_) => {}
            ClientboundGamePacket::ForgetLevelChunk(_) => {}
            ClientboundGamePacket::HorseScreenOpen(_) => {}
            ClientboundGamePacket::MapItemData(_) => {}
            ClientboundGamePacket::MerchantOffers(_) => {}
            ClientboundGamePacket::MoveVehicle(_) => {}
            ClientboundGamePacket::OpenBook(_) => {}
            ClientboundGamePacket::OpenScreen(_) => {}
            ClientboundGamePacket::OpenSignEditor(_) => {}
            ClientboundGamePacket::Ping(_) => {}
            ClientboundGamePacket::PlaceGhostRecipe(_) => {}
            ClientboundGamePacket::PlayerChatHeader(_) => {}
            ClientboundGamePacket::PlayerCombatEnd(_) => {}
            ClientboundGamePacket::PlayerCombatEnter(_) => {}
            ClientboundGamePacket::PlayerCombatKill(_) => {}
            ClientboundGamePacket::PlayerLookAt(_) => {}
            ClientboundGamePacket::RemoveMobEffect(_) => {}
            ClientboundGamePacket::ResourcePack(_) => {}
            ClientboundGamePacket::Respawn(_) => {}
            ClientboundGamePacket::SelectAdvancementsTab(_) => {}
            ClientboundGamePacket::SetActionBarText(_) => {}
            ClientboundGamePacket::SetBorderCenter(_) => {}
            ClientboundGamePacket::SetBorderLerpSize(_) => {}
            ClientboundGamePacket::SetBorderSize(_) => {}
            ClientboundGamePacket::SetBorderWarningDelay(_) => {}
            ClientboundGamePacket::SetBorderWarningDistance(_) => {}
            ClientboundGamePacket::SetCamera(_) => {}
            ClientboundGamePacket::SetDisplayChatPreview(_) => {}
            ClientboundGamePacket::SetDisplayObjective(_) => {}
            ClientboundGamePacket::SetObjective(_) => {}
            ClientboundGamePacket::SetPassengers(_) => {}
            ClientboundGamePacket::SetPlayerTeam(_) => {}
            ClientboundGamePacket::SetScore(_) => {}
            ClientboundGamePacket::SetSimulationDistance(_) => {}
            ClientboundGamePacket::SetSubtitleText(_) => {}
            ClientboundGamePacket::SetTitleText(_) => {}
            ClientboundGamePacket::SetTitlesAnimation(_) => {}
            ClientboundGamePacket::SoundEntity(_) => {}
            ClientboundGamePacket::StopSound(_) => {}
            ClientboundGamePacket::TabList(_) => {}
            ClientboundGamePacket::TagQuery(_) => {}
            ClientboundGamePacket::TakeItemEntity(_) => {}
            ClientboundGamePacket::ContainerClose(_) => {}
        }

        Ok(())
    }

    /// Runs game_tick every 50 milliseconds.
    async fn game_tick_loop(mut client: Client, tx: UnboundedSender<Event>) {
        let mut game_tick_interval = time::interval(time::Duration::from_millis(50));
        // TODO: Minecraft bursts up to 10 ticks and then skips, we should too
        game_tick_interval.set_missed_tick_behavior(time::MissedTickBehavior::Burst);
        loop {
            game_tick_interval.tick().await;
            Self::game_tick(&mut client, &tx).await;
        }
    }

    /// Runs every 50 milliseconds.
    async fn game_tick(client: &mut Client, tx: &UnboundedSender<Event>) {
        // return if there's no chunk at the player's position
        {
            let world_lock = client.world.write();
            let player_lock = client.player.write();
            let player_entity = player_lock.entity(&world_lock);
            let player_entity = if let Some(player_entity) = player_entity {
                player_entity
            } else {
                return;
            };
            let player_chunk_pos: ChunkPos = player_entity.pos().into();
            if world_lock[&player_chunk_pos].is_none() {
                return;
            }
        }

        tx.send(Event::Tick).unwrap();

        // TODO: if we're a passenger, send the required packets

        if let Err(e) = client.send_position().await {
            warn!("Error sending position: {:?}", e);
        }
        client.ai_step();

        // TODO: minecraft does ambient sounds here
    }

    /// Returns the entity associated to the player.
    pub fn entity_mut(&self) -> Entity<RwLockWriteGuard<World>> {
        let entity_id = {
            let player_lock = self.player.write();
            player_lock.entity_id
        };

        let mut world = self.world.write();

        let entity_data = world
            .entity_storage
            .get_mut_by_id(entity_id)
            .expect("Player entity should exist");
        let entity_ptr = unsafe { entity_data.as_ptr() };
        Entity::new(world, entity_id, entity_ptr)
    }
    /// Returns the entity associated to the player.
    pub fn entity(&self) -> Entity<RwLockReadGuard<World>> {
        let entity_id = {
            let player_lock = self.player.read();
            player_lock.entity_id
        };

        let world = self.world.read();

        let entity_data = world
            .entity_storage
            .get_by_id(entity_id)
            .expect("Player entity should be in the given world");
        let entity_ptr = unsafe { entity_data.as_const_ptr() };
        Entity::new(world, entity_id, entity_ptr)
    }

    /// Returns whether we have a received the login packet yet.
    pub fn logged_in(&self) -> bool {
        let world = self.world.read();
        let player = self.player.write();
        player.entity(&world).is_some()
    }

    /// Tell the server we changed our game options (i.e. render distance, main hand).
    /// If this is not set before the login packet, the default will be sent.
    pub async fn set_client_information(
        &self,
        client_information: ServerboundClientInformationPacket,
    ) -> Result<(), std::io::Error> {
        {
            let mut client_information_lock = self.client_information.write();
            *client_information_lock = client_information;
        }

        if self.logged_in() {
            let client_information_packet = {
                let client_information = self.client_information.read();
                client_information.clone().get()
            };
            log::debug!(
                "Sending client information (already logged in): {:?}",
                client_information_packet
            );
            self.write_packet(client_information_packet).await?;
        }

        Ok(())
    }

    /// Get your player entity's metadata. You can use this to get your health,
    /// xp score, and other useful information.
    pub fn metadata(&self) -> metadata::Player {
        self.entity().metadata.clone().into_player().unwrap()
    }
}

impl<T> From<std::sync::PoisonError<T>> for HandleError {
    fn from(e: std::sync::PoisonError<T>) -> Self {
        HandleError::Poison(e.to_string())
    }
}
