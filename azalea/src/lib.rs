//! Azalea is a framework for creating Minecraft bots.
//!
//! Internally, it's just a wrapper over [`azalea_client`], adding useful
//! functions for making bots. Because of this, lots of the documentation will
//! refer to `azalea_client`. You can just replace these with `azalea` in your
//! code, since everything from azalea_client is re-exported in azalea.
//!
//! # Examples
//!
//! ```rust,no_run
//! //! A bot that logs chat messages sent in the server to the console.
//!
//! use azalea::prelude::*;
//! use parking_lot::Mutex;
//! use std::sync::Arc;
//!
//! #[tokio::main]
//! async fn main() {
//!     let account = Account::offline("bot");
//!     // or Account::microsoft("example@example.com").await.unwrap();
//!
//!     azalea::start(azalea::Options {
//!         account,
//!         address: "localhost",
//!         state: State::default(),
//!         plugins: vec![],
//!         handle,
//!     })
//!     .await
//!     .unwrap();
//! }
//!
//! #[derive(Default, Clone)]
//! pub struct State {}
//!
//! async fn handle(bot: Client, event: Event, state: State) -> anyhow::Result<()> {
//!     match event {
//!         Event::Chat(m) => {
//!             println!(m.message().to_ansi(None));
//!         }
//!         _ => {}
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! [`azalea_client`]: https://crates.io/crates/azalea-client

mod bot;
pub mod prelude;

use async_trait::async_trait;
pub use azalea_client::*;
use azalea_protocol::ServerAddress;
use std::future::Future;
use thiserror::Error;

/// Plugins can keep their own personal state, listen to events, and add new functions to Client.
#[async_trait]
pub trait Plugin: Send + Sync + PluginClone + 'static {
    async fn handle(self: Box<Self>, event: Event, bot: Client);
}

/// An internal trait that allows Plugin to be cloned.
#[doc(hidden)]
pub trait PluginClone {
    fn clone_box(&self) -> Box<dyn Plugin>;
}
impl<T> PluginClone for T
where
    T: 'static + Plugin + Clone,
{
    fn clone_box(&self) -> Box<dyn Plugin> {
        Box::new(self.clone())
    }
}
impl Clone for Box<dyn Plugin> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

pub type HandleFn<Fut, S> = fn(Client, Event, S) -> Fut;

/// The options that are passed to [`azalea::start`].
///
/// [`azalea::start`]: fn.start.html
pub struct Options<S, A, Fut>
where
    A: TryInto<ServerAddress>,
    Fut: Future<Output = Result<(), anyhow::Error>>,
{
    /// The address of the server that we're connecting to. This can be a
    /// `&str`, [`ServerAddress`], or anything that implements
    /// `TryInto<ServerAddress>`.
    pub address: A,
    /// The account that's going to join the server,
    pub account: Account,
    /// A list of plugins that are going to be used. Plugins are external
    /// crates that add extra functionality to Azalea.
    pub plugins: Vec<Box<dyn Plugin>>,
    /// A struct that contains the data that you want your bot to remember
    /// across events.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parking_lot::Mutex;
    /// use std::sync::Arc;
    ///
    /// #[derive(Default, Clone)]
    /// struct State {
    ///     farming: Arc<Mutex<bool>>,
    /// }
    /// ```
    pub state: S,
    /// The function that's called whenever we get an event.
    pub handle: HandleFn<Fut, S>,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid address")]
    InvalidAddress,
}

/// Join a server and start handling events. This function will run forever until
/// it gets disconnected from the server.
///
/// # Examples
///
/// ```rust,no_run
/// let error = azalea::start(azalea::Options {
///     account,
///     address: "localhost",
///     state: State::default(),
///     plugins: vec![Box::new(autoeat::Plugin::default())],
///     handle,
/// }).await;
/// ```
pub async fn start<
    S: Send + Sync + Clone + 'static,
    A: Send + TryInto<ServerAddress>,
    Fut: Future<Output = Result<(), anyhow::Error>> + Send + 'static,
>(
    options: Options<S, A, Fut>,
) -> Result<(), Error> {
    let address = match options.address.try_into() {
        Ok(address) => address,
        Err(_) => return Err(Error::InvalidAddress),
    };

    let (bot, mut rx) = Client::join(&options.account, address).await.unwrap();

    let state = options.state;
    let bot_plugin = bot::Plugin::default();

    while let Some(event) = rx.recv().await {
        for plugin in &options.plugins {
            let plugin = plugin.clone();
            tokio::spawn(plugin.handle(event.clone(), bot.clone()));
        }

        tokio::spawn(bot::Plugin::handle(
            Box::new(bot_plugin.clone()),
            event.clone(),
            bot.clone(),
        ));
        tokio::spawn((options.handle)(bot.clone(), event.clone(), state.clone()));
    }

    Ok(())
}
