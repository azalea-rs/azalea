use std::{
    collections::HashMap,
    fmt::Debug,
    io,
    net::SocketAddr,
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

use azalea_auth::game_profile::GameProfile;
use azalea_chat::FormattedText;
use azalea_core::{
    data_registry::ResolvableDataRegistry, position::Vec3, resource_location::ResourceLocation,
    tick::GameTick,
};
use azalea_entity::{
    EntityUpdateSet, EyeHeight, LocalEntity, Position,
    indexing::{EntityIdIndex, EntityUuidIndex},
    metadata::Health,
};
use azalea_protocol::{
    ServerAddress,
    common::client_information::ClientInformation,
    connect::{Connection, ConnectionError, Proxy},
    packets::{
        self, ClientIntention, ConnectionProtocol, PROTOCOL_VERSION, Packet,
        game::{self, ServerboundGamePacket},
        handshake::s_intention::ServerboundIntention,
        login::s_hello::ServerboundHello,
    },
    resolver,
};
use azalea_world::{Instance, InstanceContainer, InstanceName, MinecraftEntityId, PartialInstance};
use bevy_app::{App, Plugin, PluginsState, Update};
use bevy_ecs::{
    bundle::Bundle,
    component::Component,
    entity::Entity,
    schedule::{InternedScheduleLabel, IntoSystemConfigs, LogLevel, ScheduleBuildSettings},
    system::{Commands, ResMut, Resource},
    world::World,
};
use derive_more::Deref;
use parking_lot::{Mutex, RwLock};
use simdnbt::owned::NbtCompound;
use thiserror::Error;
use tokio::{
    sync::{broadcast, mpsc},
    time,
};
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use crate::{
    Account, DefaultPlugins, PlayerInfo,
    attack::{self},
    chunks::ChunkBatchInfo,
    connection::RawConnection,
    disconnect::DisconnectEvent,
    events::{Event, LocalPlayerEvents},
    interact::CurrentSequenceNumber,
    inventory::Inventory,
    local_player::{
        GameProfileComponent, Hunger, InstanceHolder, PermissionLevel, PlayerAbilities, TabList,
    },
    mining::{self},
    movement::{LastSentLookDirection, PhysicsState},
    packet::{
        as_system,
        game::SendPacketEvent,
        login::{InLoginState, SendLoginPacketEvent},
    },
    player::retroactively_add_game_profile_component,
};

/// `Client` has the things that a user interacting with the library will want.
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
    /// The entity for this client in the ECS.
    pub entity: Entity,

    /// The entity component system. You probably don't need to access this
    /// directly. Note that if you're using a shared world (i.e. a swarm), this
    /// will contain all entities in all worlds.
    pub ecs: Arc<Mutex<World>>,
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
    #[error("Failed to encrypt the challenge from the server for {0:?}")]
    EncryptionError(packets::login::ClientboundHello),
    #[error("{0}")]
    SessionServer(#[from] azalea_auth::sessionserver::ClientSessionServerError),
    #[error("The given address could not be parsed into a ServerAddress")]
    InvalidAddress,
    #[error("Couldn't refresh access token: {0}")]
    Auth(#[from] azalea_auth::AuthError),
    #[error("Disconnected: {reason}")]
    Disconnect { reason: FormattedText },
}

pub struct StartClientOpts<'a> {
    pub ecs_lock: Arc<Mutex<World>>,
    pub account: &'a Account,
    pub address: &'a ServerAddress,
    pub resolved_address: &'a SocketAddr,
    pub proxy: Option<Proxy>,
    pub event_sender: Option<mpsc::UnboundedSender<Event>>,
}

impl<'a> StartClientOpts<'a> {
    pub fn new(
        account: &'a Account,
        address: &'a ServerAddress,
        resolved_address: &'a SocketAddr,
        event_sender: Option<mpsc::UnboundedSender<Event>>,
    ) -> StartClientOpts<'a> {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);

        let ecs_lock = start_ecs_runner(app);

        Self {
            ecs_lock,
            account,
            address,
            resolved_address,
            proxy: None,
            event_sender,
        }
    }

    pub fn proxy(mut self, proxy: Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}

impl Client {
    /// Create a new client from the given [`GameProfile`], ECS Entity, ECS
    /// World, and schedule runner function.
    /// You should only use this if you want to change these fields from the
    /// defaults, otherwise use [`Client::join`].
    pub fn new(entity: Entity, ecs: Arc<Mutex<World>>) -> Self {
        Self {
            // default our id to 0, it'll be set later
            entity,

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
        let (tx, rx) = mpsc::unbounded_channel();

        let client = Self::start_client(StartClientOpts::new(
            account,
            &address,
            &resolved_address,
            Some(tx),
        ))
        .await?;
        Ok((client, rx))
    }

    pub async fn join_with_proxy(
        account: &Account,
        address: impl TryInto<ServerAddress>,
        proxy: Proxy,
    ) -> Result<(Self, mpsc::UnboundedReceiver<Event>), JoinError> {
        let address: ServerAddress = address.try_into().map_err(|_| JoinError::InvalidAddress)?;
        let resolved_address = resolver::resolve_address(&address).await?;
        let (tx, rx) = mpsc::unbounded_channel();

        let client = Self::start_client(
            StartClientOpts::new(account, &address, &resolved_address, Some(tx)).proxy(proxy),
        )
        .await?;
        Ok((client, rx))
    }

    /// Create a [`Client`] when you already have the ECS made with
    /// [`start_ecs_runner`]. You'd usually want to use [`Self::join`] instead.
    pub async fn start_client(
        StartClientOpts {
            ecs_lock,
            account,
            address,
            resolved_address,
            proxy,
            event_sender,
        }: StartClientOpts<'_>,
    ) -> Result<Self, JoinError> {
        // check if an entity with our uuid already exists in the ecs and if so then
        // just use that
        let entity = {
            let mut ecs = ecs_lock.lock();

            let entity_uuid_index = ecs.resource::<EntityUuidIndex>();
            let uuid = account.uuid_or_offline();
            let entity = if let Some(entity) = entity_uuid_index.get(&account.uuid_or_offline()) {
                debug!("Reusing entity {entity:?} for client");
                entity
            } else {
                let entity = ecs.spawn_empty().id();
                debug!("Created new entity {entity:?} for client");
                // add to the uuid index
                let mut entity_uuid_index = ecs.resource_mut::<EntityUuidIndex>();
                entity_uuid_index.insert(uuid, entity);
                entity
            };

            let mut entity_mut = ecs.entity_mut(entity);
            entity_mut.insert((
                InLoginState,
                // add the Account to the entity now so plugins can access it earlier
                account.to_owned(),
                // localentity is always present for our clients, even if we're not actually logged
                // in
                LocalEntity,
            ));
            if let Some(event_sender) = event_sender {
                // this is optional so we don't leak memory in case the user doesn't want to
                // handle receiving packets
                entity_mut.insert(LocalPlayerEvents(event_sender));
            }

            entity
        };

        let mut conn = if let Some(proxy) = proxy {
            Connection::new_with_proxy(resolved_address, proxy).await?
        } else {
            Connection::new(resolved_address).await?
        };
        debug!("Created connection to {resolved_address:?}");

        conn.write(ServerboundIntention {
            protocol_version: PROTOCOL_VERSION,
            hostname: address.host.clone(),
            port: address.port,
            intention: ClientIntention::Login,
        })
        .await?;
        let conn = conn.login();

        let (read_conn, write_conn) = conn.into_split();
        let (read_conn, write_conn) = (read_conn.raw, write_conn.raw);

        // insert the client into the ecs so it finishes logging in
        {
            let mut ecs = ecs_lock.lock();

            let instance = Instance::default();
            let instance_holder = crate::local_player::InstanceHolder::new(
                entity,
                // default to an empty world, it'll be set correctly later when we
                // get the login packet
                Arc::new(RwLock::new(instance)),
            );

            let mut entity = ecs.entity_mut(entity);
            entity.insert((
                // these stay when we switch to the game state
                LocalPlayerBundle {
                    raw_connection: RawConnection::new(
                        read_conn,
                        write_conn,
                        ConnectionProtocol::Login,
                    ),
                    client_information: crate::ClientInformation::default(),
                    instance_holder,
                    metadata: azalea_entity::metadata::PlayerMetadataBundle::default(),
                },
            ));
        }

        as_system::<Commands>(&mut ecs_lock.lock(), |mut commands| {
            commands.entity(entity).insert((InLoginState,));
            commands.trigger(SendLoginPacketEvent::new(
                entity,
                ServerboundHello {
                    name: account.username.clone(),
                    // TODO: pretty sure this should generate an offline-mode uuid instead of just
                    // Uuid::default()
                    profile_id: account.uuid.unwrap_or_default(),
                },
            ))
        });

        let client = Client::new(entity, ecs_lock.clone());
        Ok(client)
    }

    /// Write a packet directly to the server.
    pub fn write_packet(&self, packet: impl Packet<ServerboundGamePacket>) {
        let packet = packet.into_variant();
        self.ecs
            .lock()
            .commands()
            .trigger(SendPacketEvent::new(self.entity, packet));
    }

    /// Disconnect this client from the server by ending all tasks.
    ///
    /// The OwnedReadHalf for the TCP connection is in one of the tasks, so it
    /// automatically closes the connection when that's dropped.
    pub fn disconnect(&self) {
        self.ecs.lock().send_event(DisconnectEvent {
            entity: self.entity,
            reason: None,
        });
    }

    pub fn raw_connection<'a>(&'a self, ecs: &'a mut World) -> &'a RawConnection {
        self.query::<&RawConnection>(ecs)
    }
    pub fn raw_connection_mut<'a>(
        &'a self,
        ecs: &'a mut World,
    ) -> bevy_ecs::world::Mut<'a, RawConnection> {
        self.query::<&mut RawConnection>(ecs)
    }

    /// Get a component from this client. This will clone the component and
    /// return it.
    ///
    ///
    /// If the component can't be cloned, try [`Self::map_component`] instead.
    /// If it isn't guaranteed to be present, use [`Self::get_component`] or
    /// [`Self::map_get_component`].
    ///
    /// You may also use [`Self::ecs`] and [`Self::query`] directly if you need
    /// more control over when the ECS is locked.
    ///
    /// # Panics
    ///
    /// This will panic if the component doesn't exist on the client.
    ///
    /// # Examples
    ///
    /// ```
    /// # use azalea_world::InstanceName;
    /// # fn example(client: &azalea_client::Client) {
    /// let world_name = client.component::<InstanceName>();
    /// # }
    pub fn component<T: Component + Clone>(&self) -> T {
        self.query::<&T>(&mut self.ecs.lock()).clone()
    }

    /// Get a component from this client, or `None` if it doesn't exist.
    ///
    /// If the component can't be cloned, try [`Self::map_component`] instead.
    /// You may also have to use [`Self::ecs`] and [`Self::query`] directly.
    pub fn get_component<T: Component + Clone>(&self) -> Option<T> {
        self.query::<Option<&T>>(&mut self.ecs.lock()).cloned()
    }

    /// Get a resource from the ECS. This will clone the resource and return it.
    pub fn resource<T: Resource + Clone>(&self) -> T {
        self.ecs.lock().resource::<T>().clone()
    }

    /// Get a required ECS resource and call the given function with it.
    pub fn map_resource<T: Resource, R>(&self, f: impl FnOnce(&T) -> R) -> R {
        let ecs = self.ecs.lock();
        let value = ecs.resource::<T>();
        f(value)
    }

    /// Get an optional ECS resource and call the given function with it.
    pub fn map_get_resource<T: Resource, R>(&self, f: impl FnOnce(Option<&T>) -> R) -> R {
        let ecs = self.ecs.lock();
        let value = ecs.get_resource::<T>();
        f(value)
    }

    /// Get a required component for this client and call the given function.
    ///
    /// Similar to [`Self::component`], but doesn't clone the component since
    /// it's passed as a reference. [`Self::ecs`] will remain locked while the
    /// callback is being run.
    ///
    /// If the component is not guaranteed to be present, use
    /// [`Self::get_component`] instead.
    ///
    /// # Panics
    ///
    /// This will panic if the component doesn't exist on the client.
    ///
    /// ```
    /// # use azalea_client::{Client, Hunger};
    /// # fn example(bot: &Client) {
    /// let hunger = bot.map_component::<Hunger, _>(|h| h.food);
    /// # }
    /// ```
    pub fn map_component<T: Component, R>(&self, f: impl FnOnce(&T) -> R) -> R {
        let mut ecs = self.ecs.lock();
        let value = self.query::<&T>(&mut ecs);
        f(value)
    }

    /// Optionally get a component for this client and call the given function.
    ///
    /// Similar to [`Self::get_component`], but doesn't clone the component
    /// since it's passed as a reference. [`Self::ecs`] will remain locked
    /// while the callback is being run.
    ///
    /// ```
    /// # use azalea_client::{Client, mining::Mining};
    /// # fn example(bot: &Client) {
    /// let is_mining = bot.map_get_component::<Mining, _>(|m| m.is_some());
    /// # }
    /// ```
    pub fn map_get_component<T: Component, R>(&self, f: impl FnOnce(Option<&T>) -> R) -> R {
        let mut ecs = self.ecs.lock();
        let value = self.query::<Option<&T>>(&mut ecs);
        f(value)
    }

    /// Get an `RwLock` with a reference to our (potentially shared) world.
    ///
    /// This gets the [`Instance`] from the client's [`InstanceHolder`]
    /// component. If it's a normal client, then it'll be the same as the
    /// world the client has loaded. If the client is using a shared world,
    /// then the shared world will be a superset of the client's world.
    pub fn world(&self) -> Arc<RwLock<Instance>> {
        let instance_holder = self.component::<InstanceHolder>();
        instance_holder.instance.clone()
    }

    /// Get an `RwLock` with a reference to the world that this client has
    /// loaded.
    ///
    /// ```
    /// # use azalea_core::position::ChunkPos;
    /// # fn example(client: &azalea_client::Client) {
    /// let world = client.partial_world();
    /// let is_0_0_loaded = world.read().chunks.limited_get(&ChunkPos::new(0, 0)).is_some();
    /// # }
    pub fn partial_world(&self) -> Arc<RwLock<PartialInstance>> {
        let instance_holder = self.component::<InstanceHolder>();
        instance_holder.partial_instance.clone()
    }

    /// Returns whether we have a received the login packet yet.
    pub fn logged_in(&self) -> bool {
        // the login packet tells us the world name
        self.query::<Option<&InstanceName>>(&mut self.ecs.lock())
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
    /// .await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn set_client_information(&self, client_information: ClientInformation) {
        {
            let mut ecs = self.ecs.lock();
            let mut client_information_mut = self.query::<&mut ClientInformation>(&mut ecs);
            *client_information_mut = client_information.clone();
        }

        if self.logged_in() {
            debug!(
                "Sending client information (already logged in): {:?}",
                client_information
            );
            self.write_packet(game::s_client_information::ServerboundClientInformation {
                client_information,
            });
        }
    }
}

