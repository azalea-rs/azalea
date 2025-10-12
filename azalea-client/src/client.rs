use std::{
    collections::HashMap,
    fmt::Debug,
    mem,
    net::SocketAddr,
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

use azalea_auth::game_profile::GameProfile;
use azalea_core::{
    data_registry::ResolvableDataRegistry, position::Vec3, resource_location::ResourceLocation,
    tick::GameTick,
};
use azalea_entity::{
    EntityUpdateSystems, PlayerAbilities, Position,
    dimensions::EntityDimensions,
    indexing::{EntityIdIndex, EntityUuidIndex},
    metadata::Health,
};
use azalea_physics::local_player::PhysicsState;
use azalea_protocol::{
    ServerAddress,
    connect::Proxy,
    packets::{Packet, game::ServerboundGamePacket},
    resolver,
};
use azalea_world::{Instance, InstanceContainer, InstanceName, MinecraftEntityId, PartialInstance};
use bevy_app::{App, AppExit, Plugin, PluginsState, SubApp, Update};
use bevy_ecs::{
    message::MessageCursor,
    prelude::*,
    schedule::{InternedScheduleLabel, LogLevel, ScheduleBuildSettings},
};
use parking_lot::{Mutex, RwLock};
use simdnbt::owned::NbtCompound;
use thiserror::Error;
use tokio::{
    sync::{
        mpsc::{self},
        oneshot,
    },
    time,
};
use tracing::{info, warn};
use uuid::Uuid;

use crate::{
    Account, DefaultPlugins,
    attack::{self},
    block_update::QueuedServerBlockUpdates,
    chunks::ChunkBatchInfo,
    connection::RawConnection,
    disconnect::DisconnectEvent,
    events::Event,
    interact::BlockStatePredictionHandler,
    inventory::Inventory,
    join::{ConnectOpts, StartJoinServerEvent},
    local_player::{Hunger, InstanceHolder, PermissionLevel, TabList},
    mining::{self},
    movement::LastSentLookDirection,
    packet::game::SendGamePacketEvent,
    player::{GameProfileComponent, PlayerInfo, retroactively_add_game_profile_component},
};

/// A Minecraft client instance that can interact with the world.
///
/// To make a new client, use either [`azalea::ClientBuilder`] or
/// [`Client::join`].
///
/// Note that `Client` is inaccessible from systems (i.e. plugins), but you can
/// achieve everything that client can do with ECS events.
///
/// [`azalea::ClientBuilder`]: https://docs.rs/azalea/latest/azalea/struct.ClientBuilder.html
#[derive(Clone)]
pub struct Client {
    /// The entity for this client in the ECS.
    pub entity: Entity,

    /// A mutually exclusive reference to the entity component system (ECS).
    ///
    /// You probably don't need to access this directly. Note that if you're
    /// using a shared world (i.e. a swarm), the ECS will contain all entities
    /// in all instances/dimensions.
    pub ecs: Arc<Mutex<World>>,
}

/// An error that happened while joining the server.
#[derive(Error, Debug)]
pub enum JoinError {
    #[error("{0}")]
    Resolver(#[from] resolver::ResolverError),
    #[error("The given address could not be parsed into a ServerAddress")]
    InvalidAddress,
}

pub struct StartClientOpts {
    pub ecs_lock: Arc<Mutex<World>>,
    pub account: Account,
    pub connect_opts: ConnectOpts,
    pub event_sender: Option<mpsc::UnboundedSender<Event>>,
}

impl StartClientOpts {
    pub fn new(
        account: Account,
        address: ServerAddress,
        resolved_address: SocketAddr,
        event_sender: Option<mpsc::UnboundedSender<Event>>,
    ) -> StartClientOpts {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);

        // appexit_rx is unused here since the user should be able to handle it
        // themselves if they're using StartClientOpts::new
        let (ecs_lock, start_running_systems, _appexit_rx) = start_ecs_runner(app.main_mut());
        start_running_systems();

        Self {
            ecs_lock,
            account,
            connect_opts: ConnectOpts {
                address,
                resolved_address,
                proxy: None,
            },
            event_sender,
        }
    }

    pub fn proxy(mut self, proxy: Proxy) -> Self {
        self.connect_opts.proxy = Some(proxy);
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
    /// use azalea_client::{Account, Client};
    ///
    /// #[tokio::main(flavor = "current_thread")]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let account = Account::offline("bot");
    ///     let (client, rx) = Client::join(account, "localhost").await?;
    ///     client.chat("Hello, world!");
    ///     client.disconnect();
    ///     Ok(())
    /// }
    /// ```
    pub async fn join(
        account: Account,
        address: impl TryInto<ServerAddress>,
    ) -> Result<(Self, mpsc::UnboundedReceiver<Event>), JoinError> {
        let address: ServerAddress = address.try_into().map_err(|_| JoinError::InvalidAddress)?;
        let resolved_address = resolver::resolve_address(&address).await?;
        let (tx, rx) = mpsc::unbounded_channel();

        let client = Self::start_client(StartClientOpts::new(
            account,
            address,
            resolved_address,
            Some(tx),
        ))
        .await;
        Ok((client, rx))
    }

    pub async fn join_with_proxy(
        account: Account,
        address: impl TryInto<ServerAddress>,
        proxy: Proxy,
    ) -> Result<(Self, mpsc::UnboundedReceiver<Event>), JoinError> {
        let address: ServerAddress = address.try_into().map_err(|_| JoinError::InvalidAddress)?;
        let resolved_address = resolver::resolve_address(&address).await?;
        let (tx, rx) = mpsc::unbounded_channel();

        let client = Self::start_client(
            StartClientOpts::new(account, address, resolved_address, Some(tx)).proxy(proxy),
        )
        .await;
        Ok((client, rx))
    }

    /// Create a [`Client`] when you already have the ECS made with
    /// [`start_ecs_runner`]. You'd usually want to use [`Self::join`] instead.
    pub async fn start_client(
        StartClientOpts {
            ecs_lock,
            account,
            connect_opts,
            event_sender,
        }: StartClientOpts,
    ) -> Self {
        // send a StartJoinServerEvent

        let (start_join_callback_tx, mut start_join_callback_rx) =
            mpsc::unbounded_channel::<Entity>();

        ecs_lock.lock().write_message(StartJoinServerEvent {
            account,
            connect_opts,
            event_sender,
            start_join_callback_tx: Some(start_join_callback_tx),
        });

        let entity = start_join_callback_rx.recv().await.expect(
            "start_join_callback should not be dropped before sending a message, this is a bug in Azalea",
        );

        Client::new(entity, ecs_lock)
    }

    /// Write a packet directly to the server.
    pub fn write_packet(&self, packet: impl Packet<ServerboundGamePacket>) {
        let packet = packet.into_variant();
        self.ecs
            .lock()
            .commands()
            .trigger(SendGamePacketEvent::new(self.entity, packet));
    }

    /// Disconnect this client from the server by ending all tasks.
    ///
    /// The OwnedReadHalf for the TCP connection is in one of the tasks, so it
    /// automatically closes the connection when that's dropped.
    pub fn disconnect(&self) {
        self.ecs.lock().write_message(DisconnectEvent {
            entity: self.entity,
            reason: None,
        });
    }

    pub fn with_raw_connection<R>(&self, f: impl FnOnce(&RawConnection) -> R) -> R {
        self.query_self::<&RawConnection, _>(f)
    }
    pub fn with_raw_connection_mut<R>(&self, f: impl FnOnce(Mut<'_, RawConnection>) -> R) -> R {
        self.query_self::<&mut RawConnection, _>(f)
    }

    /// Get a component from this client. This will clone the component and
    /// return it.
    ///
    ///
    /// If the component can't be cloned, try [`Self::query_self`] instead.
    /// If it isn't guaranteed to be present, you can use
    /// [`Self::get_component`] or [`Self::query_self`].
    ///
    ///
    /// You may also use [`Self::ecs`] directly if you need more control over
    /// when the ECS is locked.
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
        self.query_self::<&T, _>(|t| t.clone())
    }

    /// Get a component from this client, or `None` if it doesn't exist.
    ///
    /// If the component can't be cloned, consider using [`Self::query_self`]
    /// with `Option<&T>` instead.
    ///
    /// You may also have to use [`Self::query_self`] directly.
    pub fn get_component<T: Component + Clone>(&self) -> Option<T> {
        self.query_self::<Option<&T>, _>(|t| t.cloned())
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
        self.query_self::<Option<&InstanceName>, _>(|ins| ins.is_some())
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

    /// Get the bounding box dimensions for our client, which contains our
    /// width, height, and eye height.
    ///
    /// This is a shortcut for
    /// `self.component::<EntityDimensions>()`.
    pub fn dimensions(&self) -> EntityDimensions {
        self.component::<EntityDimensions>()
    }

    /// Get the position of this client's eyes.
    ///
    /// This is a shortcut for
    /// `bot.position().up(bot.dimensions().eye_height)`.
    pub fn eye_position(&self) -> Vec3 {
        self.query_self::<(&Position, &EntityDimensions), _>(|(pos, dim)| {
            pos.up(dim.eye_height as f64)
        })
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
        self.query_self::<&EntityIdIndex, _>(|entity_id_index| {
            entity_id_index.get_by_ecs_entity(entity)
        })
    }
    /// Convert a [`MinecraftEntityId`] to an ECS `Entity`.
    pub fn ecs_entity_by_minecraft_entity(&self, entity: MinecraftEntityId) -> Option<Entity> {
        self.query_self::<&EntityIdIndex, _>(|entity_id_index| {
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
    pub instance_holder: InstanceHolder,

    pub metadata: azalea_entity::metadata::PlayerMetadataBundle,
}

/// A bundle for the components that are present on a local player that is
/// currently in the `game` protocol state.
///
/// If you want to filter for this, use [`InGameState`].
#[derive(Bundle, Default)]
pub struct JoinedClientBundle {
    // note that InstanceHolder isn't here because it's set slightly before we fully join the world
    pub physics_state: PhysicsState,
    pub inventory: Inventory,
    pub tab_list: TabList,
    pub block_state_prediction_handler: BlockStatePredictionHandler,
    pub queued_server_block_updates: QueuedServerBlockUpdates,
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
                retroactively_add_game_profile_component
                    .after(EntityUpdateSystems::Index)
                    .after(crate::join::handle_start_join_server_event),
            ),
        )
        .init_resource::<InstanceContainer>()
        .init_resource::<TabList>();
    }
}

/// Create the ECS world, and return a function that begins running systems.
/// This exists to allow you to make last-millisecond updates to the world
/// before any systems start running.
///
/// You can create your app with `App::new()`, but don't forget to add
/// [`DefaultPlugins`].
#[doc(hidden)]
pub fn start_ecs_runner(
    app: &mut SubApp,
) -> (Arc<Mutex<World>>, impl FnOnce(), oneshot::Receiver<AppExit>) {
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
    let ecs = Arc::new(Mutex::new(mem::take(app.world_mut())));

    let ecs_clone = ecs.clone();
    let outer_schedule_label = *app.update_schedule.as_ref().unwrap();

    let (appexit_tx, appexit_rx) = oneshot::channel();
    let start_running_systems = move || {
        tokio::spawn(async move {
            let appexit = run_schedule_loop(ecs_clone, outer_schedule_label).await;
            appexit_tx.send(appexit)
        });
    };

    (ecs, start_running_systems, appexit_rx)
}

/// Runs the `Update` schedule 60 times per second and the `GameTick` schedule
/// 20 times per second.
///
/// Exits when we receive an `AppExit` event.
async fn run_schedule_loop(
    ecs: Arc<Mutex<World>>,
    outer_schedule_label: InternedScheduleLabel,
) -> AppExit {
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
        if let Some(exit) = should_exit(&mut ecs) {
            // it's possible for references to the World to stay around, so we clear the ecs
            ecs.clear_all();
            // ^ note that this also forcefully disconnects all of our bots without sending
            // a disconnect packet (which is fine because we want to disconnect immediately)

            return exit;
        }
    }
}

/// Checks whether the [`AppExit`] event was sent, and if so returns it.
///
/// This is based on Bevy's `should_exit` function: https://github.com/bevyengine/bevy/blob/b9fd7680e78c4073dfc90fcfdc0867534d92abe0/crates/bevy_app/src/app.rs#L1292
fn should_exit(ecs: &mut World) -> Option<AppExit> {
    let mut reader = MessageCursor::default();

    let events = ecs.get_resource::<Messages<AppExit>>()?;
    let mut events = reader.read(events);

    if events.len() != 0 {
        return Some(
            events
                .find(|exit| exit.is_error())
                .cloned()
                .unwrap_or(AppExit::Success),
        );
    }

    None
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
