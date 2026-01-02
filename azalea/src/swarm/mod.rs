//! Swarms are a way to conveniently control many bots.
//!
//! See [`Swarm`] for more information.

mod builder;
mod chat;
mod events;
pub mod prelude;

use std::sync::{
    Arc,
    atomic::{self, AtomicBool},
};

use azalea_client::{account::Account, chat::ChatPacket, join::ConnectOpts};
use azalea_entity::LocalEntity;
use azalea_protocol::address::ResolvedAddr;
use azalea_world::InstanceContainer;
use bevy_app::{PluginGroup, PluginGroupBuilder};
use bevy_ecs::prelude::*;
pub use builder::SwarmBuilder;
use futures::future::BoxFuture;
use parking_lot::RwLock;
use tokio::{sync::mpsc, task};
use tracing::{debug, error, warn};

use crate::{Client, JoinOpts, client_impl::StartClientOpts};

/// A swarm is a way to conveniently control many bots at once, while also
/// being able to control bots at an individual level when desired.
///
/// It can safely be cloned, so there should be no need to wrap them in a Mutex.
///
/// Swarms are created from [`SwarmBuilder`].
///
/// Clients can be added to the swarm later via [`Swarm::add`], and can be
/// removed with [`Client::disconnect`].
#[derive(Clone, Resource)]
pub struct Swarm {
    /// A way to directly access the ECS.
    ///
    /// This will not work if called within a system, as the ECS is already
    /// locked.
    #[doc(alias = "ecs_lock")] // former type name
    pub ecs: Arc<RwLock<World>>,

    // the address is public and mutable so plugins can change it
    pub address: Arc<RwLock<ResolvedAddr>>,

    pub instance_container: Arc<RwLock<InstanceContainer>>,

    /// This is used internally to make the client handler function work.
    pub(crate) bots_tx: mpsc::UnboundedSender<(Option<crate::Event>, Client)>,
    /// This is used internally to make the swarm handler function work.
    pub(crate) swarm_tx: mpsc::UnboundedSender<SwarmEvent>,
}

/// An event about something that doesn't have to do with a single bot.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum SwarmEvent {
    /// All the bots in the swarm have successfully joined the server.
    Login,
    /// The swarm was created.
    ///
    /// This is only fired once, and it's guaranteed to be the first event to
    /// fire.
    Init,
    /// A bot got disconnected from the server.
    ///
    /// If you'd like to implement special auto-reconnect behavior beyond what's
    /// built-in, you can disable that with [`SwarmBuilder::reconnect_delay`]
    /// and then call [`Swarm::add_with_opts`] with the account and options
    /// from this event.
    ///
    /// [`SwarmBuilder::reconnect_delay`]: crate::swarm::SwarmBuilder::reconnect_after
    Disconnect(Box<Account>, Box<JoinOpts>),
    /// At least one bot received a chat message.
    Chat(ChatPacket),
}

pub type SwarmHandleFn<SS, Fut> = fn(Swarm, SwarmEvent, SS) -> Fut;
pub type BoxSwarmHandleFn<SS, R> =
    Box<dyn Fn(Swarm, SwarmEvent, SS) -> BoxFuture<'static, R> + Send + Sync>;

/// Make a bot [`Swarm`].
///
/// [`Swarm`]: struct.Swarm.html
///
/// # Examples
/// ```rust,no_run
/// use azalea::{prelude::*, swarm::prelude::*};
/// use std::time::Duration;
///
/// #[derive(Clone, Component, Default)]
/// struct State {}
///
/// #[derive(Clone, Default, Resource)]
/// struct SwarmState {}
///
/// #[tokio::main]
/// async fn main() -> AppExit {
///     let mut accounts = Vec::new();
///     let mut states = Vec::new();
///
///     for i in 0..10 {
///         accounts.push(Account::offline(&format!("bot{i}")));
///         states.push(State::default());
///     }
///
///     SwarmBuilder::new()
///         .add_accounts(accounts.clone())
///         .set_handler(handle)
///         .set_swarm_handler(swarm_handle)
///         .join_delay(Duration::from_millis(1000))
///         .start("localhost")
///         .await
/// }
///
/// async fn handle(bot: Client, event: Event, _state: State) -> anyhow::Result<()> {
///     match &event {
///         _ => {}
///     }
///     Ok(())
/// }
///
/// async fn swarm_handle(
///     mut swarm: Swarm,
///     event: SwarmEvent,
///     _state: SwarmState,
/// ) -> anyhow::Result<()> {
///     match &event {
///         SwarmEvent::Chat(m) => {
///             println!("{}", m.message().to_ansi());
///         }
///         _ => {}
///     }
///     Ok(())
/// }
impl Swarm {
    /// Add a new account to the swarm.
    ///
    /// You can remove it later by calling [`Client::disconnect`].
    ///
    /// # Errors
    ///
    /// Returns an error if the server's address could not be resolved.
    pub async fn add<S: Component + Clone>(&self, account: &Account, state: S) -> Client {
        self.add_with_opts(account, state, &JoinOpts::default())
            .await
    }
    /// Add a new account to the swarm, using custom options.
    ///
    /// This is useful if you want bots in the same swarm to connect to
    /// different addresses. Usually you'll just want [`Self::add`] though.
    ///
    /// # Errors
    ///
    /// Returns an error if the server's address could not be resolved.
    pub async fn add_with_opts<S: Component + Clone>(
        &self,
        account: &Account,
        state: S,
        join_opts: &JoinOpts,
    ) -> Client {
        debug!(
            "add_with_opts called for account {} with opts {join_opts:?}",
            account.username()
        );

        let mut address = self.address.read().clone();
        if let Some(custom_server_addr) = join_opts.custom_server_addr.clone() {
            address.server = custom_server_addr;
        }
        if let Some(custom_socket_addr) = join_opts.custom_socket_addr {
            address.socket = custom_socket_addr;
        }
        let server_proxy = join_opts.server_proxy.clone();
        let sessionserver_proxy = join_opts.sessionserver_proxy.clone();

        let (tx, rx) = mpsc::unbounded_channel();

        let client = Client::start_client(StartClientOpts {
            ecs_lock: self.ecs.clone(),
            account: account.clone(),
            connect_opts: ConnectOpts {
                address,
                server_proxy,
                sessionserver_proxy,
            },
            event_sender: Some(tx),
        })
        .await;
        // add the state to the client
        {
            let mut ecs = self.ecs.write();
            ecs.entity_mut(client.entity).insert(state);
        }

        let cloned_bot = client.clone();
        let swarm_tx = self.swarm_tx.clone();
        let bots_tx = self.bots_tx.clone();

        let join_opts = join_opts.clone();
        task::spawn_local(Self::event_copying_task(
            rx, swarm_tx, bots_tx, cloned_bot, join_opts,
        ));

        client
    }