impl Client {
    /// Get the position of this client.
    ///
    /// This is a shortcut for `Vec3::from(&bot.component::<Position>())`.
    ///
    /// Note that this value is given a default of [`Vec3::ZERO`] when it
    /// receives the login packet, its true position may be set ticks
    /// later.
    pub fn position(&self) -> Vec3 {
        Vec3::from(
            &self
                .get_component::<Position>()
                .expect("the client's position hasn't been initialized yet"),
        )
    }

    /// Get the position of this client's eyes.
    ///
    /// This is a shortcut for
    /// `bot.position().up(bot.component::<EyeHeight>())`.
    pub fn eye_position(&self) -> Vec3 {
        self.position().up((*self.component::<EyeHeight>()) as f64)
    }

    /// Get the health of this client.
    ///
    /// This is a shortcut for `*bot.component::<Health>()`.
    pub fn health(&self) -> f32 {
        *self.component::<Health>()
    }

    /// Get the hunger level of this client, which includes both food and
    /// saturation.
    ///
    /// This is a shortcut for `self.component::<Hunger>().to_owned()`.
    pub fn hunger(&self) -> Hunger {
        self.component::<Hunger>().to_owned()
    }

    /// Get the username of this client.
    ///
    /// This is a shortcut for
    /// `bot.component::<GameProfileComponent>().name.to_owned()`.
    pub fn username(&self) -> String {
        self.profile().name.to_owned()
    }

