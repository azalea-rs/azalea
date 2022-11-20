mod plugins;

pub use self::plugins::*;
use crate::{bot, HandleFn};
use azalea_client::{Account, Client, Event, JoinError, Plugins};
use azalea_protocol::{
    connect::{Connection, ConnectionError},
    resolver::{self, ResolverError},
    ServerAddress,
};
use azalea_world::WeakWorldContainer;
use futures::future::join_all;
use log::error;
use parking_lot::{Mutex, RwLock};
use std::{future::Future, net::SocketAddr, sync::Arc, time::Duration};
use thiserror::Error;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

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
/// The `S` type parameter is the type of the state for individual bots.
/// It's used to make the [`Swarm::add_account`] function work.
#[derive(Clone)]
pub struct Swarm<S> {
    bots: Arc<Mutex<Vec<SwarmBotData<S>>>>,

    resolved_address: SocketAddr,
    address: ServerAddress,
    world_container: Arc<RwLock<WeakWorldContainer>>,
    /// Plugins that are set for new bots
    plugins: Plugins,

    /// A single receiver that combines all the receivers of all the bots.
    /// (bot index, event)
    bots_rx: Arc<Mutex<UnboundedReceiver<(Option<Event>, SwarmBotData<S>)>>>,
    bots_tx: Arc<Mutex<UnboundedSender<(Option<Event>, SwarmBotData<S>)>>>,
}

/// The data stored for each bot in the swarm.
#[derive(Clone)]
pub struct SwarmBotData<S> {
    pub bot: Client,
    pub state: S,
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

    let (bots_tx, mut bots_rx) = mpsc::unbounded_channel();

    let mut swarm = Swarm {
        bots: Arc::new(Mutex::new(Vec::new())),

        resolved_address,
        address,
        world_container,
        plugins,

        bots_rx: Arc::new(Mutex::new(bots_rx)),
        bots_tx: Arc::new(Mutex::new(bots_tx)),
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
    while let (Some(event), SwarmBotData { bot, state, .. }) = swarm.bot_recv().await {
        // bot event handling
        let cloned_plugins = (*bot.plugins).clone();
        for plugin in cloned_plugins.into_iter() {
            tokio::spawn(plugin.handle(event.clone(), bot.clone()));
        }

        // swarm event handling
        match &event {
            Event::Login => {
                internal_state.bots_joined += 1;
                if internal_state.bots_joined == swarm.bots.lock().len() {
                    fire_swarm_event(SwarmEvent::Login);
                }
            }
            _ => {}
        }

        tokio::spawn((options.handle)(bot, event, state));
    }

    let _ = join_task.abort();

    Ok(())
}

impl<S> Swarm<S>
where
    S: Send + Sync + Clone + 'static,
{
    /// Wait for any bot to get an event. We return the event and SwarmBotData
    async fn bot_recv(&mut self) -> (Option<Event>, SwarmBotData<S>) {
        let mut bots_rx = self.bots_rx.lock();
        let (event, bot) = bots_rx.recv().await.unwrap();
        (event, bot)
    }

    /// Add a new account as part of the swarm.
    pub async fn add_account(&mut self, account: &Account, state: S) -> Result<Client, JoinError> {
        let conn = Connection::new(&self.resolved_address).await?;
        let (conn, game_profile) = Client::handshake(conn, account, &self.address.clone()).await?;

        // tx is moved to the bot so it can send us events
        // rx is used to receive events from the bot
        let (tx, mut rx) = mpsc::unbounded_channel();
        let mut bot = Client::new(game_profile, conn, Some(self.world_container.clone()));
        tx.send(Event::Initialize).unwrap();
        bot.start_tasks(tx);

        bot.plugins = Arc::new(self.plugins.clone());

        let cloned_bots_tx = self.bots_tx.clone();
        let cloned_bot = bot.clone();
        let cloned_state = state.clone();
        tokio::spawn(async move {
            while let Some(event) = rx.recv().await {
                // we can't handle events here (since we can't copy the handler),
                // they're handled above in start_swarm
                if let Err(e) = cloned_bots_tx.lock().send((
                    Some(event),
                    SwarmBotData {
                        bot: cloned_bot.clone(),
                        state: cloned_state.clone(),
                    },
                )) {
                    error!("Error sending event to swarm: {e}");
                }
            }
        });

        self.bots.lock().push(SwarmBotData {
            bot: bot.clone(),
            state: state.clone(),
        });

        Ok(bot)
    }
}

#[derive(Default)]
struct InternalSwarmState {
    /// The number of bots connected to the server
    pub bots_joined: usize,
}

impl From<ConnectionError> for SwarmStartError {
    fn from(e: ConnectionError) -> Self {
        SwarmStartError::from(JoinError::from(e))
    }
}
