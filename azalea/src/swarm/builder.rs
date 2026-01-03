use std::{
    collections::{HashMap, hash_map},
    mem,
    sync::{
        Arc,
        atomic::{self, AtomicBool},
    },
    time::Duration,
};

use azalea_client::{DefaultPlugins, account::Account, start_ecs_runner};
use azalea_protocol::address::{ResolvableAddr, ResolvedAddr};
use azalea_world::InstanceContainer;
use bevy_app::{App, AppExit, Plugins, SubApp};
use bevy_ecs::{component::Component, resource::Resource};
use futures::future::join_all;
use parking_lot::RwLock;
use tokio::{sync::mpsc, task};
use tracing::{debug, error, warn};

use crate::{
    BoxHandleFn, HandleFn, JoinOpts, NoState,
    auto_reconnect::{AutoReconnectDelay, DEFAULT_RECONNECT_DELAY},
    bot::DefaultBotPlugins,
    swarm::{
        BoxSwarmHandleFn, DefaultSwarmPlugins, NoSwarmState, Swarm, SwarmEvent, SwarmHandleFn,
    },
};

/// Create a new [`Swarm`].
///
/// The generics of this struct stand for the following:
/// - S: State
/// - SS: Swarm State
/// - R: Return type of the handler
/// - SR: Return type of the swarm handler
///
/// You shouldn't have to manually set them though, they'll be inferred for you.
pub struct SwarmBuilder<S, SS, R, SR>
where
    S: Send + Sync + Clone + Component + 'static,
    SS: Default + Send + Sync + Clone + Resource + 'static,
    Self: Send,
{
    // SubApp is used instead of App to make it Send
    pub(crate) app: SubApp,
    /// The accounts and proxies that are going to join the server.
    pub(crate) accounts: Vec<(Account, JoinOpts)>,
    /// The individual bot states.
    ///
    /// This must be the same length as `accounts`, since each bot gets one
    /// state.
    pub(crate) states: Vec<S>,
    /// The state for the overall swarm.
    pub(crate) swarm_state: SS,
    /// The function that's called every time a bot receives an [`Event`].
    pub(crate) handler: Option<BoxHandleFn<S, R>>,
    /// The function that's called every time the swarm receives a
    /// [`SwarmEvent`].
    pub(crate) swarm_handler: Option<BoxSwarmHandleFn<SS, SR>>,

    /// How long we should wait between each bot joining the server.
    ///
    /// If this is None, every bot will connect at the same time. None is
    /// different than a duration of 0, since if a duration is present the
    /// bots will wait for the previous one to be ready.
    pub(crate) join_delay: Option<Duration>,

    /// The default reconnection delay for our bots.
    ///
    /// This will change the value of the [`AutoReconnectDelay`] resource.
    pub(crate) reconnect_after: Option<Duration>,
}
impl SwarmBuilder<NoState, NoSwarmState, (), ()> {
    /// Start creating the swarm.
    #[must_use]
    pub fn new() -> Self {
        Self::new_without_plugins()
            .add_plugins(DefaultPlugins)
            .add_plugins(DefaultBotPlugins)
            .add_plugins(DefaultSwarmPlugins)
    }

    /// [`Self::new`] but without adding the plugins by default.
    ///
    /// This is useful if you want to disable a default plugin. This also exists
    /// for `ClientBuilder`, see [`ClientBuilder::new_without_plugins`].
    ///
    /// You **must** add [`DefaultPlugins`], [`DefaultBotPlugins`], and
    /// [`DefaultSwarmPlugins`] to this.
    ///
    /// ```
    /// # use azalea::{prelude::*, swarm::prelude::*};
    /// use azalea::app::PluginGroup;
    ///
    /// let swarm_builder = SwarmBuilder::new_without_plugins()
    ///     .add_plugins(azalea::DefaultPlugins.build().disable::<azalea::chat_signing::ChatSigningPlugin>())
    ///     .add_plugins(azalea::bot::DefaultBotPlugins)
    ///     .add_plugins(azalea::swarm::DefaultSwarmPlugins);
    /// # swarm_builder.set_handler(handle).set_swarm_handler(swarm_handle);
    /// # #[derive(Clone, Component, Default, Resource)]
    /// # pub struct State;
    /// # async fn handle(mut bot: Client, event: Event, state: State) -> anyhow::Result<()> {
    /// #     Ok(())
    /// # }
    /// # async fn swarm_handle(swarm: Swarm, event: SwarmEvent, state: State) -> anyhow::Result<()> {
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// [`ClientBuilder::new_without_plugins`]: crate::ClientBuilder::new_without_plugins
    #[must_use]
    pub fn new_without_plugins() -> Self {
        SwarmBuilder {
            // we create the app here so plugins can add onto it.
            // the schedules won't run until [`Self::start`] is called.

            // `App::new()` is used instead of `SubApp::new()` so the necessary resources are
            // initialized
            app: mem::take(App::new().main_mut()),
            accounts: Vec::new(),
            states: Vec::new(),
            swarm_state: NoSwarmState,
            handler: None,
            swarm_handler: None,
            join_delay: None,
            reconnect_after: Some(DEFAULT_RECONNECT_DELAY),
        }
    }
}