    /// Get the Minecraft UUID of this client.
    ///
    /// This is a shortcut for `bot.component::<GameProfileComponent>().uuid`.
    pub fn uuid(&self) -> Uuid {
        self.profile().uuid
    }

    /// Get a map of player UUIDs to their information in the tab list.
    ///
    /// This is a shortcut for `*bot.component::<TabList>()`.
    pub fn tab_list(&self) -> HashMap<Uuid, PlayerInfo> {
        (*self.component::<TabList>()).clone()
    }

    /// Returns the [`GameProfile`] for our client. This contains your username,
    /// UUID, and skin data.
    ///
    /// These values are set by the server upon login, which means they might
    /// not match up with your actual game profile. Also, note that the username
    /// and skin that gets displayed in-game will actually be the ones from
    /// the tab list, which you can get from [`Self::tab_list`].
    ///
    /// This as also available from the ECS as [`GameProfileComponent`].
    pub fn profile(&self) -> GameProfile {
        (*self.component::<GameProfileComponent>()).clone()
    }

    /// A convenience function to get the Minecraft Uuid of a player by their
    /// username, if they're present in the tab list.
    ///
    /// You can chain this with [`Client::entity_by_uuid`] to get the ECS
    /// `Entity` for the player.
    pub fn player_uuid_by_username(&self, username: &str) -> Option<Uuid> {
        self.tab_list()
            .values()
            .find(|player| player.profile.name == username)
            .map(|player| player.profile.uuid)
    }

