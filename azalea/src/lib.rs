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
//!         state: Arc::new(Mutex::new(State::default())),
//!         plugins: vec![],
//!         handle,
//!     })
//!     .await
//!     .unwrap();
//! }
//!
//! pub struct State {}
//!
//! async fn handle(bot: Client, event: Arc<Event>, state: Arc<Mutex<State>>) -> anyhow::Result<()> {
//!     match *event {
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
use parking_lot::Mutex;
use std::{future::Future, sync::Arc};
use thiserror::Error;

/// Plugins can keep their own personal state, listen to events, and add new functions to Client.
#[async_trait]
pub trait Plugin: Send + Sync {
    async fn handle(self: Arc<Self>, event: Arc<Event>, bot: Client);
}

pub type HandleFn<Fut, S> = fn(Client, Arc<Event>, Arc<Mutex<S>>) -> Fut;

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
    pub plugins: Vec<Arc<dyn Plugin>>,
    /// A struct that contains the data that you want your bot to remember
    /// across events.
    pub state: Arc<Mutex<S>>,
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
/// ```rust,no_run
/// azalea::start(azalea::Options {
///     account,
///     address: "localhost",
///     state: Arc::new(Mutex::new(State::default())),
///     plugins: vec![&autoeat::Plugin::default()],
///     handle: Box::new(handle),
/// }).await.unwrap();
/// ```
pub async fn start<
    S: Send + 'static,
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
    let bot_plugin = Arc::new(bot::Plugin::default());

    while let Some(event) = rx.recv().await {
        // we put it into an Arc so it's cheaper to clone

        let event = Arc::new(event);

        for plugin in &options.plugins {
            tokio::spawn(plugin.clone().handle(event.clone(), bot.clone()));
        }

        tokio::spawn(bot::Plugin::handle(
            bot_plugin.clone(),
            event.clone(),
            bot.clone(),
        ));
        tokio::spawn((options.handle)(bot.clone(), event.clone(), state.clone()));
    }

    Ok(())
}
