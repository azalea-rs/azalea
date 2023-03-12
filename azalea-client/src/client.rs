use crate::{
    chat::ChatPlugin,
    disconnect::{DisconnectEvent, DisconnectPlugin},
    events::{Event, EventPlugin, LocalPlayerEvents},
    interact::{CurrentSequenceNumber, InteractPlugin},
    inventory::{InventoryComponent, InventoryPlugin},
    local_player::{
        death_event, handle_send_packet_event, update_in_loaded_chunk, GameProfileComponent,
        LocalPlayer, PhysicsState, SendPacketEvent,
    },
    movement::{LastSentLookDirection, PlayerMovePlugin},
    packet_handling::{self, PacketHandlerPlugin, PacketReceiver},
    player::retroactively_add_game_profile_component,
    task_pool::TaskPoolPlugin,
    Account, PlayerInfo,
};

use azalea_auth::{game_profile::GameProfile, sessionserver::ClientSessionServerError};
use azalea_chat::FormattedText;
use azalea_physics::{PhysicsPlugin, PhysicsSet};
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
            serverbound_key_packet::ServerboundKeyPacket, ClientboundLoginPacket,
        },
        ConnectionProtocol, PROTOCOL_VERSION,
    },
    resolver, ServerAddress,
};
use azalea_world::{
    entity::{EntityPlugin, EntityUpdateSet, Local, WorldName},
    Instance, InstanceContainer, PartialInstance,
};
use bevy_app::{App, CoreSchedule, Plugin, PluginGroup, PluginGroupBuilder};
use bevy_ecs::{
    bundle::Bundle,
    component::Component,
    entity::Entity,
    schedule::IntoSystemConfig,
    schedule::{LogLevel, ScheduleBuildSettings, ScheduleLabel},
    world::World,
};
use bevy_log::LogPlugin;
use bevy_time::{prelude::FixedTime, TimePlugin};
use derive_more::{Deref, DerefMut};
use log::{debug, error};
use parking_lot::{Mutex, RwLock};
use std::{collections::HashMap, fmt::Debug, io, net::SocketAddr, sync::Arc, time::Duration};
use thiserror::Error;
use tokio::{sync::mpsc, time};
use uuid::Uuid;

/// `Client` has the things that a user interacting with the library will want.
/// Things that a player in the world will want to know are in [`LocalPlayer`].
///
/// To make a new client, use either [`azalea::ClientBuilder`] or
/// [`Client::join`].
///
/// Note that `Client` is inaccessible from systems (i.e. plugins), but you can
/// achieve everything that client can do with events.
///
/// [`azalea::ClientBuilder`]: https://docs.rs/azalea/latest/azalea/struct.ClientBuilder.html
#[derive(Clone)]
pub struct Client {
    /// The [`GameProfile`] for our client. This contains your username, UUID,
    /// and skin data.
    ///
    /// This is immutable; the server cannot change it. To get the username and
    /// skin the server chose for you, get your player from the [`TabList`]
    /// component.
    pub profile: GameProfile,
    /// The entity for this client in the ECS.
    pub entity: Entity,
    /// The world that this client is in.
    pub world: Arc<RwLock<PartialInstance>>,

    /// The entity component system. You probably don't need to access this
    /// directly. Note that if you're using a shared world (i.e. a swarm), this
    /// will contain all entities in all worlds.
    pub ecs: Arc<Mutex<World>>,

    /// Use this to force the client to run the schedule outside of a tick.
    pub run_schedule_sender: mpsc::UnboundedSender<()>,
}

/// A component that contains some of the "settings" for this client that are
/// sent to the server, such as render distance.
pub type ClientInformation = ServerboundClientInformationPacket;

/// A component that contains a map of player UUIDs to their information in the
/// tab list.
///
/// ```
/// # fn example(client: &azalea_client::Client) {
/// let tab_list = client.component::<TabList>();
/// println!("Online players:");
/// for (uuid, player_info) in tab_list {
///     println!("- {} ({}ms)", player_info.profile.name, player_info.latency);
/// }
/// # }
#[derive(Component, Clone, Debug, Deref, DerefMut, Default)]
pub struct TabList(HashMap<Uuid, PlayerInfo>);

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
    pub fn new(
        profile: GameProfile,
        entity: Entity,
        ecs: Arc<Mutex<World>>,
        run_schedule_sender: mpsc::UnboundedSender<()>,
    ) -> Self {
        Self {
            profile,
            // default our id to 0, it'll be set later
            entity,
            world: Arc::new(RwLock::new(PartialInstance::default())),

            ecs,

            run_schedule_sender,
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
        let (run_schedule_sender, run_schedule_receiver) = mpsc::unbounded_channel();
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
        ecs_lock: Arc<Mutex<World>>,
        account: &Account,
        address: &ServerAddress,
        resolved_address: &SocketAddr,
        run_schedule_sender: mpsc::UnboundedSender<()>,
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
        let client = Client::new(
            game_profile.clone(),
            entity,
            ecs_lock.clone(),
            run_schedule_sender.clone(),
        );

        let (packet_writer_sender, packet_writer_receiver) = mpsc::unbounded_channel();

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

        let local_player = crate::local_player::LocalPlayer::new(
            entity,
            packet_writer_sender,
            // default to an empty world, it'll be set correctly later when we
            // get the login packet
            Arc::new(RwLock::new(Instance::default())),
            read_packets_task,
            write_packets_task,
        );

        ecs.entity_mut(entity).insert(JoinedClientBundle {
            local_player,
            packet_receiver,
            game_profile: GameProfileComponent(game_profile),
            physics_state: PhysicsState::default(),
            local_player_events: LocalPlayerEvents(tx),
            inventory: InventoryComponent::default(),
            client_information: ClientInformation::default(),
            tab_list: TabList::default(),
            current_sequence_number: CurrentSequenceNumber::default(),
            last_sent_direction: LastSentLookDirection::default(),
            _local: Local,
        });

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
        self.ecs.lock().send_event(DisconnectEvent {
            entity: self.entity,
        });
    }

    pub fn local_player<'a>(&'a self, ecs: &'a mut World) -> &'a LocalPlayer {
        self.query::<&LocalPlayer>(ecs)
    }
    pub fn local_player_mut<'a>(
        &'a self,
        ecs: &'a mut World,
    ) -> bevy_ecs::world::Mut<'a, LocalPlayer> {
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

    /// Get a component from this client, or `None` if it doesn't exist.
    pub fn get_component<T: Component + Clone>(&self) -> Option<T> {
        self.query::<Option<&T>>(&mut self.ecs.lock()).cloned()
    }

    /// Get a reference to our (potentially shared) world.
    ///
    /// This gets the [`Instance`] from our world container. If it's a normal
    /// client, then it'll be the same as the world the client has loaded.
    /// If the client using a shared world, then the shared world will be a
    /// superset of the client's world.
    pub fn world(&self) -> Arc<RwLock<Instance>> {
        let world_name = self.component::<WorldName>();
        let ecs = self.ecs.lock();
        let instance_container = ecs.resource::<InstanceContainer>();
        instance_container.get(&world_name).unwrap()
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
            let mut ecs = self.ecs.lock();
            let mut client_information_mut = self.query::<&mut ClientInformation>(&mut ecs);
            *client_information_mut = client_information.clone();
        }

        if self.logged_in() {
            log::debug!(
                "Sending client information (already logged in): {:?}",
                client_information
            );
            self.write_packet(client_information.get());
        }

        Ok(())
    }
}