    /// Get an ECS `Entity` in the world by its Minecraft UUID, if it's within
    /// render distance.
    pub fn entity_by_uuid(&self, uuid: Uuid) -> Option<Entity> {
        self.map_resource::<EntityUuidIndex, _>(|entity_uuid_index| entity_uuid_index.get(&uuid))
    }

    /// Convert an ECS `Entity` to a [`MinecraftEntityId`].
    pub fn minecraft_entity_by_ecs_entity(&self, entity: Entity) -> Option<MinecraftEntityId> {
        self.map_component::<EntityIdIndex, _>(|entity_id_index| {
            entity_id_index.get_by_ecs_entity(entity)
        })
    }
    /// Convert a [`MinecraftEntityId`] to an ECS `Entity`.
    pub fn ecs_entity_by_minecraft_entity(&self, entity: MinecraftEntityId) -> Option<Entity> {
        self.map_component::<EntityIdIndex, _>(|entity_id_index| {
            entity_id_index.get_by_minecraft_entity(entity)
        })
    }

    /// Call the given function with the client's [`RegistryHolder`].
    ///
    /// The player's instance (aka world) will be locked during this time, which
    /// may result in a deadlock if you try to access the instance again while
    /// in the function.
    ///
    /// [`RegistryHolder`]: azalea_core::registry_holder::RegistryHolder
    pub fn with_registry_holder<R>(
        &self,
        f: impl FnOnce(&azalea_core::registry_holder::RegistryHolder) -> R,
    ) -> R {
        let instance = self.world();
        let registries = &instance.read().registries;
        f(registries)
    }