impl<SS, SR> SwarmBuilder<NoState, SS, (), SR>
where
    SS: Default + Send + Sync + Clone + Resource + 'static,
{
    /// Set the function that's called every time a bot receives an
    /// [`Event`](crate::Event). This is the intended way to handle
    /// normal per-bot events.
    ///
    /// Currently you can have up to one handler.
    ///
    /// Note that if you're creating clients directly from the ECS using
    /// [`StartJoinServerEvent`] and the client wasn't already in the ECS, then
    /// the handler function won't be called for that client. This also applies
    /// to [`SwarmBuilder::set_swarm_handler`]. This shouldn't be a concern for
    /// most bots, though.
    ///
    /// ```
    /// # use azalea::{prelude::*, swarm::prelude::*};
    /// # let swarm_builder = SwarmBuilder::new().set_swarm_handler(swarm_handle);
    /// swarm_builder.set_handler(handle);
    ///
    /// #[derive(Clone, Component, Default)]
    /// struct State {}
    /// async fn handle(mut bot: Client, event: Event, state: State) -> anyhow::Result<()> {
    ///     Ok(())
    /// }
    ///
    /// # #[derive(Clone, Default, Resource)]
    /// # struct SwarmState {}
    /// # async fn swarm_handle(
    /// #     mut swarm: Swarm,
    /// #     event: SwarmEvent,
    /// #     state: SwarmState,
    /// # ) -> anyhow::Result<()> {
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// [`StartJoinServerEvent`]: azalea_client::join::StartJoinServerEvent
    #[must_use]
    pub fn set_handler<S, Fut, R>(self, handler: HandleFn<S, Fut>) -> SwarmBuilder<S, SS, R, SR>
    where
        Fut: Future<Output = R> + Send + 'static,
        S: Send + Sync + Clone + Component + Default + 'static,
    {
        SwarmBuilder {
            handler: Some(Box::new(move |bot, event, state: S| {
                Box::pin(handler(bot, event, state))
            })),
            // if we added accounts before the State was set, we've gotta set it to the default now
            states: vec![S::default(); self.accounts.len()],
            app: self.app,
            ..self
        }
    }
}

