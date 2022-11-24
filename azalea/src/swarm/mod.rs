mod chat;
mod plugins;

pub use self::plugins::*;
use crate::{bot, HandleFn};
use azalea_client::{Account, ChatPacket, Client, Event, JoinError, Plugins};
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
use tokio::sync::mpsc::{self, UnboundedSender};

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
/// It's used to make the [`Swarm::add`] function work.
#[derive(Clone)]
pub struct Swarm<S> {
    bot_datas: Arc<Mutex<Vec<(Client, S)>>>,

    resolved_address: SocketAddr,
    address: ServerAddress,
    pub worlds: Arc<RwLock<WeakWorldContainer>>,
    /// Plugins that are set for new bots
    plugins: Plugins,

    bots_tx: UnboundedSender<(Option<Event>, (Client, S))>,
    swarm_tx: UnboundedSender<SwarmEvent>,
}

/// An event about something that doesn't have to do with a single bot.
#[derive(Clone, Debug)]
pub enum SwarmEvent {
    /// All the bots in the swarm have successfully joined the server.
    Login,
    /// The swarm was created. This is only fired once, and it's guaranteed to
    /// be the first event to fire.
    Init,
    /// A bot got disconnected from the server.
    ///
    /// You can implement an auto-reconnect by calling [`Swarm::add`]
    /// with the account from this event.
    Disconnect(Account),
    /// At least one bot received a chat message.
    Chat(ChatPacket),
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
    let address: ServerAddress = match options.address.try_into() {
        Ok(address) => address,
        Err(_) => return Err(SwarmStartError::InvalidAddress),
    };

    // resolve the address
    let resolved_address = resolver::resolve_address(&address).await?;

    let world_container = Arc::new(RwLock::new(WeakWorldContainer::default()));

    let mut plugins = options.plugins;
    let swarm_plugins = options.swarm_plugins;

    // DEFAULT CLIENT PLUGINS
    plugins.add(bot::Plugin);
    plugins.add(crate::pathfinder::Plugin);
    // DEFAULT SWARM PLUGINS

    // we can't modify the swarm plugins after this
    let (bots_tx, mut bots_rx) = mpsc::unbounded_channel();
    let (swarm_tx, mut swarm_rx) = mpsc::unbounded_channel();

    let mut swarm = Swarm {
        bot_datas: Arc::new(Mutex::new(Vec::new())),

        resolved_address,
        address,
        worlds: world_container,
        plugins,

        bots_tx,

        swarm_tx: swarm_tx.clone(),
    };

    {
        // the chat plugin is hacky and needs the swarm to be passed like this
        let (chat_swarm_state, chat_tx) = chat::SwarmState::new(swarm.clone());
        swarm.plugins.add(chat::Plugin {
            swarm_state: chat_swarm_state,
            tx: chat_tx,
        });
    }

    let swarm_plugins = swarm_plugins.build();

    let mut swarm_clone = swarm.clone();
    let join_task = tokio::spawn(async move {
        if let Some(join_delay) = options.join_delay {
            // if there's a join delay, then join one by one
            for (account, state) in options.accounts.iter().zip(options.states) {
                swarm_clone
                    .add_with_exponential_backoff(account, state.clone())
                    .await;
                tokio::time::sleep(join_delay).await;
            }
        } else {
            let swarm_borrow = &swarm_clone;
            join_all(options.accounts.iter().zip(options.states).map(
                async move |(account, state)| -> Result<(), JoinError> {
                    swarm_borrow
                        .clone()
                        .add_with_exponential_backoff(account, state.clone())
                        .await;
                    Ok(())
                },
            ))
            .await;
        }
    });

    let swarm_state = options.swarm_state;
    let mut internal_state = InternalSwarmState::default();