    /// Resolve the given registry to its name.
    ///
    /// This is necessary for data-driven registries like [`Enchantment`].
    ///
    /// [`Enchantment`]: azalea_registry::Enchantment
    pub fn resolve_registry_name(
        &self,
        registry: &impl ResolvableDataRegistry,
    ) -> Option<ResourceLocation> {
        self.with_registry_holder(|registries| registry.resolve_name(registries))
    }
    /// Resolve the given registry to its name and data and call the given
    /// function with it.
    ///
    /// This is necessary for data-driven registries like [`Enchantment`].
    ///
    /// If you just want the value name, use [`Self::resolve_registry_name`]
    /// instead.
    ///
    /// [`Enchantment`]: azalea_registry::Enchantment
    pub fn with_resolved_registry<R>(
        &self,
        registry: impl ResolvableDataRegistry,
        f: impl FnOnce(&ResourceLocation, &NbtCompound) -> R,
    ) -> Option<R> {
        self.with_registry_holder(|registries| {
            registry
                .resolve(registries)
                .map(|(name, data)| f(name, data))
        })
    }
}

/// A bundle of components that's inserted right when we switch to the `login`
/// state and stay present on our clients until we disconnect.
///
/// For the components that are only present in the `game` state, see
/// [`JoinedClientBundle`].
#[derive(Bundle)]
pub struct LocalPlayerBundle {
    pub raw_connection: RawConnection,
    pub client_information: ClientInformation,
    pub instance_holder: InstanceHolder,

