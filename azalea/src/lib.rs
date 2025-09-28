#![doc = include_str!("../README.md")]
#![feature(type_changing_struct_update)]

pub mod accept_resource_packs;
pub mod auto_respawn;
pub mod auto_tool;
pub mod bot;
pub mod container;
pub mod nearest_entity;
pub mod pathfinder;
pub mod prelude;
pub mod swarm;

use std::{net::SocketAddr, time::Duration};

use app::Plugins;
pub use azalea_auth as auth;
pub use azalea_block as blocks;
pub use azalea_brigadier as brigadier;
pub use azalea_buf as buf;
pub use azalea_chat::FormattedText;
pub use azalea_client::*;
pub use azalea_core as core;
// these are re-exported on this level because they're very common
pub use azalea_core::{
    position::{BlockPos, Vec3},
    resource_location::ResourceLocation,
};
pub use azalea_entity as entity;
pub use azalea_physics as physics;
pub use azalea_protocol as protocol;
pub use azalea_registry as registry;
pub use azalea_world as world;
pub use bevy_app as app;
use bevy_app::AppExit;
pub use bevy_ecs as ecs;
use ecs::component::Component;
use futures::{Future, future::BoxFuture};
use protocol::{ServerAddress, connect::Proxy, resolver::ResolverError};
use swarm::SwarmBuilder;
use thiserror::Error;

use crate::bot::DefaultBotPlugins;

pub type BoxHandleFn<S, R> =
    Box<dyn Fn(Client, azalea_client::Event, S) -> BoxFuture<'static, R> + Send>;
pub type HandleFn<S, Fut> = fn(Client, azalea_client::Event, S) -> Fut;

