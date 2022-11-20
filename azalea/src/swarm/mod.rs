mod plugins;

pub use self::plugins::*;
use crate::{bot, HandleFn};
use async_trait::async_trait;
use azalea_client::{
    Account, Client, ClientInformation, Event, JoinError, PhysicsState, Player, Plugin, Plugins,
};
use azalea_protocol::{
    connect::{Connection, ConnectionError},
    resolver::{self, ResolverError},
    ServerAddress,
};
use azalea_world::WeakWorldContainer;
use azalea_world::World;
use futures::{
    future::{join_all, select_all, try_join_all},
    FutureExt,
};
use log::error;
use parking_lot::{Mutex, RwLock};
use std::{any::Any, future::Future, net::SocketAddr, sync::Arc, time::Duration};
use thiserror::Error;
use tokio::sync::mpsc::{self, UnboundedReceiver};

/// A helper macro that generates a [`Plugins`] struct from a list of objects
/// that implement [`Plugin`].
///
/// ```rust,no_run
/// plugins![azalea_pathfinder::Plugin::default()];
/// ```
#[macro_export]
macro_rules! swarm_plugins {
    ($($plugin:expr),*) => {
        {
            let mut plugins = azalea::SwarmPlugins::new();
            $(
                plugins.add($plugin);
            )*
            plugins
        }
    };
}

/// A swarm is a way to conveniently control many bots at once, while also
/// being able to control bots at an individual level when desired.
///
/// The `S` type parameter is the type of the state for individual clients.
/// It's used to make the [`Swarm::add_account`] function work.
#[derive(Clone)]
pub struct Swarm<S> {
    bots: Arc<Mutex<Vec<Client>>>,
    receivers: Arc<Mutex<Vec<UnboundedReceiver<Event>>>>,
    states: Arc<Mutex<Vec<S>>>,

    resolved_address: SocketAddr,
    address: ServerAddress,
    world_container: Arc<RwLock<WeakWorldContainer>>,
    /// Plugins that are set for new bots
    plugins: Plugins,
}

/// An event about something that doesn't have to do with a single bot.
#[derive(Clone, Debug)]
pub enum SwarmEvent {
    /// All the bots in the swarm have successfully joined the server.
    Login,
}

pub type SwarmHandleFn<Fut, S, SS> = fn(Swarm<S>, SwarmEvent, SS) -> Fut;

/// The options that are passed to [`azalea::start_swarm`].
///
/// [`azalea::start`]: crate::start_swarm
pub struct SwarmOptions<S, SS, A, Fut, SwarmFut>
where
    A: TryInto<ServerAddress>,
    Fut: Future<Output = Result<(), anyhow::Error>>,
    SwarmFut: Future<Output = Result<(), anyhow::Error>>,
{
    /// The address of the server that we're connecting to. This can be a
    /// `&str`, [`ServerAddress`], or anything that implements
    /// `TryInto<ServerAddress>`.
    ///
    /// [`ServerAddress`]: azalea_protocol::ServerAddress
    pub address: A,
    /// The accounts that are going to join the server.
    pub accounts: Vec<Account>,
    pub plugins: Plugins,
    pub swarm_plugins: SwarmPlugins<S>,
    /// The individual bot states. This must be the same length as `accounts`,
    /// since each bot gets one state.
    pub states: Vec<S>,
    pub swarm_state: SS,
    pub handle: HandleFn<Fut, S>,
    pub swarm_handle: SwarmHandleFn<SwarmFut, S, SS>,

    /// How long we should wait between each bot joining the server. Set to
    /// None to have every bot connect at the same time. None is different than
    /// a duration of 0, since if a duration is present the bots will wait for
    /// the previous one to be ready.
    pub join_delay: Option<std::time::Duration>,
}