    pub metadata: azalea_entity::metadata::PlayerMetadataBundle,
}

/// A bundle for the components that are present on a local player that is
/// currently in the `game` protocol state. If you want to filter for this, use
/// [`InGameState`].
#[derive(Bundle, Default)]
pub struct JoinedClientBundle {
    // note that InstanceHolder isn't here because it's set slightly before we fully join the world
    pub physics_state: PhysicsState,
    pub inventory: Inventory,
    pub tab_list: TabList,
    pub current_sequence_number: CurrentSequenceNumber,
    pub last_sent_direction: LastSentLookDirection,
    pub abilities: PlayerAbilities,
    pub permission_level: PermissionLevel,
    pub chunk_batch_info: ChunkBatchInfo,
    pub hunger: Hunger,

    pub entity_id_index: EntityIdIndex,

    pub mining: mining::MineBundle,
    pub attack: attack::AttackBundle,

    pub in_game_state: InGameState,
}

/// A marker component for local players that are currently in the
/// `game` state.
#[derive(Component, Clone, Debug, Default)]
pub struct InGameState;
/// A marker component for local players that are currently in the
/// `configuration` state.
#[derive(Component, Clone, Debug, Default)]
pub struct InConfigState;

pub struct AzaleaPlugin;
impl Plugin for AzaleaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                // add GameProfileComponent when we get an AddPlayerEvent
                retroactively_add_game_profile_component.after(EntityUpdateSet::Index),
            ),
        )
        .init_resource::<InstanceContainer>()
        .init_resource::<TabList>();
    }
}

/// Start running the ECS loop!
///
/// You can create your app with `App::new()`, but don't forget to add
/// [`DefaultPlugins`].
#[doc(hidden)]
pub fn start_ecs_runner(mut app: App) -> Arc<Mutex<World>> {
    // this block is based on Bevy's default runner:
    // https://github.com/bevyengine/bevy/blob/390877cdae7a17095a75c8f9f1b4241fe5047e83/crates/bevy_app/src/schedule_runner.rs#L77-L85
    if app.plugins_state() != PluginsState::Cleaned {
        // Wait for plugins to load
        if app.plugins_state() == PluginsState::Adding {
            info!("Waiting for plugins to load ...");
            while app.plugins_state() == PluginsState::Adding {
                thread::yield_now();
            }
        }
        // Finish adding plugins and cleanup
        app.finish();
        app.cleanup();
    }

    // all resources should have been added by now so we can take the ecs from the
    // app
    let ecs = Arc::new(Mutex::new(std::mem::take(app.world_mut())));

    tokio::spawn(run_schedule_loop(
        ecs.clone(),
        *app.main().update_schedule.as_ref().unwrap(),
    ));

    ecs
}