/// A bundle for the components that are present on a local player that received
/// a login packet. If you want to filter for this, just use [`Local`].
#[derive(Bundle)]
pub struct JoinedClientBundle {
    pub local_player: LocalPlayer,
    pub packet_receiver: PacketReceiver,
    pub game_profile: GameProfileComponent,
    pub physics_state: PhysicsState,
    pub local_player_events: LocalPlayerEvents,
    pub inventory: InventoryComponent,
    pub client_information: ClientInformation,
    pub tab_list: TabList,
    pub current_sequence_number: CurrentSequenceNumber,
    pub last_sent_direction: LastSentLookDirection,
    pub _local: Local,
}

pub struct AzaleaPlugin;
impl Plugin for AzaleaPlugin {
    fn build(&self, app: &mut App) {
        // Minecraft ticks happen every 50ms
        app.insert_resource(FixedTime::new(Duration::from_millis(50)));

        app.add_system(update_in_loaded_chunk.after(PhysicsSet));

        // fire the Death event when the player dies.
        app.add_system(death_event);

        // add GameProfileComponent when we get an AddPlayerEvent
        app.add_system(retroactively_add_game_profile_component.after(EntityUpdateSet::Index));

        app.add_event::<SendPacketEvent>()
            .add_system(handle_send_packet_event);

        app.init_resource::<InstanceContainer>();
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

    app.edit_schedule(CoreSchedule::Main, |schedule| {
        schedule.set_build_settings(ScheduleBuildSettings {
            ambiguity_detection: LogLevel::Warn,
            ..Default::default()
        });
    });

    app.add_plugins(DefaultPlugins);
    app
}

/// Start running the ECS loop! You must create your `App` from [`init_ecs_app`]
/// first.
#[doc(hidden)]
pub fn start_ecs(
    mut app: App,
    run_schedule_receiver: mpsc::UnboundedReceiver<()>,
    run_schedule_sender: mpsc::UnboundedSender<()>,
) -> Arc<Mutex<World>> {
    app.setup();

    // all resources should have been added by now so we can take the ecs from the
    // app
    let ecs = Arc::new(Mutex::new(app.world));

    tokio::spawn(run_schedule_loop(
        ecs.clone(),
        app.outer_schedule_label,
        run_schedule_receiver,
    ));
    tokio::spawn(tick_run_schedule_loop(run_schedule_sender));

    ecs
}

async fn run_schedule_loop(
    ecs: Arc<Mutex<World>>,
    outer_schedule_label: Box<dyn ScheduleLabel>,
    mut run_schedule_receiver: mpsc::UnboundedReceiver<()>,
) {
    loop {
        // whenever we get an event from run_schedule_receiver, run the schedule
        run_schedule_receiver.recv().await;
        let mut ecs = ecs.lock();
        ecs.run_schedule_ref(&*outer_schedule_label);
        ecs.clear_trackers();
    }
}

/// Send an event to run the schedule every 50 milliseconds. It will stop when
/// the receiver is dropped.
pub async fn tick_run_schedule_loop(run_schedule_sender: mpsc::UnboundedSender<()>) {
    let mut game_tick_interval = time::interval(time::Duration::from_millis(50));
    // TODO: Minecraft bursts up to 10 ticks and then skips, we should too
    game_tick_interval.set_missed_tick_behavior(time::MissedTickBehavior::Burst);

    loop {
        game_tick_interval.tick().await;
        if let Err(e) = run_schedule_sender.send(()) {
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
            .add(LogPlugin::default())
            .add(TimePlugin::default())
            .add(PacketHandlerPlugin)
            .add(AzaleaPlugin)
            .add(EntityPlugin)
            .add(PhysicsPlugin)
            .add(EventPlugin)
            .add(TaskPoolPlugin::default())
            .add(InventoryPlugin)
            .add(ChatPlugin)
            .add(DisconnectPlugin)
            .add(PlayerMovePlugin)
            .add(InteractPlugin)
    }
}