#[derive(Error, Debug)]
pub enum SwarmStartError {
    #[error("Invalid address")]
    InvalidAddress,
    #[error(transparent)]
    ResolveAddress(#[from] ResolverError),
    #[error("Join error: {0}")]
    Join(#[from] azalea_client::JoinError),
}

/// Make a bot swarm.
pub async fn start_swarm<
    S: Send + Sync + Clone + 'static,
    SS: Send + Sync + Clone + 'static,
    A: Send + TryInto<ServerAddress>,
    Fut: Future<Output = Result<(), anyhow::Error>> + Send + 'static,
    SwarmFut: Future<Output = Result<(), anyhow::Error>> + Send + 'static,
>(
    options: SwarmOptions<S, SS, A, Fut, SwarmFut>,
) -> Result<(), SwarmStartError> {
    assert_eq!(
        options.accounts.len(),
        options.states.len(),
        "There must be exactly one state per bot."
    );

    // convert the TryInto<ServerAddress> into a ServerAddress
    let address = match options.address.try_into() {
        Ok(address) => address,
        Err(_) => return Err(SwarmStartError::InvalidAddress),
    };

    // resolve the address
    let address: ServerAddress = address.try_into().map_err(|_| JoinError::InvalidAddress)?;
    let resolved_address = resolver::resolve_address(&address).await?;

    let world_container = Arc::new(RwLock::new(WeakWorldContainer::default()));

    let mut plugins = options.plugins;
    plugins.add(bot::Plugin::default());

    let mut swarm = Swarm {
        bots: Arc::new(Mutex::new(Vec::new())),
        receivers: Arc::new(Mutex::new(Vec::new())),
        states: Arc::new(Mutex::new(Vec::new())),
        resolved_address,
        address,
        world_container,
        plugins,
    };

    let mut swarm_clone = swarm.clone();
    let join_task = tokio::spawn(async move {
        if let Some(join_delay) = options.join_delay {
            // if there's a join delay, then join one by one
            for (account, state) in options.accounts.iter().zip(options.states) {
                // exponential backoff
                let mut disconnects = 0;
                while let Err(e) = swarm_clone.add_account(account, state.clone()).await {
                    disconnects += 1;
                    let delay = (Duration::from_secs(5) * 2u32.pow(disconnects))
                        .min(Duration::from_secs(120));
                    error!("Error joining account: {e}. Waiting {delay:?} and trying again.");
                    tokio::time::sleep(delay).await;
                }
                tokio::time::sleep(join_delay).await;
            }
        } else {
            let swarm_borrow = &swarm_clone;
            join_all(options.accounts.iter().zip(options.states).map(
                async move |(account, state)| -> Result<(), JoinError> {
                    // exponential backoff
                    let mut disconnects = 0;
                    while let Err(e) = swarm_borrow
                        .clone()
                        .add_account(account, state.clone())
                        .await
                    {
                        disconnects += 1;
                        let delay = (Duration::from_secs(5) * 2u32.pow(disconnects))
                            .min(Duration::from_secs(120));
                        error!("Error joining account: {e}. Waiting {delay:?} and trying again.");
                        tokio::time::sleep(delay).await;
                    }
                    Ok(())
                },
            ))
            .await;
        }
    });

    let swarm_state = options.swarm_state;
    let mut internal_state = InternalSwarmState::default();

    // Send an event to the swarm_handle function.
    let cloned_swarm = swarm.clone();
    let fire_swarm_event = move |event: SwarmEvent| {
        let cloned_swarm_plugins = options.swarm_plugins.clone();
        for plugin in cloned_swarm_plugins.into_iter() {
            tokio::spawn(plugin.handle(event.clone(), cloned_swarm.clone()));
        }
        tokio::spawn((options.swarm_handle)(
            cloned_swarm.clone(),
            event,
            swarm_state.clone(),
        ));
    };

    // bot events
    while let (Some(event), bot_index) = swarm.bot_recv().await {
        let bot = swarm.bots.lock()[bot_index].clone();
        let bot_state = swarm.states.lock()[bot_index].clone();
        let cloned_plugins = (*bot.plugins).clone();
        for plugin in cloned_plugins.into_iter() {
            tokio::spawn(plugin.handle(event.clone(), bot.clone()));
        }

        // swarm event handling
        match &event {
            Event::Login => {
                internal_state.clients_joined += 1;
                if internal_state.clients_joined == swarm.bots.lock().len() {
                    fire_swarm_event(SwarmEvent::Login);
                }
            }
            _ => {}
        }

        tokio::spawn((options.handle)(bot, event, bot_state));
    }

    let _ = join_task.abort();

    Ok(())
}

impl<S> Swarm<S>
where
    S: Send + Sync + Clone + 'static,
{
    /// Wait for any bot to get an event. We return the event and index (so we
    /// can get the state and bot from that index)
    async fn bot_recv(&mut self) -> (Option<Event>, usize) {
        let mut receivers = self.receivers.lock();
        if receivers.is_empty() {
            // TODO
        }
        let mut futures = Vec::with_capacity(receivers.len());
        for rx in receivers.iter_mut() {
            futures.push(rx.recv().boxed());
        }
        let (event, index, _remaining) = select_all(futures).await;
        (event, index)
    }

    /// Add a new account as part of the swarm.
    pub async fn add_account(&mut self, account: &Account, state: S) -> Result<Client, JoinError> {
        let conn = Connection::new(&self.resolved_address).await?;
        let (conn, game_profile) = Client::handshaw     ake(conn, account, &self.address.clone()).await?;

        let (tx, rx) = mpsc::unbounded_channel();
        let mut client = Client::new(game_profile, conn, Some(self.world_container.clone()));
        tx.send(Event::Initialize).unwrap();
        client.start_tasks(tx);

        client.plugins = Arc::new(self.plugins.clone());

        self.bots.lock().push(client.clone());
        self.receivers.lock().push(rx);
        self.states.lock().push(state.clone());

        Ok(client)
    }
}

#[derive(Default)]
struct InternalSwarmState {
    /// The number of clients connected to the server
    pub clients_joined: usize,
}

impl From<ConnectionError> for SwarmStartError {
    fn from(e: ConnectionError) -> Self {
        SwarmStartError::from(JoinError::from(e))
    }
}
