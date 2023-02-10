pub use crate::chat::ChatPacket;
use crate::{
    events::{Event, EventPlugin, LocalPlayerEvents},
    local_player::{
        death_event, update_in_loaded_chunk, GameProfileComponent, LocalPlayer, PhysicsState,
    },
    movement::{local_player_ai_step, send_position, sprint_listener, walk_listener},
    packet_handling::{self, PacketHandlerPlugin},
    player::retroactively_add_game_profile_component,
    task_pool::TaskPoolPlugin,
    Account, PlayerInfo, StartSprintEvent, StartWalkEvent,
};

use azalea_auth::{game_profile::GameProfile, sessionserver::ClientSessionServerError};
use azalea_chat::FormattedText;
use azalea_ecs::{
    app::{App, Plugin, PluginGroup, PluginGroupBuilder},
    component::Component,
    entity::Entity,
    schedule::{IntoSystemDescriptor, Schedule, Stage, SystemSet},
    AppTickExt,
};
use azalea_ecs::{ecs::Ecs, TickPlugin};
use azalea_physics::PhysicsPlugin;
use azalea_protocol::{
    connect::{Connection, ConnectionError},
    packets::{
        game::{
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
            serverbound_key_packet::{NonceOrSaltSignature, ServerboundKeyPacket},
            ClientboundLoginPacket,
        },
        ConnectionProtocol, PROTOCOL_VERSION,
    },
    resolver, ServerAddress,
};
use azalea_world::{
    entity::{EntityPlugin, Local, WorldName},
    PartialWorld, World, WorldContainer,
};
use log::{debug, error};
use parking_lot::{Mutex, RwLock};
use std::{collections::HashMap, fmt::Debug, io, net::SocketAddr, sync::Arc};
use thiserror::Error;
use tokio::{sync::mpsc, time};
use uuid::Uuid;

pub type ClientInformation = ServerboundClientInformationPacket;

