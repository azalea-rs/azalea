use std::time::Duration;

use azalea_client::{DefaultPlugins, account::Account};
use azalea_protocol::address::ResolvableAddr;
use bevy_app::{AppExit, Plugins};
use bevy_ecs::component::Component;

use crate::{
    HandleFn, JoinOpts, NoState,
    bot::DefaultBotPlugins,
    swarm::{self, SwarmBuilder},
};

/// A builder for creating new [`Client`](crate::Client)s. This is the
/// recommended way of making a bot.
///
/// ```no_run
/// # use azalea::prelude::*;
/// # #[tokio::main]
/// # async fn main() {
/// ClientBuilder::new()
///     .set_handler(handle)
///     .start(Account::offline("bot"), "localhost")
///     .await;
/// # }
/// # #[derive(Clone, Component, Default)]
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
    /// # #[derive(Clone, Component, Default)]
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

    /// Set the function that's called every time a bot receives an
    /// [`Event`](crate::Client). This is the way to handle normal per-bot
    /// events.
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
    /// # #[derive(Clone, Component, Default)]
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
    /// [`DEFAULT_RECONNECT_DELAY`]: crate::auto_reconnect::DEFAULT_RECONNECT_DELAY
    #[must_use]
    pub fn reconnect_after(mut self, delay: impl Into<Option<Duration>>) -> Self {
        self.swarm.reconnect_after = delay.into();
        self
    }

    /// Build this `ClientBuilder` into an actual [`Client`](crate::Client) and
    /// join the given server.
    ///
    /// If the client can't join, it'll keep retrying forever until it can.
    ///
    /// The `address` argument can be a `&str`, [`ServerAddr`],
    /// [`ResolvedAddr`], or anything else that implements [`ResolvableAddr`].
    ///
    /// # Errors
    ///
    /// This will error if the given address is invalid or couldn't be resolved
    /// to a Minecraft server.
    ///
    /// [`ServerAddr`]: azalea_protocol::address::ServerAddr
    /// [`ResolvedAddr`]: azalea_protocol::address::ResolvedAddr
    pub async fn start(mut self, account: Account, address: impl ResolvableAddr) -> AppExit {
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
        address: impl ResolvableAddr,
        opts: JoinOpts,
    ) -> AppExit {
        self.swarm.accounts = vec![(account, opts.clone())];
        if self.swarm.states.is_empty() {
            self.swarm.states = vec![S::default()];
        }
        self.swarm.start_with_opts(address, opts).await
    }
}
impl Default for ClientBuilder<NoState, ()> {
    fn default() -> Self {
        Self::new()
    }
}