async fn run_schedule_loop(ecs: Arc<Mutex<World>>, outer_schedule_label: InternedScheduleLabel) {
    let mut last_update: Option<Instant> = None;
    let mut last_tick: Option<Instant> = None;

    // azalea runs the Update schedule at most 60 times per second to simulate
    // framerate. unlike vanilla though, we also only handle packets during Updates
    // due to everything running in ecs systems.
    const UPDATE_DURATION_TARGET: Duration = Duration::from_micros(1_000_000 / 60);
    // minecraft runs at 20 tps
    const GAME_TICK_DURATION_TARGET: Duration = Duration::from_micros(1_000_000 / 20);

    loop {
        // sleep until the next update if necessary
        let now = Instant::now();
        if let Some(last_update) = last_update {
            let elapsed = now.duration_since(last_update);
            if elapsed < UPDATE_DURATION_TARGET {
                time::sleep(UPDATE_DURATION_TARGET - elapsed).await;
            }
        }
        last_update = Some(now);

        let mut ecs = ecs.lock();

        // if last tick is None or more than 50ms ago, run the GameTick schedule
        ecs.run_schedule(outer_schedule_label);
        if last_tick
            .map(|last_tick| last_tick.elapsed() > GAME_TICK_DURATION_TARGET)
            .unwrap_or(true)
        {
            if let Some(last_tick) = &mut last_tick {
                *last_tick += GAME_TICK_DURATION_TARGET;

                // if we're more than 10 ticks behind, set last_tick to now.
                // vanilla doesn't do it in exactly the same way but it shouldn't really matter
                if (now - *last_tick) > GAME_TICK_DURATION_TARGET * 10 {
                    warn!(
                        "GameTick is more than 10 ticks behind, skipping ticks so we don't have to burst too much"
                    );
                    *last_tick = now;
                }
            } else {
                last_tick = Some(now);
            }
            ecs.run_schedule(GameTick);
        }

        ecs.clear_trackers();
    }
}

/// A resource that contains a [`broadcast::Sender`] that will be sent every
/// Minecraft tick.
///
/// This is useful for running code every schedule from async user code.
///
/// ```
/// use azalea_client::TickBroadcast;
/// # async fn example(client: azalea_client::Client) {
/// let mut receiver = {
///     let ecs = client.ecs.lock();
///     let tick_broadcast = ecs.resource::<TickBroadcast>();
///     tick_broadcast.subscribe()
/// };
/// while receiver.recv().await.is_ok() {
///     // do something
/// }
/// # }
/// ```
#[derive(Resource, Deref)]
pub struct TickBroadcast(broadcast::Sender<()>);

pub fn send_tick_broadcast(tick_broadcast: ResMut<TickBroadcast>) {
    let _ = tick_broadcast.0.send(());
}
/// A plugin that makes the [`RanScheduleBroadcast`] resource available.
pub struct TickBroadcastPlugin;
impl Plugin for TickBroadcastPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TickBroadcast(broadcast::channel(1).0))
            .add_systems(GameTick, send_tick_broadcast);
    }
}

pub struct AmbiguityLoggerPlugin;
impl Plugin for AmbiguityLoggerPlugin {
    fn build(&self, app: &mut App) {
        app.edit_schedule(Update, |schedule| {
            schedule.set_build_settings(ScheduleBuildSettings {
                ambiguity_detection: LogLevel::Warn,
                ..Default::default()
            });
        });
        app.edit_schedule(GameTick, |schedule| {
            schedule.set_build_settings(ScheduleBuildSettings {
                ambiguity_detection: LogLevel::Warn,
                ..Default::default()
            });
        });
    }
}
