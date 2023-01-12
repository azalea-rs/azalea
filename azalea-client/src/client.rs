pub use crate::chat::ChatPacket;
use crate::{
    local_player::{
        death_event, send_tick_event, update_in_loaded_chunk, LocalPlayer, PhysicsState,
    },
    movement::{local_player_ai_step, send_position},
    packet_handling::{self, PacketHandlerPlugin},
    plugins::PluginStates,
    Account, PlayerInfo,
};

use azalea_auth::{game_profile::GameProfile, sessionserver::SessionServerError};
use azalea_protocol::{
    connect::{Connection, ConnectionError},
    packets::{
        game::{
            clientbound_player_combat_kill_packet::ClientboundPlayerCombatKillPacket,
            serverbound_client_information_packet::ServerboundClientInformationPacket,
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
    resolver, ServerAddress,
};
use azalea_world::{
    entity::Entity, EntityInfos, EntityPlugin, PartialWorld, World, WorldContainer,
};
use bevy_app::App;
use bevy_ecs::{
    query::WorldQuery,
    schedule::{IntoSystemDescriptor, Schedule, Stage, SystemSet},
};
use iyes_loopless::prelude::*;
use log::{debug, error};
use parking_lot::{Mutex, RwLock};
use std::{fmt::Debug, io, ops::DerefMut, sync::Arc, time::Duration};
use thiserror::Error;
use tokio::{sync::mpsc, time};

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
    /// The [`GameProfile`] for our client. This contains your username, UUID,
    /// and skin data.
    ///
    /// This is immutable; the server cannot change it. To get the username and
    /// skin the server chose for you, get your player from
    /// [`Self::players`].
    pub profile: GameProfile,
    /// The entity for this client in the ECS.
    pub entity: Entity,
    /// The world that this client is in.
    pub world: Arc<RwLock<PartialWorld>>,

    /// Plugins are a way for other crates to add custom functionality to the
    /// client and keep state. If you're not making a plugin and you're using
    /// the `azalea` crate. you can ignore this field.
    pub plugins: Arc<PluginStates>,

    /// The entity component system. You probably don't need to access this
    /// directly. Note that if you're using a shared world (i.e. a swarm), this
    /// will contain all entities in all worlds.
    pub ecs: Arc<Mutex<bevy_ecs::world::World>>,
}

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
        entity: Entity,
        ecs: Arc<Mutex<bevy_ecs::world::World>>,
    ) -> Self {
        Self {
            profile,
            // default our id to 0, it'll be set later
            entity,
            world: Arc::new(RwLock::new(PartialWorld::default())),
            // The plugins can be modified by the user by replacing the plugins
            // field right after this. No Mutex so the user doesn't need to .lock().
            plugins: Arc::new(PluginStates::default()),

            ecs,
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
    ) -> Result<(Self, mpsc::UnboundedReceiver<Event>), JoinError> {
        let address: ServerAddress = address.try_into().map_err(|_| JoinError::InvalidAddress)?;
        let resolved_address = resolver::resolve_address(&address).await?;

        let conn = Connection::new(&resolved_address).await?;
        let (conn, game_profile) = Self::handshake(conn, account, &address).await?;
        let (read_conn, write_conn) = conn.into_split();

        // The buffer has to be 1 to avoid a bug where if it lags events are
        // received a bit later instead of the instant they were fired.
        // That bug especially causes issues with the pathfinder.
        let (tx, rx) = mpsc::unbounded_channel();
        tx.send(Event::Init).unwrap();

        // An event that causes the schedule to run. This is only used internally.
        let (run_schedule_sender, run_schedule_receiver) = mpsc::unbounded_channel();

        let ecs_lock = start_ecs(run_schedule_receiver, run_schedule_sender.clone()).await;

        let mut ecs = ecs_lock.lock();
        ecs.init_resource::<EntityInfos>();

        let entity_mut = ecs.spawn_empty();
        let entity = entity_mut.id();

        // we got the GameConnection, so the server is now connected :)
        let client = Client::new(game_profile.clone(), entity, ecs_lock.clone());

        let (packet_writer_sender, packet_writer_receiver) = mpsc::unbounded_channel();

        let mut local_player = crate::local_player::LocalPlayer::new(
            entity,
            game_profile,
            packet_writer_sender,
            // default to an empty world, it'll be set correctly later when we
            // get the login packet
            Arc::new(RwLock::new(World::default())),
            ecs.resource_mut::<EntityInfos>().deref_mut(),
            tx,
        );

        // start receiving packets
        let packet_receiver = packet_handling::PacketReceiver {
            packets: Arc::new(Mutex::new(Vec::new())),
            run_schedule_sender: run_schedule_sender.clone(),
        };

        let read_packets_task = tokio::spawn(packet_receiver.clone().read_task(read_conn));
        let write_packets_task = tokio::spawn(
            packet_receiver
                .clone()
                .write_task(write_conn, packet_writer_receiver),
        );
        local_player.tasks.push(read_packets_task);
        local_player.tasks.push(write_packets_task);

        ecs.entity_mut(entity)
            .insert((local_player, packet_receiver, PhysicsState::default()));

        // just start up the game loop and we're ready!

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
                            if matches!(
                                e,
                                SessionServerError::InvalidSession
                                    | SessionServerError::ForbiddenOperation
                            ) {
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
    pub fn write_packet(&self, packet: ServerboundGamePacket) {
        self.local_player_mut(&mut self.ecs.lock())
            .write_packet(packet)
    }

    /// Disconnect this client from the server by ending all tasks.
    ///
    /// The OwnedReadHalf for the TCP connection is in one of the tasks, so it
    /// automatically closes the connection when that's dropped.
    pub fn disconnect(&self) {
        self.local_player_mut(&mut self.ecs.lock()).disconnect();
    }

    pub fn local_player<'a>(&'a self, ecs: &'a mut bevy_ecs::world::World) -> &'a LocalPlayer {
        self.query::<&LocalPlayer>(ecs)
    }
    pub fn local_player_mut<'a>(
        &'a self,
        ecs: &'a mut bevy_ecs::world::World,
    ) -> bevy_ecs::world::Mut<'a, LocalPlayer> {
        self.query::<&mut LocalPlayer>(ecs)
    }

    /// Get a reference to our (potentially shared) world.
    ///
    /// This gets the [`WeakWorld`] from our world container. If it's a normal
    /// client, then it'll be the same as the world the client has loaded.
    /// If the client using a shared world, then the shared world will be a
    /// superset of the client's world.
    pub fn world(&self) -> Arc<RwLock<World>> {
        let mut ecs = self.ecs.lock();

        let world_name = {
            let local_player = self.local_player(&mut ecs);
            local_player
                .world_name
                .as_ref()
                .expect("World name must be known if we're doing Client::world")
                .clone()
        };

        let world_container = ecs.resource::<WorldContainer>();
        world_container.get(&world_name).unwrap()
    }

    /// Returns whether we have a received the login packet yet.
    pub fn logged_in(&self) -> bool {
        // the login packet tells us the world name
        self.local_player(&mut self.ecs.lock()).world_name.is_some()
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
            self.local_player_mut(&mut self.ecs.lock())
                .client_information = client_information;
        }

        if self.logged_in() {
            let client_information_packet = self
                .local_player(&mut self.ecs.lock())
                .client_information
                .clone()
                .get();
            log::debug!(
                "Sending client information (already logged in): {:?}",
                client_information_packet
            );
            self.write_packet(client_information_packet);
        }

        Ok(())
    }

    /// Query data of our player's entity.
    pub fn query<'w, Q: WorldQuery>(
        &self,
        ecs: &'w mut bevy_ecs::world::World,
    ) -> <Q as WorldQuery>::Item<'w> {
        ecs.query::<Q>()
            .get_mut(ecs, self.entity)
            .expect("Player entity should always exist when being queried")
    }
}

