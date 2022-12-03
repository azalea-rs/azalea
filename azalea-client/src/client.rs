pub use crate::chat::ChatPacket;
use crate::{movement::WalkDirection, plugins::PluginStates, Account, PlayerInfo};
use azalea_auth::game_profile::GameProfile;
use azalea_chat::Component;
use azalea_core::{ChunkPos, GameType, ResourceLocation, Vec3};
use azalea_protocol::{
    connect::{Connection, ConnectionError, ReadConnection, WriteConnection},
    packets::{
        game::{
            clientbound_player_combat_kill_packet::ClientboundPlayerCombatKillPacket,
            serverbound_accept_teleportation_packet::ServerboundAcceptTeleportationPacket,
            serverbound_client_information_packet::ServerboundClientInformationPacket,
            serverbound_custom_payload_packet::ServerboundCustomPayloadPacket,
            serverbound_keep_alive_packet::ServerboundKeepAlivePacket,
            serverbound_move_player_pos_rot_packet::ServerboundMovePlayerPosRotPacket,
            ClientboundGamePacket, ServerboundGamePacket,
        },
        handshake::{
            client_intention_packet::ClientIntentionPacket, ClientboundHandshakePacket,
            ServerboundHandshakePacket,
        },
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
    WeakWorld, WeakWorldContainer, World,
};
use log::{debug, error, info, trace, warn};
use parking_lot::{Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::{
    collections::HashMap,
    fmt::Debug,
    io::{self, Cursor},
    sync::Arc,
};
use thiserror::Error;
use tokio::{
    sync::mpsc::{self, Receiver, Sender},
    task::JoinHandle,
    time::{self},
};
use uuid::Uuid;

pub type ClientInformation = ServerboundClientInformationPacket;

/// Events are sent before they're processed, so for example game ticks happen
/// at the beginning of a tick before anything has happened.
#[derive(Debug, Clone)]
pub enum Event {
    /// Happens right after the bot switches into the Game state, but before
    /// it's actually spawned. This can be useful for setting the client
    /// information with `Client::set_client_information`, so the packet
    /// doesn't have to be sent twice.
    Init,
    Login,
    Chat(ChatPacket),
    /// Happens 20 times per second, but only when the world is loaded.
    Tick,
    Packet(Arc<ClientboundGamePacket>),
    /// Happens when a player is added, removed, or updated in the tab list.
    UpdatePlayers(UpdatePlayersEvent),
    /// Emits when the player dies.
    Death(Option<Arc<ClientboundPlayerCombatKillPacket>>),
}

/// Happens when a player is added, removed, or updated in the tab list.
#[derive(Debug, Clone)]
pub enum UpdatePlayersEvent {
    /// A player with the given info was added to the tab list (usually means
    /// they joined the server).
    Add(PlayerInfo),
    /// A player with the given UUID was removed from the tab list (usually
    /// means they left the server)
    Remove { uuid: Uuid },
    /// The latency of the player with the given UUID was updated in the tab
    /// list. Note that this can be spoofed by the player and may not represent
    /// their actual latency.
    Latency {
        uuid: Uuid,
        /// The time it took in milliseconds for this player to reply to the ping packet.
        latency: i32,
    },
    /// The played switched to a different gamemode (i.e. survival, creative, spectator)
    GameMode { uuid: Uuid, game_mode: GameType },
    /// The name of the player with the given UUID in the tab list was changed or reset.
    DisplayName {
        uuid: Uuid,
        display_name: Option<Component>,
    },
}

/// A player that you control that is currently in a Minecraft server.
#[derive(Clone)]
pub struct Client {
    pub profile: GameProfile,
    pub read_conn: Arc<tokio::sync::Mutex<ReadConnection<ClientboundGamePacket>>>,
    pub write_conn: Arc<tokio::sync::Mutex<WriteConnection<ServerboundGamePacket>>>,
    pub entity_id: Arc<RwLock<u32>>,
    /// The world that this client has access to. This supports shared worlds.
    pub world: Arc<RwLock<World>>,
    /// A container of world names to worlds. If we're not using a shared world
    /// (i.e. not a swarm), then this will only contain data about the world
    /// we're currently in.
    world_container: Arc<RwLock<WeakWorldContainer>>,
    pub world_name: Arc<RwLock<Option<ResourceLocation>>>,
    pub physics_state: Arc<Mutex<PhysicsState>>,
    pub client_information: Arc<RwLock<ClientInformation>>,
    pub dead: Arc<Mutex<bool>>,
    /// Plugins are a way for other crates to add custom functionality to the
    /// client and keep state. If you're not making a plugin and you're using
    /// the `azalea` crate. you can ignore this field.
    pub plugins: Arc<PluginStates>,
    /// A map of player uuids to their information in the tab list
    pub players: Arc<RwLock<HashMap<Uuid, PlayerInfo>>>,
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
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
    #[error("{0}")]
    Send(#[from] mpsc::error::SendError<Event>),
}

impl Client {
    /// Create a new client from the given GameProfile, Connection, and World.
    /// You should only use this if you want to change these fields from the
    /// defaults, otherwise use [`Client::join`].
    pub fn new(
        profile: GameProfile,
        conn: Connection<ClientboundGamePacket, ServerboundGamePacket>,
        world_container: Option<Arc<RwLock<WeakWorldContainer>>>,
    ) -> Self {
        let (read_conn, write_conn) = conn.into_split();
        let (read_conn, write_conn) = (
            Arc::new(tokio::sync::Mutex::new(read_conn)),
            Arc::new(tokio::sync::Mutex::new(write_conn)),
        );

        Self {
            profile,
            read_conn,
            write_conn,
            // default our id to 0, it'll be set later
            entity_id: Arc::new(RwLock::new(0)),
            world: Arc::new(RwLock::new(World::default())),
            world_container: world_container
                .unwrap_or_else(|| Arc::new(RwLock::new(WeakWorldContainer::new()))),
            world_name: Arc::new(RwLock::new(None)),
            physics_state: Arc::new(Mutex::new(PhysicsState::default())),
            client_information: Arc::new(RwLock::new(ClientInformation::default())),
            dead: Arc::new(Mutex::new(false)),
            // The plugins can be modified by the user by replacing the plugins
            // field right after this. No Mutex so the user doesn't need to .lock().
            plugins: Arc::new(PluginStates::default()),
            players: Arc::new(RwLock::new(HashMap::new())),
            tasks: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Connect to a Minecraft server.
    ///
    /// To change the render distance and other settings, use
    /// [`Client::set_client_information`]. To watch for events like packets
    /// sent by the server, use the `rx` variable this function returns.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use azalea_client::{Client, Account};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let account = Account::offline("bot");
    ///     let (client, rx) = Client::join(&account, "localhost").await?;
    ///     client.chat("Hello, world!").await?;
    ///     client.disconnect().await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn join(
        account: &Account,
        address: impl TryInto<ServerAddress>,
    ) -> Result<(Self, Receiver<Event>), JoinError> {
        let address: ServerAddress = address.try_into().map_err(|_| JoinError::InvalidAddress)?;
        let resolved_address = resolver::resolve_address(&address).await?;

        let conn = Connection::new(&resolved_address).await?;
        let (conn, game_profile) = Self::handshake(conn, account, &address).await?;

        // The buffer has to be 1 to avoid a bug where if it lags events are
        // received a bit later instead of the instant they were fired.
        // That bug especially causes issues with the pathfinder.
        let (tx, rx) = mpsc::channel(1);

        // we got the GameConnection, so the server is now connected :)
        let client = Client::new(game_profile, conn, None);

        tx.send(Event::Init).await.expect("Failed to send event");

        // just start up the game loop and we're ready!

        client.start_tasks(tx);

        Ok((client, rx))
    }

    /// Do a handshake with the server and get to the game state from the initial handshake state.
    pub async fn handshake(
        mut conn: Connection<ClientboundHandshakePacket, ServerboundHandshakePacket>,
        account: &Account,
        address: &ServerAddress,
    ) -> Result<
        (
            Connection<ClientboundGamePacket, ServerboundGamePacket>,
            GameProfile,
        ),
        JoinError,
    > {
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

        let (conn, profile) = loop {
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

        Ok((conn, profile))
    }

    /// Write a packet directly to the server.
    pub async fn write_packet(&self, packet: ServerboundGamePacket) -> Result<(), std::io::Error> {
        self.write_conn.lock().await.write(packet).await?;
        Ok(())
    }

    /// Disconnect this client from the server, ending all tasks.
    pub async fn disconnect(&self) -> Result<(), std::io::Error> {
        if let Err(e) = self.write_conn.lock().await.shutdown().await {
            warn!(
                "Error shutting down connection, but it might be fine: {}",
                e
            );
        }
        let tasks = self.tasks.lock();
        for task in tasks.iter() {
            task.abort();
        }
        Ok(())
    }

    /// Start the protocol and game tick loop.
    #[doc(hidden)]
    pub fn start_tasks(&self, tx: Sender<Event>) {
        // if you get an error right here that means you're doing something with locks wrong
        // read the error to see where the issue is
        // you might be able to just drop the lock or put it in its own scope to fix

        let mut tasks = self.tasks.lock();
        tasks.push(tokio::spawn(Client::protocol_loop(
            self.clone(),
            tx.clone(),
        )));
        tasks.push(tokio::spawn(Client::game_tick_loop(self.clone(), tx)));
    }

    async fn protocol_loop(client: Client, tx: Sender<Event>) {
        loop {
            let r = client.read_conn.lock().await.read().await;
            match r {
                Ok(packet) => match Self::handle(&packet, &client, &tx).await {
                    Ok(_) => {}
                    Err(e) => {
                        error!("Error handling packet: {}", e);
                        if !IGNORE_ERRORS {
                            panic!("Error handling packet: {e}");
                        }
                    }
                },
                Err(e) => {
                    if let ReadPacketError::ConnectionClosed = e {
                        info!("Connection closed");
                        if let Err(e) = client.disconnect().await {
                            error!("Error shutting down connection: {:?}", e);
                        }
                        break;
                    }
                    if IGNORE_ERRORS {
                        warn!("{}", e);
                        if let ReadPacketError::FrameSplitter { .. } = e {
                            panic!("Error: {e:?}");
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
        tx: &Sender<Event>,
    ) -> Result<(), HandleError> {
        let packet = Arc::new(packet.clone());
        tx.send(Event::Packet(packet.clone())).await?;
        match &*packet {
            ClientboundGamePacket::Login(p) => {
                debug!("Got login packet");

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

                    // add this world to the world_container (or don't if it's already there)
                    let weak_world =
                        client
                            .world_container
                            .write()
                            .insert(p.dimension.clone(), height, min_y);
                    // set the loaded_world to an empty world
                    // (when we add chunks or entities those will be in the world_container)
                    let mut world_lock = client.world.write();
                    *world_lock = World::new(
                        client.client_information.read().view_distance.into(),
                        weak_world,
                        p.player_id,
                    );

                    let entity = EntityData::new(
                        client.profile.uuid,
                        Vec3::default(),
                        EntityMetadata::Player(metadata::Player::default()),
                    );
                    // make it so other entities don't update this entity in a shared world
                    world_lock.add_entity(p.player_id, entity);

                    *client.entity_id.write() = p.player_id;
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

                tx.send(Event::Login).await?;
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
                client.disconnect().await?;
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
                    let player_entity_id = *client.entity_id.read();

                    let mut world_lock = client.world.write();

                    let mut player_entity = world_lock.entity_mut(player_entity_id).unwrap();

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
                use azalea_protocol::packets::game::clientbound_player_info_packet::Action;

                debug!("Got player info packet {:?}", p);
                let mut events = Vec::new();
                {
                    let mut players_lock = client.players.write();
                    match &p.action {
                        Action::AddPlayer(players) => {
                            for player in players {
                                let player_info = PlayerInfo {
                                    profile: GameProfile {
                                        uuid: player.uuid,
                                        name: player.name.clone(),
                                        properties: player.properties.clone(),
                                    },
                                    uuid: player.uuid,
                                    gamemode: player.gamemode,
                                    latency: player.latency,
                                    display_name: player.display_name.clone(),
                                };
                                players_lock.insert(player.uuid, player_info.clone());
                                events.push(Event::UpdatePlayers(UpdatePlayersEvent::Add(
                                    player_info,
                                )));
                            }
                        }
                        Action::UpdateGameMode(players) => {
                            for player in players {
                                if let Some(p) = players_lock.get_mut(&player.uuid) {
                                    p.gamemode = player.gamemode;
                                    events.push(Event::UpdatePlayers(
                                        UpdatePlayersEvent::GameMode {
                                            uuid: player.uuid,
                                            game_mode: player.gamemode,
                                        },
                                    ));
                                } else {
                                    warn!(
                                    "Ignoring PlayerInfo (UpdateGameMode) for unknown player {}",
                                    player.uuid
                                );
                                }
                            }
                        }
                        Action::UpdateLatency(players) => {
                            for player in players {
                                if let Some(p) = players_lock.get_mut(&player.uuid) {
                                    p.latency = player.latency;
                                    events.push(Event::UpdatePlayers(
                                        UpdatePlayersEvent::Latency {
                                            uuid: player.uuid,
                                            latency: player.latency,
                                        },
                                    ));
                                } else {
                                    warn!(
                                        "Ignoring PlayerInfo (UpdateLatency) for unknown player {}",
                                        player.uuid
                                    );
                                }
                            }
                        }
                        Action::UpdateDisplayName(players) => {
                            for player in players {
                                if let Some(p) = players_lock.get_mut(&player.uuid) {
                                    p.display_name = player.display_name.clone();
                                    events.push(Event::UpdatePlayers(
                                        UpdatePlayersEvent::DisplayName {
                                            uuid: player.uuid,
                                            display_name: player.display_name.clone(),
                                        },
                                    ));
                                } else {
                                    warn!(
                                    "Ignoring PlayerInfo (UpdateDisplayName) for unknown player {}",
                                    player.uuid
                                );
                                }
                            }
                        }
                        Action::RemovePlayer(players) => {
                            for player in players {
                                if players_lock.remove(&player.uuid).is_some() {
                                    events.push(Event::UpdatePlayers(UpdatePlayersEvent::Remove {
                                        uuid: player.uuid,
                                    }));
                                } else {
                                    warn!(
                                        "Ignoring PlayerInfo (RemovePlayer) for unknown player {}",
                                        player.uuid
                                    );
                                }
                            }
                        }
                    }
                }
                for event in events {
                    tx.send(event).await?;
                }
            }
            ClientboundGamePacket::SetChunkCacheCenter(p) => {
                debug!("Got chunk cache center packet {:?}", p);
                client
                    .world
                    .write()
                    .update_view_center(&ChunkPos::new(p.x, p.z));
            }
            ClientboundGamePacket::LevelChunkWithLight(p) => {
                // debug!("Got chunk with light packet {} {}", p.x, p.z);
                let pos = ChunkPos::new(p.x, p.z);

                // OPTIMIZATION: if we already know about the chunk from the
                // shared world (and not ourselves), then we don't need to
                // parse it again. This is only used when we have a shared
                // world, since we check that the chunk isn't currently owned
                // by this client.
                let shared_has_chunk = client.world.read().get_chunk(&pos).is_some();
                let this_client_has_chunk = client
                    .world
                    .read()
                    .chunk_storage
                    .limited_get(&pos)
                    .is_some();
                if shared_has_chunk && !this_client_has_chunk {
                    trace!(
                        "Skipping parsing chunk {:?} because we already know about it",
                        pos
                    );
                    return Ok(());
                }

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
                    // warn!("Server sent an entity data packet for an entity id ({}) that we don't know about", p.id);
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
                if p.health == 0.0 {
                    // we can't define a variable here with client.dead.lock()
                    // because of https://github.com/rust-lang/rust/issues/57478
                    if !*client.dead.lock() {
                        *client.dead.lock() = true;
                        tx.send(Event::Death(None)).await?;
                    }
                }
            }
            ClientboundGamePacket::SetExperience(p) => {
                debug!("Got set experience packet {:?}", p);
            }
            ClientboundGamePacket::TeleportEntity(p) => {
                let mut world_lock = client.world.write();
                let _ = world_lock.set_entity_pos(
                    p.id,
                    Vec3 {
                        x: p.x,
                        y: p.y,
                        z: p.z,
                    },
                );
            }
            ClientboundGamePacket::UpdateAdvancements(p) => {
                debug!("Got update advancements packet {:?}", p);
            }
            ClientboundGamePacket::RotateHead(_p) => {
                // debug!("Got rotate head packet {:?}", p);
            }
            ClientboundGamePacket::MoveEntityPos(p) => {
                let mut world_lock = client.world.write();

                let _ = world_lock.move_entity_with_delta(p.entity_id, &p.delta);
            }
            ClientboundGamePacket::MoveEntityPosRot(p) => {
                let mut world_lock = client.world.write();

                let _ = world_lock.move_entity_with_delta(p.entity_id, &p.delta);
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
                debug!("Got player chat packet {:?}", p);
                tx.send(Event::Chat(ChatPacket::Player(Arc::new(p.clone()))))
                    .await?;
            }
            ClientboundGamePacket::SystemChat(p) => {
                debug!("Got system chat packet {:?}", p);
                tx.send(Event::Chat(ChatPacket::System(Arc::new(p.clone()))))
                    .await?;
            }
            ClientboundGamePacket::Sound(_p) => {
                // debug!("Got sound packet {:?}", p);
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
            ClientboundGamePacket::PlayerCombatKill(p) => {
                debug!("Got player kill packet {:?}", p);
                if *client.entity_id.read() == p.player_id {
                    // we can't define a variable here with client.dead.lock()
                    // because of https://github.com/rust-lang/rust/issues/57478
                    if !*client.dead.lock() {
                        *client.dead.lock() = true;
                        tx.send(Event::Death(Some(Arc::new(p.clone())))).await?;
                    }
                }
            }
            ClientboundGamePacket::PlayerLookAt(_) => {}
            ClientboundGamePacket::RemoveMobEffect(_) => {}
            ClientboundGamePacket::ResourcePack(_) => {}
            ClientboundGamePacket::Respawn(p) => {
                debug!("Got respawn packet {:?}", p);
                // Sets clients dead state to false.
                let mut dead_lock = client.dead.lock();
                *dead_lock = false;
            }
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
    async fn game_tick_loop(mut client: Client, tx: Sender<Event>) {
        let mut game_tick_interval = time::interval(time::Duration::from_millis(50));
        // TODO: Minecraft bursts up to 10 ticks and then skips, we should too
        game_tick_interval.set_missed_tick_behavior(time::MissedTickBehavior::Burst);
        loop {
            game_tick_interval.tick().await;
            Self::game_tick(&mut client, &tx).await;
        }
    }

    /// Runs every 50 milliseconds.
    async fn game_tick(client: &mut Client, tx: &Sender<Event>) {
        // return if there's no chunk at the player's position

        {
            let world_lock = client.world.read();
            let player_entity_id = *client.entity_id.read();
            let player_entity = world_lock.entity(player_entity_id);
            let Some(player_entity) = player_entity else {
                return;
            };
            let player_chunk_pos: ChunkPos = player_entity.pos().into();
            if world_lock.get_chunk(&player_chunk_pos).is_none() {
                return;
            }
        }

        tx.send(Event::Tick)
            .await
            .expect("Sending tick event should never fail");

        // TODO: if we're a passenger, send the required packets

        if let Err(e) = client.send_position().await {
            warn!("Error sending position: {:?}", e);
        }
        client.ai_step();

        // TODO: minecraft does ambient sounds here
    }

    /// Get a [`WeakWorld`] from our world container. If it's a normal client,
    /// then it'll be the same as the world the client has loaded. If the
    /// client using a shared world, then the shared world will be a superset
    /// of the client's world.
    ///
    /// # Panics
    /// Panics if the client has not received the login packet yet. You can check this with [`Client::logged_in`].
    pub fn world(&self) -> Arc<WeakWorld> {
        let world_name = self.world_name.read();
        let world_name = world_name
            .as_ref()
            .expect("Client has not received login packet yet");
        if let Some(world) = self.world_container.read().get(world_name) {
            world
        } else {
            unreachable!("The world name must be in the world container");
        }
    }

    /// Returns the entity associated to the player.
    pub fn entity_mut(&self) -> Entity<RwLockWriteGuard<World>> {
        let entity_id = *self.entity_id.read();

        let world = self.world.write();

        let entity_data = world
            .entity_storage
            .get_by_id(entity_id)
            .expect("Player entity should exist");
        let entity_ptr = unsafe { entity_data.as_ptr() };
        Entity::new(world, entity_id, entity_ptr)
    }
    /// Returns the entity associated to the player.
    pub fn entity(&self) -> Entity<RwLockReadGuard<World>> {
        let entity_id = *self.entity_id.read();
        let world = self.world.read();

        let entity_data = world
            .entity_storage
            .get_by_id(entity_id)
            .expect("Player entity should be in the given world");
        let entity_ptr = unsafe { entity_data.as_ptr() };
        Entity::new(world, entity_id, entity_ptr)
    }

    /// Returns whether we have a received the login packet yet.
    pub fn logged_in(&self) -> bool {
        // the login packet tells us the world name
        self.world_name.read().is_some()
    }

    /// Tell the server we changed our game options (i.e. render distance, main hand).
    /// If this is not set before the login packet, the default will be sent.
    ///
    /// ```rust,no_run
    /// # use azalea_client::{Client, ClientInformation};
    /// # async fn example(bot: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// bot.set_client_information(ClientInformation {
    ///     view_distance: 2,
    ///     ..Default::default()
    /// })
    /// .await?;
    /// # Ok(())
    /// # }
    /// ```
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
