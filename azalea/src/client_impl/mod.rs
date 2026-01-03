use std::{collections::HashMap, sync::Arc};

use azalea_auth::game_profile::GameProfile;
use azalea_client::{
    DefaultPlugins,
    account::Account,
    connection::RawConnection,
    disconnect::DisconnectEvent,
    join::{ConnectOpts, StartJoinServerEvent},
    local_player::{Hunger, InstanceHolder, TabList},
    packet::game::SendGamePacketEvent,
    player::{GameProfileComponent, PlayerInfo},
    start_ecs_runner,
};
use azalea_core::data_registry::{DataRegistryWithKey, ResolvableDataRegistry};
use azalea_entity::indexing::{EntityIdIndex, EntityUuidIndex};
use azalea_protocol::{
    address::{ResolvableAddr, ResolvedAddr},
    connect::Proxy,
    packets::{Packet, game::ServerboundGamePacket},
    resolve::ResolveError,
};
use azalea_registry::{DataRegistryKeyRef, identifier::Identifier};
use azalea_world::{Instance, InstanceName, MinecraftEntityId, PartialInstance};
use bevy_app::App;
use bevy_ecs::{
    entity::Entity,
    resource::Resource,
    world::{Mut, World},
};
use parking_lot::RwLock;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{
    entity_ref::EntityRef,
    events::{Event, LocalPlayerEvents},
};

pub mod attack;
pub mod chat;
pub mod client_information;
pub mod entity_query;
pub mod interact;
pub mod inventory;
pub mod mining;
pub mod movement;

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
    ///
    /// You can nearly always use [`Self::component`], [`Self::query_self`],
    /// [`Self::query_entity`], or another one of those related functions to
    /// access the ECS instead.
    pub ecs: Arc<RwLock<World>>,
}

pub struct StartClientOpts {
    pub ecs_lock: Arc<RwLock<World>>,
    pub account: Account,
    pub connect_opts: ConnectOpts,
    pub event_sender: Option<mpsc::UnboundedSender<Event>>,
}

impl StartClientOpts {
    pub fn new(
        account: Account,
        address: ResolvedAddr,
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
                server_proxy: None,
                sessionserver_proxy: None,
            },
            event_sender,
        }
    }

    /// Configure the SOCKS5 proxy used for connecting to the server and for
    /// authenticating with Mojang.
    ///
    /// To configure these separately, for example to only use the proxy for the
    /// Minecraft server and not for authentication, you may use
    /// [`Self::server_proxy`] and [`Self::sessionserver_proxy`] individually.
    pub fn proxy(self, proxy: Proxy) -> Self {
        self.server_proxy(proxy.clone()).sessionserver_proxy(proxy)
    }
    /// Configure the SOCKS5 proxy that will be used for connecting to the
    /// Minecraft server.
    ///
    /// To avoid errors on servers with the "prevent-proxy-connections" option
    /// set, you should usually use [`Self::proxy`] instead.
    ///
    /// Also see [`Self::sessionserver_proxy`].
    pub fn server_proxy(mut self, proxy: Proxy) -> Self {
        self.connect_opts.server_proxy = Some(proxy);
        self
    }
    /// Configure the SOCKS5 proxy that this bot will use for authenticating the
    /// server join with Mojang's API.
    ///
    /// Also see [`Self::proxy`] and [`Self::server_proxy`].
    pub fn sessionserver_proxy(mut self, proxy: Proxy) -> Self {
        self.connect_opts.sessionserver_proxy = Some(proxy);
        self
    }
}

impl Client {
    /// Create a new client from the given [`GameProfile`], ECS Entity, ECS
    /// World, and schedule runner function.
    /// You should only use this if you want to change these fields from the
    /// defaults, otherwise use [`Client::join`].
    pub fn new(entity: Entity, ecs: Arc<RwLock<World>>) -> Self {
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
    /// use azalea::{Account, Client};
    ///
    /// #[tokio::main]
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
        address: impl ResolvableAddr,
    ) -> Result<(Self, mpsc::UnboundedReceiver<Event>), ResolveError> {
        let address = address.resolve().await?;
        let (tx, rx) = mpsc::unbounded_channel();

        let client = Self::start_client(StartClientOpts::new(account, address, Some(tx))).await;
        Ok((client, rx))
    }

    pub async fn join_with_proxy(
        account: Account,
        address: impl ResolvableAddr,
        proxy: Proxy,
    ) -> Result<(Self, mpsc::UnboundedReceiver<Event>), ResolveError> {
        let address = address.resolve().await?;
        let (tx, rx) = mpsc::unbounded_channel();

        let client =
            Self::start_client(StartClientOpts::new(account, address, Some(tx)).proxy(proxy)).await;
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

        ecs_lock.write().write_message(StartJoinServerEvent {
            account,
            connect_opts,
            start_join_callback_tx: Some(start_join_callback_tx),
        });

        let entity = start_join_callback_rx.recv().await.expect(
            "start_join_callback should not be dropped before sending a message, this is a bug in Azalea",
        );

        if let Some(event_sender) = event_sender {
            ecs_lock
                .write()
                .entity_mut(entity)
                .insert(LocalPlayerEvents(event_sender));
        }

        Client::new(entity, ecs_lock)
    }