/// `Client` has the things that a user interacting with the library will want.
/// Things that a player in the world will want to know are in [`LocalPlayer`].
///
/// To make a new client, use either [`azalea::ClientBuilder`] or
/// [`Client::join`].
///
/// [`azalea::ClientBuilder`]: https://docs.rs/azalea/latest/azalea/struct.ClientBuilder.html
#[derive(Clone)]
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

    /// The entity component system. You probably don't need to access this
    /// directly. Note that if you're using a shared world (i.e. a swarm), this
    /// will contain all entities in all worlds.
    pub ecs: Arc<Mutex<Ecs>>,
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
    SessionServer(#[from] azalea_auth::sessionserver::ClientSessionServerError),
    #[error("The given address could not be parsed into a ServerAddress")]
    InvalidAddress,
    #[error("Couldn't refresh access token: {0}")]
    Auth(#[from] azalea_auth::AuthError),
    #[error("Disconnected: {reason}")]
    Disconnect { reason: FormattedText },
}

impl Client {
    /// Create a new client from the given GameProfile, Connection, and World.
    /// You should only use this if you want to change these fields from the
    /// defaults, otherwise use [`Client::join`].
    pub fn new(profile: GameProfile, entity: Entity, ecs: Arc<Mutex<Ecs>>) -> Self {
        Self {
            profile,
            // default our id to 0, it'll be set later
            entity,
            world: Arc::new(RwLock::new(PartialWorld::default())),

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
    ///     client.chat("Hello, world!");
    ///     client.disconnect();
    ///     Ok(())
    /// }
    /// ```
    pub async fn join(
        account: &Account,
        address: impl TryInto<ServerAddress>,
    ) -> Result<(Self, mpsc::UnboundedReceiver<Event>), JoinError> {
        let address: ServerAddress = address.try_into().map_err(|_| JoinError::InvalidAddress)?;
        let resolved_address = resolver::resolve_address(&address).await?;

        // An event that causes the schedule to run. This is only used internally.
        let (run_schedule_sender, run_schedule_receiver) = mpsc::channel(1);
        let app = init_ecs_app();
        let ecs_lock = start_ecs(app, run_schedule_receiver, run_schedule_sender.clone());

        Self::start_client(
            ecs_lock,
            account,
            &address,
            &resolved_address,
            run_schedule_sender,
        )
        .await
    }

    /// Create a [`Client`] when you already have the ECS made with
    /// [`start_ecs`]. You'd usually want to use [`Self::join`] instead.
    pub async fn start_client(
        ecs_lock: Arc<Mutex<Ecs>>,
        account: &Account,
        address: &ServerAddress,
        resolved_address: &SocketAddr,
        run_schedule_sender: mpsc::Sender<()>,
    ) -> Result<(Self, mpsc::UnboundedReceiver<Event>), JoinError> {
        let conn = Connection::new(resolved_address).await?;
        let (conn, game_profile) = Self::handshake(conn, account, address).await?;
        let (read_conn, write_conn) = conn.into_split();

        let (tx, rx) = mpsc::unbounded_channel();

        let mut ecs = ecs_lock.lock();

        // Make the ecs entity for this client
        let entity_mut = ecs.spawn_empty();
        let entity = entity_mut.id();

        // we got the GameConnection, so the server is now connected :)
        let client = Client::new(game_profile.clone(), entity, ecs_lock.clone());

        let (packet_writer_sender, packet_writer_receiver) = mpsc::unbounded_channel();

        let mut local_player = crate::local_player::LocalPlayer::new(
            entity,
            packet_writer_sender,
            // default to an empty world, it'll be set correctly later when we
            // get the login packet
            Arc::new(RwLock::new(World::default())),
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

        ecs.entity_mut(entity).insert((
            local_player,
            packet_receiver,
            GameProfileComponent(game_profile),
            PhysicsState::default(),
            Local,
            LocalPlayerEvents(tx),
        ));

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
                                ClientSessionServerError::InvalidSession
                                    | ClientSessionServerError::ForbiddenOperation
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
                    return Err(JoinError::Disconnect { reason: p.reason });
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
            .write_packet(packet);
    }

    /// Disconnect this client from the server by ending all tasks.
    ///
    /// The OwnedReadHalf for the TCP connection is in one of the tasks, so it
    /// automatically closes the connection when that's dropped.
    pub fn disconnect(&self) {
        self.local_player_mut(&mut self.ecs.lock()).disconnect();
    }

    pub fn local_player<'a>(&'a self, ecs: &'a mut Ecs) -> &'a LocalPlayer {
        self.query::<&LocalPlayer>(ecs)
    }
    pub fn local_player_mut<'a>(
        &'a self,
        ecs: &'a mut Ecs,
    ) -> azalea_ecs::ecs::Mut<'a, LocalPlayer> {
        self.query::<&mut LocalPlayer>(ecs)
    }

    /// Get a component from this client. This will clone the component and
    /// return it.
    ///
    /// # Panics
    ///
    /// This will panic if the component doesn't exist on the client.
    ///
    /// # Examples
    ///
    /// ```
    /// # use azalea_world::entity::WorldName;
    /// # fn example(client: &azalea_client::Client) {
    /// let world_name = client.component::<WorldName>();
    /// # }
    pub fn component<T: Component + Clone>(&self) -> T {
        self.query::<&T>(&mut self.ecs.lock()).clone()
    }

    /// Get a reference to our (potentially shared) world.
    ///
    /// This gets the [`World`] from our world container. If it's a normal
    /// client, then it'll be the same as the world the client has loaded.
    /// If the client using a shared world, then the shared world will be a
    /// superset of the client's world.
    pub fn world(&self) -> Arc<RwLock<World>> {
        let world_name = self.component::<WorldName>();
        let ecs = self.ecs.lock();
        let world_container = ecs.resource::<WorldContainer>();
        world_container.get(&world_name).unwrap()
    }

    /// Returns whether we have a received the login packet yet.
    pub fn logged_in(&self) -> bool {
        // the login packet tells us the world name
        self.query::<Option<&WorldName>>(&mut self.ecs.lock())
            .is_some()
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

    /// Get a HashMap of all the players in the tab list.
    pub fn players(&mut self) -> HashMap<Uuid, PlayerInfo> {
        self.local_player(&mut self.ecs.lock()).players.clone()
    }
}

pub struct AzaleaPlugin;
impl Plugin for AzaleaPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StartWalkEvent>()
            .add_event::<StartSprintEvent>();

        app.add_plugins(DefaultPlugins);

        app.add_tick_system_set(
            SystemSet::new()
                .with_system(send_position)
                .with_system(update_in_loaded_chunk)
                .with_system(
                    local_player_ai_step
                        .before("ai_step")
                        .after("sprint_listener"),
                ),
        );

        // fire the Death event when the player dies.
        app.add_system(death_event.after("tick").after("packet"));

        // walk and sprint event listeners
        app.add_system(walk_listener.label("walk_listener").before("travel"))
            .add_system(
                sprint_listener
                    .label("sprint_listener")
                    .before("travel")
                    .before("walk_listener"),
            );

        // add GameProfileComponent when we get an AddPlayerEvent
        app.add_system(
            retroactively_add_game_profile_component
                .after("tick")
                .after("packet"),
        );

        app.init_resource::<WorldContainer>();
    }
}

/// Create the [`App`]. This won't actually run anything yet.
///
/// Note that you usually only need this if you're creating a client manually,
/// otherwise use [`Client::join`].
///
/// Use [`start_ecs`] to actually start running the app and then
/// [`Client::start_client`] to add a client to the ECS and make it join a
/// server.
#[doc(hidden)]
pub fn init_ecs_app() -> App {
    // if you get an error right here that means you're doing something with locks
    // wrong read the error to see where the issue is
    // you might be able to just drop the lock or put it in its own scope to fix

    let mut app = App::new();
    app.add_plugin(AzaleaPlugin);
    app
}

/// Start running the ECS loop! You must create your `App` from [`init_ecs_app`]
/// first.
#[doc(hidden)]
pub fn start_ecs(
    app: App,
    run_schedule_receiver: mpsc::Receiver<()>,
    run_schedule_sender: mpsc::Sender<()>,
) -> Arc<Mutex<Ecs>> {
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
    ecs: Arc<Mutex<Ecs>>,
    mut schedule: Schedule,
    mut run_schedule_receiver: mpsc::Receiver<()>,
) {
    loop {
        // whenever we get an event from run_schedule_receiver, run the schedule
        run_schedule_receiver.recv().await;
        schedule.run(&mut ecs.lock());
    }
}

/// Send an event to run the schedule every 50 milliseconds. It will stop when
/// the receiver is dropped.
pub async fn tick_run_schedule_loop(run_schedule_sender: mpsc::Sender<()>) {
    let mut game_tick_interval = time::interval(time::Duration::from_millis(50));
    // TODO: Minecraft bursts up to 10 ticks and then skips, we should too
    game_tick_interval.set_missed_tick_behavior(time::MissedTickBehavior::Burst);

    loop {
        game_tick_interval.tick().await;
        if let Err(e) = run_schedule_sender.send(()).await {
            println!("tick_run_schedule_loop error: {e}");
            // the sender is closed so end the task
            return;
        }
    }
}

/// This plugin group will add all the default plugins necessary for Azalea to
/// work.
pub struct DefaultPlugins;

impl PluginGroup for DefaultPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(TickPlugin::default())
            .add(PacketHandlerPlugin)
            .add(EntityPlugin)
            .add(PhysicsPlugin)
            .add(EventPlugin)
            .add(TaskPoolPlugin::default())
    }
}