/// Start the protocol and game tick loop.
#[doc(hidden)]
pub async fn start_ecs(
    run_schedule_receiver: mpsc::UnboundedReceiver<()>,
    run_schedule_sender: mpsc::UnboundedSender<()>,
) -> Arc<Mutex<bevy_ecs::world::World>> {
    // if you get an error right here that means you're doing something with locks
    // wrong read the error to see where the issue is
    // you might be able to just drop the lock or put it in its own scope to fix

    let mut app = App::new();
    app.add_fixed_timestep(Duration::from_millis(50), "tick")
        .add_fixed_timestep_system_set(
            "tick",
            0,
            SystemSet::new()
                .with_system(send_position)
                .with_system(update_in_loaded_chunk)
                .with_system(local_player_ai_step)
                .with_system(send_tick_event),
        );

    // fire the Death event when the player dies.
    app.add_system(death_event.after("tick").after("packet"));
    app.add_plugin(PacketHandlerPlugin);
    app.add_plugin(EntityPlugin);

    app.init_resource::<WorldContainer>();

    // all resources should have been added by now so we can take the ecs from the
    // app
    let ecs = Arc::new(Mutex::new(app.world));

    tokio::spawn(run_schedule_loop(
        ecs.clone(),
        app.schedule,
        run_schedule_receiver,
    ));
    tokio::spawn(tick_run_schedule_loop(run_schedule_sender));

    ecs
}

async fn run_schedule_loop(
    ecs: Arc<Mutex<bevy_ecs::world::World>>,
    mut schedule: Schedule,
    mut run_schedule_receiver: mpsc::UnboundedReceiver<()>,
) {
    loop {
        // whenever we get an event from run_schedule_receiver, run the schedule
        run_schedule_receiver.recv().await;
        schedule.run(&mut ecs.lock());
    }
}

/// Send an event to run the schedule every 50 milliseconds. It will stop when
/// the receiver is dropped.
pub async fn tick_run_schedule_loop(run_schedule_sender: mpsc::UnboundedSender<()>) {
    let mut game_tick_interval = time::interval(time::Duration::from_millis(50));
    // TODO: Minecraft bursts up to 10 ticks and then skips, we should too
    game_tick_interval.set_missed_tick_behavior(time::MissedTickBehavior::Burst);

    println!("tick_run_schedule_loop started");

    loop {
        game_tick_interval.tick().await;
        if let Err(e) = run_schedule_sender.send(()) {
            println!("tick_run_schedule_loop error: {}", e);
            // the sender is closed so end the task
            return;
        }
    }
}
