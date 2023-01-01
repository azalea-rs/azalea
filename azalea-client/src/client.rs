pub use crate::chat::ChatPacket;
use crate::{
    local_player::LocalPlayer, movement::WalkDirection, plugins::PluginStates, Account, PlayerInfo,
};
use azalea_auth::{game_profile::GameProfile, sessionserver::SessionServerError};
use azalea_core::{ChunkPos, ResourceLocation, Vec3};
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
            serverbound_key_packet::ServerboundKeyPacket, ClientboundLoginPacket,
        },
        ConnectionProtocol, PROTOCOL_VERSION,
    },
    read::ReadPacketError,
    resolver, ServerAddress,
};
use azalea_world::{
    entity::{
        self,
        metadata::{self, PlayerMetadataBundle},
        EntityId,
    },
    PartialChunkStorage, PartialWorld, WeakWorldContainer, World,
};
use bevy_ecs::{
    prelude::Component,
    query::{QueryState, WorldQuery},
    schedule::{Schedule, Stage, StageLabel, SystemStage},
    system::{Query, SystemState},
};
use log::{debug, error, info, trace, warn};
use parking_lot::{Mutex, RwLock};
use std::{
    any,
    backtrace::Backtrace,
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

/// Something that happened in-game, such as a tick passing or chat message
/// being sent.
///
/// Note: Events are sent before they're processed, so for example game ticks
/// happen at the beginning of a tick before anything has happened.
#[derive(Debug, Clone)]
pub enum Event {
    /// Happens right after the bot switches into the Game state, but before
    /// it's actually spawned. This can be useful for setting the client
    /// information with `Client::set_client_information`, so the packet
    /// doesn't have to be sent twice.
    Init,
    /// The client is now in the world. Fired when we receive a login packet.
    Login,
    /// A chat message was sent in the game chat.
    Chat(ChatPacket),
    /// Happens 20 times per second, but only when the world is loaded.
    Tick,
    Packet(Arc<ClientboundGamePacket>),
    /// A player joined the game (or more specifically, was added to the tab
    /// list).
    AddPlayer(PlayerInfo),
    /// A player left the game (or maybe is still in the game and was just
    /// removed from the tab list).
    RemovePlayer(PlayerInfo),
    /// A player was updated in the tab list (gamemode, display
    /// name, or latency changed).
    UpdatePlayer(PlayerInfo),
    /// The client player died in-game.
    Death(Option<Arc<ClientboundPlayerCombatKillPacket>>),
}

/// Client has the things that a user interacting with the library will want.
/// Things that a player in the world will want to know are in [`LocalPlayer`].
pub struct Client {
    pub profile: GameProfile,
    pub entity_id: Arc<RwLock<EntityId>>,
    /// A container of world names to worlds. If we're not using a shared world
    /// (i.e. not a swarm), then this will only contain data about the world
    /// we're currently in.
    world_container: Arc<RwLock<WeakWorldContainer>>,
    /// The world that this client is in.
    pub world: Arc<RwLock<PartialWorld>>,

    /// Plugins are a way for other crates to add custom functionality to the
    /// client and keep state. If you're not making a plugin and you're using
    /// the `azalea` crate. you can ignore this field.
    pub plugins: Arc<PluginStates>,
    tasks: Arc<Mutex<Vec<JoinHandle<()>>>>,
}

/// Whether we should ignore errors when decoding packets.
const IGNORE_ERRORS: bool = !cfg!(debug_assertions);

/// An error that happened while joining the server.
#[derive(Error, Debug)]
pub enum JoinError {
    #[error("{0}")]
    Resolver(#[from] resolver::ResolverError),
    #[error("{0}")]
    Connection(#[from] ConnectionError),
    #[error("{0}")]
    ReadPacket(#[from] Box<azalea_protocol::read::ReadPacketError>),
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    SessionServer(#[from] azalea_auth::sessionserver::SessionServerError),
    #[error("The given address could not be parsed into a ServerAddress")]
    InvalidAddress,
    #[error("Couldn't refresh access token: {0}")]
    Auth(#[from] azalea_auth::AuthError),
}

impl Client {
    /// Create a new client from the given GameProfile, Connection, and World.
    /// You should only use this if you want to change these fields from the
    /// defaults, otherwise use [`Client::join`].
    pub fn new(
        profile: GameProfile,
        world_container: Option<Arc<RwLock<WeakWorldContainer>>>,
    ) -> Self {
        Self {
            profile,
            // default our id to 0, it'll be set later
            entity_id: Arc::new(RwLock::new(EntityId(0))),
            world: Arc::new(RwLock::new(PartialWorld::default())),
            world_container: world_container
                .unwrap_or_else(|| Arc::new(RwLock::new(WeakWorldContainer::new()))),
            // The plugins can be modified by the user by replacing the plugins
            // field right after this. No Mutex so the user doesn't need to .lock().
            plugins: Arc::new(PluginStates::default()),
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
        tx.send(Event::Init).await.expect("Failed to send event");

        // we got the GameConnection, so the server is now connected :)
        let client = Client::new(game_profile.clone(), None);

        let (read_conn, write_conn) = conn.into_split();

        let local_player = crate::local_player::LocalPlayer::new(
            game_profile,
            write_conn,
            client.world.clone(),
            tx,
        );

        // just start up the game loop and we're ready!

        client.start_tasks(read_conn);

        Ok((client, rx))
    }

    /// Do a handshake with the server and get to the game state from the
    /// initial handshake state.
    ///
    /// This will also automatically refresh the account's access token if
    /// it's expired.
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
                name: account.username.clone(),
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
                        // keep track of the number of times we tried
                        // authenticating so we can give up after too many
                        let mut attempts: usize = 1;

                        while let Err(e) = {
                            let access_token = access_token.lock().clone();
                            conn.authenticate(
                                &access_token,
                                &account
                                    .uuid
                                    .expect("Uuid must be present if access token is present."),
                                e.secret_key,
                                &p,
                            )
                            .await
                        } {
                            if attempts >= 2 {
                                // if this is the second attempt and we failed
                                // both times, give up
                                return Err(e.into());
                            }
                            if let SessionServerError::InvalidSession = e {
                                // uh oh, we got an invalid session and have
                                // to reauthenticate now
                                account.refresh().await?;
                            } else {
                                return Err(e.into());
                            }
                            attempts += 1;
                        }
                    }

                    conn.write(
                        ServerboundKeyPacket {
                            key_bytes: e.encrypted_public_key,
                            encrypted_challenge: e.encrypted_nonce,
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
        self.local_player_mut().write_packet_async(packet).await
    }

    /// Disconnect this client from the server, ending all tasks.
    pub async fn disconnect(&self) -> Result<(), std::io::Error> {
        if let Err(e) = self.local_player_mut().write_conn.shutdown().await {
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

    pub fn local_player(&self) -> &LocalPlayer {
        self.query::<&LocalPlayer>()
    }
    pub fn local_player_mut(&self) -> &mut LocalPlayer {
        self.query::<&mut LocalPlayer>().into_inner()
    }

    /// Start the protocol and game tick loop.
    #[doc(hidden)]
    pub fn start_tasks(&self, read_conn: ReadConnection<ClientboundGamePacket>) {
        // if you get an error right here that means you're doing something with locks
        // wrong read the error to see where the issue is
        // you might be able to just drop the lock or put it in its own scope to fix

        let mut tasks = self.tasks.lock();
        tasks.push(tokio::spawn(self.protocol_loop(tx.clone(), read_conn)));
        let ecs = self.world_container.clone().read().ecs;
        tasks.push(tokio::spawn(Client::game_tick_loop(ecs.clone())));
    }

    async fn protocol_loop(
        &self,
        tx: Sender<Event>,
        read_conn: ReadConnection<ClientboundGamePacket>,
    ) {
        loop {
            let r = read_conn.lock().await.read().await;
            match r {
                Ok(packet) => LocalPlayer::send_event(Event::Packet(packet.clone()), &tx),
                Err(e) => {
                    let e = *e;
                    if let ReadPacketError::ConnectionClosed = e {
                        info!("Connection closed");
                        if let Err(e) = client.disconnect().await {
                            error!("Error shutting down connection: {:?}", e);
                        }
                        break;
                    }
                    let default_backtrace = Backtrace::capture();
                    if IGNORE_ERRORS {
                        let backtrace =
                            any::request_ref::<Backtrace>(&e).unwrap_or(&default_backtrace);
                        warn!("{e}\n{backtrace}");
                        match e {
                            ReadPacketError::FrameSplitter { .. } => panic!("Error: {e:?}"),
                            _ => continue,
                        }
                    } else {
                        let backtrace =
                            any::request_ref::<Backtrace>(&e).unwrap_or(&default_backtrace);
                        panic!("{e}\n{backtrace}")
                    }
                }
            }
        }
    }

    // handling packets is outside of the ecs schedule since it's not bound to
    // ticks.
    pub fn handle_packet(
        client: &Client,
        packet: ClientboundGamePacket,
        ecs: &mut bevy_ecs::world::World,
        tx: &mpsc::Sender<Event>,
    ) -> Result<(), HandlePacketError> {
        let (mut local_player,) = query.get_mut((*player_entity_id).into()).unwrap();

        match &packet {
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

                    let world_name = p.dimension.clone();

                    local_player.world_name = Some(world_name.clone());
                    // add this world to the world_container (or don't if it's already there)
                    let weak_world = world_container.insert(world_name, height, min_y);
                    // set the loaded_world to an empty world
                    // (when we add chunks or entities those will be in the world_container)
                    let mut world_lock = local_player.world.write();
                    *world_lock = PartialWorld::new(
                        local_player.client_information.view_distance.into(),
                        weak_world,
                        Some(EntityId(p.player_id)),
                    );

                    let player_bundle = entity::PlayerBundle {
                        entity: entity::EntityBundle::new(
                            local_player.profile.uuid,
                            Vec3::default(),
                            azalea_registry::EntityKind::Player,
                        ),
                        metadata: PlayerMetadataBundle::default(),
                    };
                    // let entity = EntityData::new(
                    //     client.profile.uuid,
                    //     Vec3::default(),
                    //     EntityMetadata::Player(metadata::Player::default()),
                    // );
                    // the first argument makes it so other entities don't update this entity in
                    // a shared world
                    world_lock.add_entity(EntityId(p.player_id), player_bundle);

                    *client.entity_id.write() = EntityId(p.player_id);
                }

                // send the client information that we have set
                let client_information_packet: ClientInformation =
                    client.local_player().client_information.clone();
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
                    let player_entity_id = *client.entity();
                    let world = client.world();
                    // let mut player_entity = world.entity_mut(player_entity_id).unwrap();
                    let (mut physics, position) =
                        client.query::<(&mut entity::Physics, &mut entity::Position)>();

                    let delta_movement = physics.delta;

                    let is_x_relative = p.relative_arguments.x;
                    let is_y_relative = p.relative_arguments.y;
                    let is_z_relative = p.relative_arguments.z;

                    let (delta_x, new_pos_x) = if is_x_relative {
                        physics.last_pos.x += p.x;
                        (delta_movement.x, position.x + p.x)
                    } else {
                        physics.last_pos.x = p.x;
                        (0.0, p.x)
                    };
                    let (delta_y, new_pos_y) = if is_y_relative {
                        physics.last_pos.y += p.y;
                        (delta_movement.y, position.y + p.y)
                    } else {
                        physics.last_pos.y = p.y;
                        (0.0, p.y)
                    };
                    let (delta_z, new_pos_z) = if is_z_relative {
                        physics.last_pos.z += p.z;
                        (delta_movement.z, position.z + p.z)
                    } else {
                        physics.last_pos.z = p.z;
                        (0.0, p.z)
                    };

                    let mut y_rot = p.y_rot;
                    let mut x_rot = p.x_rot;
                    if p.relative_arguments.x_rot {
                        x_rot += physics.x_rot;
                    }
                    if p.relative_arguments.y_rot {
                        y_rot += physics.y_rot;
                    }

                    physics.delta = Vec3 {
                        x: delta_x,
                        y: delta_y,
                        z: delta_z,
                    };
                    entity::set_rotation(physics.into_inner(), y_rot, x_rot);
                    // TODO: minecraft sets "xo", "yo", and "zo" here but idk what that means
                    // so investigate that ig
                    let new_pos = Vec3 {
                        x: new_pos_x,
                        y: new_pos_y,
                        z: new_pos_z,
                    };
                    world
                        .set_entity_pos(
                            player_entity_id,
                            new_pos,
                            position.into_inner(),
                            physics.into_inner(),
                        )
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
            ClientboundGamePacket::PlayerInfoUpdate(p) => {
                debug!("Got player info packet {:?}", p);
                let mut events = Vec::new();
                {
                    let mut players_lock = client.players.write();
                    for updated_info in &p.entries {
                        // add the new player maybe
                        if p.actions.add_player {
                            let player_info = PlayerInfo {
                                profile: updated_info.profile.clone(),
                                uuid: updated_info.profile.uuid,
                                gamemode: updated_info.game_mode,
                                latency: updated_info.latency,
                                display_name: updated_info.display_name.clone(),
                            };
                            players_lock.insert(updated_info.profile.uuid, player_info.clone());
                            events.push(Event::AddPlayer(player_info));
                        } else if let Some(info) = players_lock.get_mut(&updated_info.profile.uuid)
                        {
                            // `else if` because the block for add_player above
                            // already sets all the fields
                            if p.actions.update_game_mode {
                                info.gamemode = updated_info.game_mode;
                            }
                            if p.actions.update_latency {
                                info.latency = updated_info.latency;
                            }
                            if p.actions.update_display_name {
                                info.display_name = updated_info.display_name.clone();
                            }
                            events.push(Event::UpdatePlayer(info.clone()));
                        } else {
                            warn!(
                                "Ignoring PlayerInfoUpdate for unknown player {}",
                                updated_info.profile.uuid
                            );
                        }
                    }
                }
                for event in events {
                    tx.send(event).await?;
                }
            }
            ClientboundGamePacket::PlayerInfoRemove(p) => {
                let mut events = Vec::new();
                {
                    let mut players_lock = client.players.write();
                    for uuid in &p.profile_ids {
                        if let Some(info) = players_lock.remove(uuid) {
                            events.push(Event::RemovePlayer(info));
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
                let this_client_has_chunk = client.world.read().chunks.limited_get(&pos).is_some();
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
                let bundle = p.as_entity_bundle();
                let mut world = client.world.write();
                world.add_entity(EntityId(p.id), bundle);
                // the bundle doesn't include the default entity metadata so we add that
                // separately
                let mut entities = world.entity_infos.shared.write();
                let mut entity = entities.ecs_entity_mut(EntityId(p.id)).unwrap();
                p.apply_metadata(&mut entity);
            }
            ClientboundGamePacket::SetEntityData(p) => {
                debug!("Got set entity data packet {:?}", p);
                let world = client.world.write();
                let mut entities = world.entity_infos.shared.write();
                let entity = entities.ecs_entity_mut(EntityId(p.id));
                if let Some(mut entity) = entity {
                    entity::metadata::apply_metadata(&mut entity, p.packed_items.0.clone());
                } else {
                    // warn!("Server sent an entity data packet for an
                    // entity id ({}) that we don't
                    // know about", p.id);
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
                let bundle = p.as_player_bundle();
                let mut world = client.world.write();
                world.add_entity(EntityId(p.id), bundle);
                // the default metadata was already included in the bundle
                // for us
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
                let mut world = client.world.write();
                let (pos, physics) = self.query::<(&entity::Position, &entity::Physics)>();
                let _ = world.set_entity_pos(
                    EntityId(p.id),
                    Vec3 {
                        x: p.x,
                        y: p.y,
                        z: p.z,
                    },
                    pos,
                    physics,
                );
            }
            ClientboundGamePacket::UpdateAdvancements(p) => {
                debug!("Got update advancements packet {:?}", p);
            }
            ClientboundGamePacket::RotateHead(_p) => {
                // debug!("Got rotate head packet {:?}", p);
            }
            ClientboundGamePacket::MoveEntityPos(p) => {
                let mut world = client.world.write();
                let _ = world.move_entity_with_delta(EntityId(p.entity_id), &p.delta);
            }
            ClientboundGamePacket::MoveEntityPosRot(p) => {
                let mut world = client.world.write();
                let _ = world.move_entity_with_delta(EntityId(p.entity_id), &p.delta);
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
            ClientboundGamePacket::CommandSuggestions(_) => {}
            ClientboundGamePacket::ContainerSetData(_) => {}
            ClientboundGamePacket::ContainerSetSlot(_) => {}
            ClientboundGamePacket::Cooldown(_) => {}
            ClientboundGamePacket::CustomChatCompletions(_) => {}
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
            ClientboundGamePacket::PlayerCombatEnd(_) => {}
            ClientboundGamePacket::PlayerCombatEnter(_) => {}
            ClientboundGamePacket::PlayerCombatKill(p) => {
                debug!("Got player kill packet {:?}", p);
                if client.entity() == EntityId(p.player_id) {
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
            ClientboundGamePacket::DisguisedChat(_) => {}
            ClientboundGamePacket::UpdateEnabledFeatures(_) => {}
            ClientboundGamePacket::ContainerClose(_) => {}
        }
        Ok(())
    }

    /// Start the game tick loop for every client in the shared world. This
    /// should only be run once per shared world!
    async fn game_tick_loop(ecs: Arc<Mutex<bevy_ecs::world::World>>) {
        let mut game_tick_interval = time::interval(time::Duration::from_millis(50));
        // TODO: Minecraft bursts up to 10 ticks and then skips, we should too
        game_tick_interval.set_missed_tick_behavior(time::MissedTickBehavior::Burst);

        let mut schedule = Schedule::default();
        #[derive(StageLabel)]
        pub struct Tick;
        schedule.add_stage(
            Tick,
            SystemStage::single_threaded()
                .with_system(LocalPlayer::update_in_loaded_chunk)
                .with_system(LocalPlayer::send_position)
                .with_system(LocalPlayer::ai_step),
        );

        loop {
            game_tick_interval.tick().await;

            schedule.run(&mut ecs.lock());
        }
    }

    /// Get a reference to our (potentially shared) world.
    ///
    /// This gets the [`WeakWorld`] from our world container. If it's a normal
    /// client, then it'll be the same as the world the client has loaded.
    /// If the client using a shared world, then the shared world will be a
    /// superset of the client's world.
    pub fn world(&self) -> Arc<World> {
        self.world.read().shared.clone()
    }

    pub fn entity(&self) -> EntityId {
        *self.entity_id.read()
    }

    /// Query data of our player's entity.
    pub fn query<'w, 's, Q: WorldQuery>(&'w self) -> <Q as WorldQuery>::Item<'_> {
        let mut ecs = &mut self.world_container.write().ecs.lock();
        QueryState::<Q>::new(ecs)
            .get_mut(ecs, self.entity().into())
            .expect("Player entity should always exist when being queried")
    }

    /// Returns whether we have a received the login packet yet.
    pub fn logged_in(&self) -> bool {
        // the login packet tells us the world name
        self.world_name.read().is_some()
    }

    /// Tell the server we changed our game options (i.e. render distance, main
    /// hand). If this is not set before the login packet, the default will
    /// be sent.
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
            self.local_player_mut().client_information = client_information;
        }

        if self.logged_in() {
            let client_information_packet = self.local_player().client_information.clone().get();
            log::debug!(
                "Sending client information (already logged in): {:?}",
                client_information_packet
            );
            self.write_packet(client_information_packet).await?;
        }

        Ok(())
    }
}
