//! Azalea is a framework for creating Minecraft bots.
//!
//! Internally, it's just a wrapper over [`azalea_client`], adding useful
//! functions for making bots. Because of this, lots of the documentation will
//! refer to `azalea_client`. You can just replace these with `azalea` in your
//! code, since everything from azalea_client is re-exported in azalea.
//!
//! # Installation
//!
//! First, install Rust nightly with `rustup install nightly` and `rustup
//! default nightly`.
//!
//! Then, add one of the following lines to your Cargo.toml.\
//! Latest bleeding-edge version:
//! `azalea = { git="https://github.com/mat-1/Cargo.toml" }`
//! Latest "stable" release:
//! `azalea = "0.3"`
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
//!         plugins: plugins![],
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

pub use azalea_client::*;
pub use azalea_core::{BlockPos, Vec3};
use azalea_protocol::ServerAddress;
use std::{future::Future, sync::Arc};
use thiserror::Error;

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
    ///
    /// [`ServerAddress`]: azalea_protocol::ServerAddress
    pub address: A,
    /// The account that's going to join the server.
    pub account: Account,
    /// The plugins that are going to be used. Plugins are external crates that
    /// add extra functionality to Azalea. You should use the [`plugins`] macro
    /// for this field.
    ///
    /// ```rust,no_run
    /// plugins![azalea_pathfinder::Plugin::default()]
    /// ```
    pub plugins: Plugins,
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// use azalea::prelude::*;
    ///
    /// async fn handle(bot: Client, event: Event, state: State) -> anyhow::Result<()> {
    ///     Ok(())
    /// }
    /// ```
    pub handle: HandleFn<Fut, S>,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid address")]
    InvalidAddress,
    #[error("Join error: {0}")]
    Join(#[from] azalea_client::JoinError),
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
///     plugins: plugins![azalea_pathfinder::Plugin::default()],
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

    let (mut bot, mut rx) = Client::join(&options.account, address).await?;

    bot.plugins = Arc::new(options.plugins);

    let state = options.state;
    let bot_plugin = bot::Plugin::default();

    while let Some(event) = rx.recv().await {
        let cloned_plugins = (*bot.plugins).clone();
        for plugin in cloned_plugins.into_iter() {
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


/// A helper macro that generates a [`Plugins`] struct from a list of objects
/// that implement [`Plugin`].
///
/// ```rust,no_run
/// plugins![azalea_pathfinder::Plugin::default()];
/// ```
#[macro_export]
macro_rules! plugins {
    ($($plugin:expr),*) => {
        {
            let mut plugins = azalea_client::Plugins::new();
            $(
                plugins.add($plugin);
            )*
            plugins
        }
    };
}