/// An error related to resolving the server address when starting a client.
#[derive(Error, Debug)]
pub enum StartError {
    #[error("Invalid address")]
    InvalidAddress,
    #[error(transparent)]
    ResolveAddress(#[from] ResolverError),
}

/// A builder for creating new [`Client`]s. This is the recommended way of
/// making Azalea bots.
///
/// ```no_run
/// # use azalea::prelude::*;
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() {
/// ClientBuilder::new()
///     .set_handler(handle)
///     .start(Account::offline("bot"), "localhost")
///     .await;
/// # }
/// # #[derive(Component, Clone, Default)]
/// # pub struct State;
/// # async fn handle(mut bot: Client, event: Event, state: State) -> anyhow::Result<()> {
/// #     Ok(())
/// # }
/// ```
pub struct ClientBuilder<S, R>
where
    S: Default + Send + Sync + Clone + Component + 'static,
    R: Send + 'static,
    Self: Send,
{
    /// Internally, ClientBuilder is just a wrapper over SwarmBuilder since it's
    /// technically just a subset of it so we can avoid duplicating code this
    /// way.
    swarm: SwarmBuilder<S, swarm::NoSwarmState, R, ()>,
}
impl ClientBuilder<NoState, ()> {
    /// Start building a client that can join the world.
    #[must_use]
    pub fn new() -> Self {
        Self::new_without_plugins()
            .add_plugins(DefaultPlugins)
            .add_plugins(DefaultBotPlugins)
    }

    /// [`Self::new`] but without adding the plugins by default.
    ///
    /// This is useful if you want to disable a default plugin. This also exists
    /// for swarms, see [`SwarmBuilder::new_without_plugins`].
    ///
    /// Note that you can also disable `LogPlugin` by disabling the `log`
    /// feature.
    ///
    /// You **must** add [`DefaultPlugins`] and [`DefaultBotPlugins`] to this.
    ///
    /// ```
    /// # use azalea::prelude::*;
    /// use azalea::app::PluginGroup;
    ///
    /// let client_builder = ClientBuilder::new_without_plugins()
    ///     .add_plugins(
    ///         azalea::DefaultPlugins
    ///             .build()
    ///             .disable::<azalea::chat_signing::ChatSigningPlugin>(),
    ///     )
    ///     .add_plugins(azalea::bot::DefaultBotPlugins);
    /// # client_builder.set_handler(handle);
    /// # #[derive(Component, Clone, Default)]
    /// # pub struct State;
    /// # async fn handle(mut bot: Client, event: Event, state: State) -> anyhow::Result<()> {
    /// #     Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn new_without_plugins() -> Self {
        Self {
            swarm: SwarmBuilder::new_without_plugins(),
        }
    }

    /// Set the function that's called every time a bot receives an [`Event`].
    /// This is the way to handle normal per-bot events.
    ///
    /// Currently, you can have up to one client handler.
    ///
    /// Note that if you're creating clients directly from the ECS using
    /// [`StartJoinServerEvent`] and the client wasn't already in the ECS, then
    /// the handler function won't be called for that client. This shouldn't be
    /// a concern for most bots, though.
    ///
    /// ```
    /// # use azalea::prelude::*;
    /// # let client_builder = azalea::ClientBuilder::new();
    /// client_builder.set_handler(handle);
    ///
    /// # #[derive(Component, Clone, Default)]
    /// # pub struct State;
    /// async fn handle(mut bot: Client, event: Event, state: State) -> anyhow::Result<()> {
    ///     Ok(())
    /// }
    /// ```
    ///
    /// [`StartJoinServerEvent`]: azalea_client::join::StartJoinServerEvent
    #[must_use]
    pub fn set_handler<S, Fut, R>(self, handler: HandleFn<S, Fut>) -> ClientBuilder<S, R>
    where
        S: Default + Send + Sync + Clone + Component + 'static,
        Fut: Future<Output = R> + Send + 'static,
        R: Send + 'static,
    {
        ClientBuilder {
            swarm: self.swarm.set_handler(handler),
        }
    }
}
impl<S, R> ClientBuilder<S, R>
where
    S: Default + Send + Sync + Clone + Component + 'static,
    R: Send + 'static,
{
    /// Set the client state instead of initializing defaults.
    #[must_use]
    pub fn set_state(mut self, state: S) -> Self {
        self.swarm.states = vec![state];
        self
    }
    /// Add a group of plugins to the client.
    ///
    /// See [`Self::new_without_plugins`] to learn how to disable default
    /// plugins.
    #[must_use]
    pub fn add_plugins<M>(mut self, plugins: impl Plugins<M>) -> Self {
        self.swarm = self.swarm.add_plugins(plugins);
        self
    }

    /// Configures the auto-reconnection behavior for our bot.
    ///
    /// If this is `Some`, then it'll set the default reconnection delay for our
    /// bot (how long it'll wait after being kicked before it tries
    /// rejoining). if it's `None`, then auto-reconnecting will be disabled.
    ///
    /// If this function isn't called, then our client will reconnect after
    /// [`DEFAULT_RECONNECT_DELAY`].
    ///
    /// [`DEFAULT_RECONNECT_DELAY`]: azalea_client::auto_reconnect::DEFAULT_RECONNECT_DELAY
    #[must_use]
    pub fn reconnect_after(mut self, delay: impl Into<Option<Duration>>) -> Self {
        self.swarm.reconnect_after = delay.into();
        self
    }

    /// Build this `ClientBuilder` into an actual [`Client`] and join the given
    /// server. If the client can't join, it'll keep retrying forever until it
    /// can.
    ///
    /// The `address` argument can be a `&str`, [`ServerAddress`], or anything
    /// that implements `TryInto<ServerAddress>`.
    ///
    /// # Errors
    ///
    /// This will error if the given address is invalid or couldn't be resolved
    /// to a Minecraft server.
    ///
    /// [`ServerAddress`]: azalea_protocol::ServerAddress
    pub async fn start(
        mut self,
        account: Account,
        address: impl TryInto<ServerAddress>,
    ) -> Result<AppExit, StartError> {
        self.swarm.accounts = vec![(account, JoinOpts::default())];
        if self.swarm.states.is_empty() {
            self.swarm.states = vec![S::default()];
        }
        self.swarm.start(address).await
    }

    /// Do the same as [`Self::start`], but allow passing in custom join
    /// options.
    pub async fn start_with_opts(
        mut self,
        account: Account,
        address: impl TryInto<ServerAddress>,
        opts: JoinOpts,
    ) -> Result<AppExit, StartError> {
        self.swarm.accounts = vec![(account, opts.clone())];
        if self.swarm.states.is_empty() {
            self.swarm.states = vec![S::default()];
        }
        self.swarm.start_with_default_opts(address, opts).await
    }
}
impl Default for ClientBuilder<NoState, ()> {
    fn default() -> Self {
        Self::new()
    }
}

/// A marker that can be used in place of a State in [`ClientBuilder`] or
/// [`SwarmBuilder`]. You probably don't need to use this manually since the
/// compiler will infer it for you.
///
/// [`SwarmBuilder`]: swarm::SwarmBuilder
#[derive(Component, Clone, Default)]
pub struct NoState;

/// Optional settings when adding an account to a swarm or client.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct JoinOpts {
    /// The Socks5 proxy that this bot will use.
    pub proxy: Option<Proxy>,
    /// Override the server address that this specific bot will send in the
    /// handshake packet.
    pub custom_address: Option<ServerAddress>,
    /// Override the socket address that this specific bot will use to connect
    /// to the server.
    pub custom_resolved_address: Option<SocketAddr>,
}

impl JoinOpts {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, other: &Self) {
        if let Some(proxy) = other.proxy.clone() {
            self.proxy = Some(proxy);
        }
        if let Some(custom_address) = other.custom_address.clone() {
            self.custom_address = Some(custom_address);
        }
        if let Some(custom_resolved_address) = other.custom_resolved_address {
            self.custom_resolved_address = Some(custom_resolved_address);
        }
    }

    /// Set the proxy that this bot will use.
    #[must_use]
    pub fn proxy(mut self, proxy: Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
    /// Set the custom address that this bot will send in the handshake packet.
    #[must_use]
    pub fn custom_address(mut self, custom_address: ServerAddress) -> Self {
        self.custom_address = Some(custom_address);
        self
    }
    /// Set the custom resolved address that this bot will use to connect to the
    /// server.
    #[must_use]
    pub fn custom_resolved_address(mut self, custom_resolved_address: SocketAddr) -> Self {
        self.custom_resolved_address = Some(custom_resolved_address);
        self
    }
}
