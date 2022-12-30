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
    PartialChunkStorage, PartialWorld, WeakWorld, WeakWorldContainer,
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
            world_name: Arc::new(RwLock::new(None)),
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
        let local_player =
            crate::local_player::LocalPlayer::new(game_profile, conn, client.world.clone(), tx);

        // just start up the game loop and we're ready!

        client.start_tasks();

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
        self.local_player_mut().write_packet(packet).await
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
    pub fn start_tasks(&self) {
        // if you get an error right here that means you're doing something with locks
        // wrong read the error to see where the issue is
        // you might be able to just drop the lock or put it in its own scope to fix

        let mut tasks = self.tasks.lock();
        tasks.push(tokio::spawn(Client::protocol_loop(
            self.clone(),
            tx.clone(),
        )));
        let ecs = self.world_container.clone().read().ecs;
        tasks.push(tokio::spawn(Client::game_tick_loop(ecs.clone())));
    }

    async fn protocol_loop(local_player: LocalPlayer, tx: Sender<Event>) {
        loop {
            let r = local_player.read_conn.lock().await.read().await;
            match r {
                Ok(packet) => {
                    match {
                        LocalPlayer::send_event(Event::Packet(packet.clone()), &tx);
                        LocalPlayer::handle_packet(&packet, &client, &tx)
                    } {
                    Ok(_) => {}
                    Err(e) => {
                        error!("Error handling packet: {}", e);
                        if !IGNORE_ERRORS {
                            panic!("Error handling packet: {e}");
                        }
                    }
                },
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
            };
        }
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
    pub fn world(&self) -> Arc<WeakWorld> {
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