    /// Copy the events from a client's receiver into bots_tx, until the bot is
    /// removed from the ECS.
    async fn event_copying_task(
        mut rx: mpsc::UnboundedReceiver<crate::Event>,
        swarm_tx: mpsc::UnboundedSender<SwarmEvent>,
        bots_tx: mpsc::UnboundedSender<(Option<crate::Event>, Client)>,
        bot: Client,
        join_opts: JoinOpts,
    ) {
        while let Some(event) = rx.recv().await {
            if rx.len() > 1_000 {
                static WARNED_1_000: AtomicBool = AtomicBool::new(false);
                if !WARNED_1_000.swap(true, atomic::Ordering::Relaxed) {
                    warn!(
                        "The client's Event channel has more than 1,000 items! If you don't need it, consider disabling the `packet-event` feature for `azalea`."
                    )
                }

                if rx.len() > 10_000 {
                    static WARNED_10_000: AtomicBool = AtomicBool::new(false);
                    if !WARNED_10_000.swap(true, atomic::Ordering::Relaxed) {
                        warn!("The client's Event channel has more than 10,000 items!!")
                    }

                    if rx.len() > 100_000 {
                        static WARNED_100_000: AtomicBool = AtomicBool::new(false);
                        if !WARNED_100_000.swap(true, atomic::Ordering::Relaxed) {
                            warn!("The client's Event channel has more than 100,000 items!!!")
                        }

                        if rx.len() > 1_000_000 {
                            static WARNED_1_000_000: AtomicBool = AtomicBool::new(false);
                            if !WARNED_1_000_000.swap(true, atomic::Ordering::Relaxed) {
                                warn!(
                                    "The client's Event channel has more than 1,000,000 items!!!! your code is almost certainly leaking memory"
                                )
                            }
                        }
                    }
                }
            }

            if let crate::Event::Disconnect(_) = event {
                debug!(
                    "Sending SwarmEvent::Disconnect due to receiving an Event::Disconnect from client {}",
                    bot.entity
                );
                let account = bot.account();
                swarm_tx
                    .send(SwarmEvent::Disconnect(
                        Box::new(account),
                        Box::new(join_opts.clone()),
                    ))
                    .unwrap();
            }

            // we can't handle events here (since we can't copy the handler),
            // they're handled above in SwarmBuilder::start
            if let Err(e) = bots_tx.send((Some(event), bot.clone())) {
                error!(
                    "Error sending event to swarm, aborting event_copying_task for {}: {e}",
                    bot.entity
                );
                break;
            }
        }
        debug!(
            "client sender ended for {}, this won't trigger SwarmEvent::Disconnect unless the client already sent its own disconnect event",
            bot.entity
        );
    }

    /// Get an array of ECS [`Entity`]s for all [`LocalEntity`]s in our world.
    /// This will include clients that were disconnected without being removed
    /// from the ECS.
    ///
    /// [`LocalEntity`]: azalea_entity::LocalEntity
    pub fn client_entities(&self) -> Box<[Entity]> {
        let mut ecs = self.ecs.write();
        let mut query = ecs.query_filtered::<Entity, With<LocalEntity>>();
        query.iter(&ecs).collect::<Box<[Entity]>>()
    }
}

impl IntoIterator for Swarm {
    type Item = Client;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    /// Iterate over the bots in this swarm.
    ///
    /// ```rust,no_run
    /// # use azalea::{prelude::*, swarm::prelude::*};
    /// #[derive(Clone, Component)]
    /// # pub struct State;
    /// # fn example(swarm: Swarm) {
    /// for bot in swarm {
    ///     let state = bot.component::<State>();
    ///     // ...
    /// }
    /// # }
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        let client_entities = self.client_entities();

        client_entities
            .into_iter()
            .map(|entity| Client::new(entity, self.ecs.clone()))
            .collect::<Box<[Client]>>()
            .into_iter()
    }
}

/// This plugin group will add all the default plugins necessary for swarms to
/// work.
pub struct DefaultSwarmPlugins;

impl PluginGroup for DefaultSwarmPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(chat::SwarmChatPlugin)
            .add(events::SwarmPlugin)
    }
}

/// A marker that can be used in place of a SwarmState in [`SwarmBuilder`].
///
/// You probably don't need to use this manually since the compiler will infer
/// it for you.
#[derive(Clone, Default, Resource)]
pub struct NoSwarmState;