    // Watch swarm_rx and send those events to the plugins and swarm_handle.
    let swarm_clone = swarm.clone();
    let swarm_plugins_clone = swarm_plugins.clone();
    tokio::spawn(async move {
        while let Some(event) = swarm_rx.recv().await {
            for plugin in swarm_plugins_clone.clone().into_iter() {
                tokio::spawn(plugin.handle(event.clone(), swarm_clone.clone()));
            }
            tokio::spawn((options.swarm_handle)(
                swarm_clone.clone(),
                event,
                swarm_state.clone(),
            ));
        }
    });

    // bot events
    while let Some((Some(event), (bot, state))) = bots_rx.recv().await {
        // bot event handling
        let cloned_plugins = (*bot.plugins).clone();
        for plugin in cloned_plugins.into_iter() {
            tokio::spawn(plugin.handle(event.clone(), bot.clone()));
        }

        // swarm event handling
        // remove this #[allow] when more checks are added
        #[allow(clippy::single_match)]
        match &event {
            Event::Login => {
                internal_state.bots_joined += 1;
                if internal_state.bots_joined == swarm.bot_datas.lock().len() {
                    swarm_tx.send(SwarmEvent::Login).unwrap();
                }
            }
            _ => {}
        }

        tokio::spawn((options.handle)(bot, event, state));
    }

    join_task.abort();

    Ok(())
}

impl<S> Swarm<S>
where
    S: Send + Sync + Clone + 'static,
{
    /// Add a new account to the swarm. You can remove it later by calling [`Client::disconnect`].
    pub async fn add(&mut self, account: &Account, state: S) -> Result<Client, JoinError> {
        let conn = Connection::new(&self.resolved_address).await?;
        let (conn, game_profile) = Client::handshake(conn, account, &self.address.clone()).await?;

        // tx is moved to the bot so it can send us events
        // rx is used to receive events from the bot
        let (tx, mut rx) = mpsc::channel(1);
        let mut bot = Client::new(game_profile, conn, Some(self.worlds.clone()));
        tx.send(Event::Init).await.expect("Failed to send event");
        bot.start_tasks(tx);

        bot.plugins = Arc::new(self.plugins.clone().build());

        let cloned_bots_tx = self.bots_tx.clone();
        let cloned_bot = bot.clone();
        let cloned_state = state.clone();
        let owned_account = account.clone();
        let bot_datas = self.bot_datas.clone();
        let swarm_tx = self.swarm_tx.clone();
        // send the init event immediately so it's the first thing we get
        swarm_tx.send(SwarmEvent::Init).unwrap();
        tokio::spawn(async move {
            while let Some(event) = rx.recv().await {
                // we can't handle events here (since we can't copy the handler),
                // they're handled above in start_swarm
                if let Err(e) =
                    cloned_bots_tx.send((Some(event), (cloned_bot.clone(), cloned_state.clone())))
                {
                    error!("Error sending event to swarm: {e}");
                }
            }
            // the bot disconnected, so we remove it from the swarm
            let mut bot_datas = bot_datas.lock();
            let index = bot_datas
                .iter()
                .position(|(b, _)| b.profile.uuid == cloned_bot.profile.uuid)
                .expect("bot disconnected but not found in   swarm");
            bot_datas.remove(index);

            swarm_tx
                .send(SwarmEvent::Disconnect(owned_account))
                .unwrap();
        });

        self.bot_datas.lock().push((bot.clone(), state.clone()));

        Ok(bot)
    }

    /// Add a new account to the swarm, retrying if it couldn't join. This will
    /// run forever until the bot joins or the task is aborted.
    pub async fn add_with_exponential_backoff(&mut self, account: &Account, state: S) -> Client {
        let mut disconnects = 0;
        loop {
            match self.add(account, state.clone()).await {
                Ok(bot) => return bot,
                Err(e) => {
                    disconnects += 1;
                    let delay = (Duration::from_secs(5) * 2u32.pow(disconnects))
                        .min(Duration::from_secs(120));
                    let username = account.username.clone();
                    error!("Error joining {username}: {e}. Waiting {delay:?} and trying again.");
                    tokio::time::sleep(delay).await;
                }
            }
        }
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