impl<S, R> SwarmBuilder<S, NoSwarmState, R, ()>
where
    S: Send + Sync + Clone + Component + 'static,
{
    /// Set the function that's called every time the swarm receives a
    /// [`SwarmEvent`]. This is the intended way to handle global swarm events.
    ///
    /// Currently you can have up to one swarm handler.
    ///
    /// Note that if you're creating clients directly from the ECS using
    /// [`StartJoinServerEvent`] and the client wasn't already in the ECS, then
    /// this handler function won't be called for that client. This also applies
    /// to [`SwarmBuilder::set_handler`]. This shouldn't be a concern for
    /// most bots, though.
    ///
    /// ```
    /// # use azalea::{prelude::*, swarm::prelude::*};
    /// # let swarm_builder = SwarmBuilder::new().set_handler(handle);
    /// swarm_builder.set_swarm_handler(swarm_handle);
    ///
    /// # #[derive(Clone, Component, Default)]
    /// # struct State {}
    ///
    /// # async fn handle(mut bot: Client, event: Event, state: State) -> anyhow::Result<()> {
    /// #     Ok(())
    /// # }
    ///
    /// #[derive(Clone, Default, Resource)]
    /// struct SwarmState {}
    /// async fn swarm_handle(
    ///     mut swarm: Swarm,
    ///     event: SwarmEvent,
    ///     state: SwarmState,
    /// ) -> anyhow::Result<()> {
    ///     Ok(())
    /// }
    /// ```
    ///
    /// [`StartJoinServerEvent`]: azalea_client::join::StartJoinServerEvent
    #[must_use]
    pub fn set_swarm_handler<SS, Fut, SR>(
        self,
        handler: SwarmHandleFn<SS, Fut>,
    ) -> SwarmBuilder<S, SS, R, SR>
    where
        SS: Default + Send + Sync + Clone + Resource + 'static,
        Fut: Future<Output = SR> + Send + 'static,
    {
        SwarmBuilder {
            handler: self.handler,
            app: self.app,
            accounts: self.accounts,
            states: self.states,
            swarm_state: SS::default(),
            swarm_handler: Some(Box::new(move |swarm, event, state| {
                Box::pin(handler(swarm, event, state))
            })),
            join_delay: self.join_delay,
            reconnect_after: self.reconnect_after,
        }
    }
}