    /// Write a packet directly to the server.
    pub fn write_packet(&self, packet: impl Packet<ServerboundGamePacket>) {
        let packet = packet.into_variant();
        self.ecs
            .write()
            .commands()
            .trigger(SendGamePacketEvent::new(self.entity, packet));
    }

    /// Disconnect this client from the server by ending all tasks.
    ///
    /// The OwnedReadHalf for the TCP connection is in one of the tasks, so it
    /// automatically closes the connection when that's dropped.
    pub fn disconnect(&self) {
        self.ecs.write().write_message(DisconnectEvent {
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

    /// Get a resource from the ECS. This will clone the resource and return it.
    pub fn resource<T: Resource + Clone>(&self) -> T {
        self.ecs.read().resource::<T>().clone()
    }

    /// Get a required ECS resource and call the given function with it.
    pub fn map_resource<T: Resource, R>(&self, f: impl FnOnce(&T) -> R) -> R {
        let ecs = self.ecs.read();
        let value = ecs.resource::<T>();
        f(value)
    }

    /// Get an optional ECS resource and call the given function with it.
    pub fn map_get_resource<T: Resource, R>(&self, f: impl FnOnce(Option<&T>) -> R) -> R {
        let ecs = self.ecs.read();
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
    /// # fn example(client: &azalea::Client) {
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

    /// Returns the client as an [`EntityRef`], allowing you to treat it as any
    /// other entity.
    pub fn entity(&self) -> EntityRef {
        self.entity_ref_for(self.entity)
    }

    /// Create an [`EntityRef`] for the given ECS entity.
    pub fn entity_ref_for(&self, entity: Entity) -> EntityRef {
        EntityRef::new(self.clone(), entity)
    }
}

impl Client {
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

    /// Get a map of player UUIDs to their information in the tab list.
    ///
    /// This is a shortcut for `*bot.component::<TabList>()`.
    pub fn tab_list(&self) -> HashMap<Uuid, PlayerInfo> {
        (**self.component::<TabList>()).clone()
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
        (**self.component::<GameProfileComponent>()).clone()
    }

    /// Returns the [`Account`] for our client.
    pub fn account(&self) -> Account {
        self.component::<Account>().clone()
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

    /// Get an [`Entity`] in the world by its Minecraft UUID, if it's within
    /// render distance.
    ///
    /// Also see [`Self::entity_by_uuid`] and
    /// [`Self::entity_id_by_minecraft_id`].
    pub fn entity_id_by_uuid(&self, uuid: Uuid) -> Option<Entity> {
        self.map_resource::<EntityUuidIndex, _>(|entity_uuid_index| entity_uuid_index.get(&uuid))
    }
    /// Get an [`EntityRef`] in the world by its Minecraft UUID, if it's within
    /// render distance.
    ///
    /// Also see [`Self::entity_id_by_uuid`].
    pub fn entity_by_uuid(&self, uuid: Uuid) -> Option<EntityRef> {
        self.entity_id_by_uuid(uuid).map(|e| self.entity_ref_for(e))
    }

    /// Get an [`Entity`] in the world by its [`MinecraftEntityId`].
    ///
    /// Also see [`Self::entity_by_uuid`] and [`Self::entity_id_by_uuid`].
    pub fn entity_id_by_minecraft_id(&self, id: MinecraftEntityId) -> Option<Entity> {
        self.query_self::<&EntityIdIndex, _>(|entity_id_index| {
            entity_id_index.get_by_minecraft_entity(id)
        })
    }
    /// Get an [`EntityRef`] in the world by its [`MinecraftEntityId`].
    ///
    /// Also see [`Self::entity_id_by_uuid`].
    pub fn entity_by_minecraft_id(&self, id: MinecraftEntityId) -> Option<EntityRef> {
        self.entity_id_by_minecraft_id(id)
            .map(|e| EntityRef::new(self.clone(), e))
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
    /// [`Enchantment`]: azalea_registry::data::Enchantment
    pub fn resolve_registry_name(
        &self,
        registry: &impl ResolvableDataRegistry,
    ) -> Option<Identifier> {
        self.with_registry_holder(|registries| registry.key(registries).map(|r| r.into_ident()))
    }
    /// Resolve the given registry to its name and data and call the given
    /// function with it.
    ///
    /// This is necessary for data-driven registries like [`Enchantment`].
    ///
    /// If you just want the value name, use [`Self::resolve_registry_name`]
    /// instead.
    ///
    /// [`Enchantment`]: azalea_registry::data::Enchantment
    pub fn with_resolved_registry<R: ResolvableDataRegistry, Ret>(
        &self,
        registry: R,
        f: impl FnOnce(&Identifier, &R::DeserializesTo) -> Ret,
    ) -> Option<Ret> {
        self.with_registry_holder(|registries| {
            registry
                .resolve(registries)
                .map(|(name, data)| f(name, data))
        })
    }
}