impl<S, SS, R, SR> SwarmBuilder<S, SS, R, SR>
where
    S: Send + Sync + Clone + Component + 'static,
    SS: Default + Send + Sync + Clone + Resource + 'static,
    R: Send + 'static,
    SR: Send + 'static,
{
    /// Add a vec of [`Account`]s to the swarm.
    ///
    /// Use [`Self::add_account`] to only add one account. If you want the
    /// clients to have different default states, add them one at a time with
    /// [`Self::add_account_with_state`].
    ///
    /// By default, every account will join at the same time, you can add a
    /// delay with [`Self::join_delay`].
    #[must_use]
    pub fn add_accounts(mut self, accounts: Vec<Account>) -> Self
    where
        S: Default,
    {
        for account in accounts {
            self = self.add_account(account);
        }

        self
    }

    /// Add a single new [`Account`] to the swarm.
    ///
    /// Use [`Self::add_accounts`] to add multiple accounts at a time.
    ///
    /// This will make the state for this client be the default, use
    /// [`Self::add_account_with_state`] to avoid that.
    #[must_use]
    pub fn add_account(self, account: Account) -> Self
    where
        S: Default,
    {
        self.add_account_with_state_and_opts(account, S::default(), JoinOpts::default())
    }

    /// Add an account with a custom initial state.
    ///
    /// Use just [`Self::add_account`] to use the `Default` implementation for
    /// the state.
    #[must_use]
    pub fn add_account_with_state(self, account: Account, state: S) -> Self {
        self.add_account_with_state_and_opts(account, state, JoinOpts::default())
    }

    /// Add an account with a custom initial state.
    ///
    /// Use just [`Self::add_account`] to use the `Default` implementation for
    /// the state.
    #[must_use]
    pub fn add_account_with_opts(self, account: Account, opts: JoinOpts) -> Self
    where
        S: Default,
    {
        self.add_account_with_state_and_opts(account, S::default(), opts)
    }

    /// Same as [`Self::add_account_with_state`], but allow passing in custom
    /// join options.
    #[must_use]
    pub fn add_account_with_state_and_opts(
        mut self,
        account: Account,
        state: S,
        join_opts: JoinOpts,
    ) -> Self {
        self.accounts.push((account, join_opts));
        self.states.push(state);
        self
    }

    /// Set the swarm state instead of initializing defaults.
    #[must_use]
    pub fn set_swarm_state(mut self, swarm_state: SS) -> Self {
        self.swarm_state = swarm_state;
        self
    }

    /// Add one or more plugins to this swarm.
    ///
    /// See [`Self::new_without_plugins`] to learn how to disable default
    /// plugins.
    #[must_use]
    pub fn add_plugins<M>(mut self, plugins: impl Plugins<M>) -> Self {
        self.app.add_plugins(plugins);
        self
    }

    /// Set how long we should wait between each bot joining the server.
    ///
    /// By default, every bot will connect at the same time. If you set this
    /// field, however, the bots will wait for the previous one to have
    /// connected and *then* they'll wait the given duration.
    #[must_use]
    pub fn join_delay(mut self, delay: Duration) -> Self {
        self.join_delay = Some(delay);
        self
    }

    /// Configures the auto-reconnection behavior for our bots.
    ///
    /// If this is `Some`, then it'll set the default reconnection delay for our
    /// bots (how long they'll wait after being kicked before they try
    /// rejoining). if it's `None`, then auto-reconnecting will be disabled.
    ///
    /// If this function isn't called, then our clients will reconnect after
    /// [`DEFAULT_RECONNECT_DELAY`].
    #[must_use]
    pub fn reconnect_after(mut self, delay: impl Into<Option<Duration>>) -> Self {
        self.reconnect_after = delay.into();
        self
    }

    /// Build this `SwarmBuilder` into an actual [`Swarm`] and join the given
    /// server.
    ///
    /// The `address` argument can be a `&str`, [`ServerAddr`],
    /// [`ResolvedAddr`], or anything else that implements [`ResolvableAddr`].
    ///
    /// [`ServerAddr`]: azalea_protocol::address::ServerAddr
    pub async fn start(self, address: impl ResolvableAddr) -> AppExit {
        self.start_with_opts(address, JoinOpts::default()).await
    }

    #[doc(hidden)]
    #[deprecated = "renamed to `start_with_opts`."]
    pub async fn start_with_default_opts(
        self,
        address: impl ResolvableAddr,
        default_join_opts: JoinOpts,
    ) -> AppExit {
        self.start_with_opts(address, default_join_opts).await
    }

    /// Do the same as [`Self::start`], but allow passing in default join
    /// options for the bots.
    pub async fn start_with_opts(
        mut self,
        address: impl ResolvableAddr,
        join_opts: JoinOpts,
    ) -> AppExit {
        assert_eq!(
            self.accounts.len(),
            self.states.len(),
            "There must be exactly one state per bot."
        );

        debug!("Starting Azalea {}", env!("CARGO_PKG_VERSION"));

        let address = if let Some(socket_addr) = join_opts.custom_socket_addr {
            let server_addr = if let Some(server_addr) = join_opts
                .custom_server_addr
                .clone()
                .or_else(|| address.clone().server_addr().ok())
            {
                server_addr
            } else {
                error!(
                    "Failed to parse address: {address:?}. If this was expected, consider passing in a `ServerAddr` instead."
                );
                return AppExit::error();
            };

            ResolvedAddr {
                server: server_addr,
                socket: socket_addr,
            }
        } else {
            let Ok(addr) = address.clone().resolve().await else {
                error!(
                    "Failed to resolve address: {address:?}. If this was expected, consider resolving the address earlier with `ResolvableAddr::resolve`."
                );
                return AppExit::error();
            };
            addr
        };

        let instance_container = Arc::new(RwLock::new(InstanceContainer::default()));

        // we can't modify the swarm plugins after this
        let (bots_tx, mut bots_rx) = mpsc::unbounded_channel();
        let (swarm_tx, mut swarm_rx) = mpsc::unbounded_channel();

        swarm_tx.send(SwarmEvent::Init).unwrap();

        let main_schedule_label = self.app.update_schedule.unwrap();

        let local_set = task::LocalSet::new();

        local_set.run_until(async move {
            // start_ecs_runner must be run inside of the LocalSet
            let (ecs_lock, start_running_systems, appexit_rx) = start_ecs_runner(&mut self.app);

            let swarm = Swarm {
                ecs: ecs_lock.clone(),

                address: Arc::new(RwLock::new(address)),
                instance_container,

                bots_tx,

                swarm_tx: swarm_tx.clone(),
            };

            // run the main schedule so the startup systems run
            {
                let mut ecs = ecs_lock.write();
                ecs.insert_resource(swarm.clone());
                ecs.insert_resource(self.swarm_state.clone());
                if let Some(reconnect_after) = self.reconnect_after {
                    ecs.insert_resource(AutoReconnectDelay {
                        delay: reconnect_after,
                    });
                } else {
                    ecs.remove_resource::<AutoReconnectDelay>();
                }
                ecs.run_schedule(main_schedule_label);
                ecs.clear_trackers();
            }

            // only do this after we inserted the Swarm and state resources to avoid errors
            // where Res<Swarm> is inaccessible
            start_running_systems();

            // SwarmBuilder (self) isn't Send so we have to take all the things we need out
            // of it
            let swarm_clone = swarm.clone();
            let join_delay = self.join_delay;
            let accounts = self.accounts.clone();
            let states = self.states.clone();

            task::spawn_local(async move {
                if let Some(join_delay) = join_delay {
                    // if there's a join delay, then join one by one
                    for ((account, bot_join_opts), state) in accounts.iter().zip(states) {
                        let mut join_opts = join_opts.clone();
                        join_opts.update(bot_join_opts);
                        let _ = swarm_clone.add_with_opts(account, state, &join_opts).await;
                        tokio::time::sleep(join_delay).await;
                    }
                } else {
                    // otherwise, join all at once
                    let swarm_borrow = &swarm_clone;
                    join_all(accounts.iter().zip(states).map(
                        |((account, bot_join_opts), state)| async {
                            let mut join_opts = join_opts.clone();
                            join_opts.update(bot_join_opts);
                            let _ = swarm_borrow
                                .clone()
                                .add_with_opts(account, state, &join_opts)
                                .await;
                        },
                    ))
                    .await;
                }

                swarm_tx.send(SwarmEvent::Login).unwrap();
            });

            let swarm_state = self.swarm_state;

            // Watch swarm_rx and send those events to the swarm_handle.
            let swarm_clone = swarm.clone();
            let swarm_handler_task = task::spawn_local(async move {
                while let Some(event) = swarm_rx.recv().await {
                    if let Some(swarm_handler) = &self.swarm_handler {
                        task::spawn_local((swarm_handler)(
                            swarm_clone.clone(),
                            event,
                            swarm_state.clone(),
                        ));
                    }
                }

                unreachable!(
                    "The `Swarm` here contains a sender for the `SwarmEvent`s, so swarm_rx.recv() will never fail"
                );
            });

            // bot events
            let client_handler_task = task::spawn_local(async move {
                while let Some((Some(first_event), first_bot)) = bots_rx.recv().await {
                    if bots_rx.len() > 1_000 {
                        static WARNED: AtomicBool = AtomicBool::new(false);
                        if !WARNED.swap(true, atomic::Ordering::Relaxed) {
                            warn!("the Client Event channel has more than 1000 items!")
                        }
                    }

                    if let Some(handler) = &self.handler {
                        let ecs_mutex = first_bot.ecs.clone();
                        let mut ecs = ecs_mutex.write();
                        let mut query = ecs.query::<Option<&S>>();
                        let Ok(Some(first_bot_state)) = query.get(&ecs, first_bot.entity) else {
                            error!(
                                "the first bot ({} / {}) is missing the required state component! none of the client handler functions will be called.",
                                first_bot.username(),
                                first_bot.entity
                            );
                            continue;
                        };
                        let first_bot_entity = first_bot.entity;
                        let first_bot_state = first_bot_state.clone();

                        task::spawn_local((handler)(first_bot, first_event, first_bot_state.clone()));

                        // this makes it not have to keep locking the ecs
                        let mut states = HashMap::new();
                        states.insert(first_bot_entity, first_bot_state);
                        while let Ok((Some(event), bot)) = bots_rx.try_recv() {
                            let state = match states.entry(bot.entity) {
                                hash_map::Entry::Occupied(e) => e.into_mut(),
                                hash_map::Entry::Vacant(e) => {
                                    let Ok(Some(state)) = query.get(&ecs, bot.entity) else {
                                        error!(
                                            "one of our bots ({} / {}) is missing the required state component! its client handler function will not be called.",
                                            bot.username(),
                                            bot.entity
                                        );
                                        continue;
                                    };
                                    let state = state.clone();
                                    e.insert(state)
                                }
                            };
                            task::spawn_local((handler)(bot, event, state.clone()));
                        }
                    }
                }
            });

            let app_exit = appexit_rx
                .await
                .expect("appexit_tx shouldn't be dropped by the ECS runner before sending");

            swarm_handler_task.abort();
            client_handler_task.abort();

            app_exit
        }).await
    }
}

impl Default for SwarmBuilder<NoState, NoSwarmState, (), ()> {
    fn default() -> Self {
        Self::new()
    }
}
