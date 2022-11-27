use crate::{bot, pathfinder, HandleFn};
use azalea_client::{Account, Client, Plugins};
use azalea_protocol::ServerAddress;
use std::{future::Future, sync::Arc};
use thiserror::Error;

/// A helper macro that generates a [`Plugins`] struct from a list of objects
/// that implement [`Plugin`].
///
/// ```rust,no_run
/// plugins![azalea_pathfinder::Plugin];
/// ```
///
/// [`Plugin`]: crate::Plugin
#[macro_export]
macro_rules! plugins {
    ($($plugin:expr),*) => {
        {
            let mut plugins = azalea::Plugins::new();
            $(
                plugins.add($plugin);
            )*
            plugins
        }
    };
}

/// The options that are passed to [`azalea::start`].
///
/// [`azalea::start`]: crate::start()
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
    /// plugins![azalea_pathfinder::Plugin]
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
pub enum StartError {
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
///     plugins: plugins![azalea_pathfinder::Plugin],
///     handle,
/// }).await;
/// ```
pub async fn start<
    S: Send + Sync + Clone + 'static,
    A: Send + TryInto<ServerAddress>,
    Fut: Future<Output = Result<(), anyhow::Error>> + Send + 'static,
>(
    options: Options<S, A, Fut>,
) -> Result<(), StartError> {
    let address = match options.address.try_into() {
        Ok(address) => address,
        Err(_) => return Err(StartError::InvalidAddress),
    };

    let (mut bot, mut rx) = Client::join(&options.account, address).await?;

    let mut plugins = options.plugins;
    // DEFAULT PLUGINS
    plugins.add(bot::Plugin);
    plugins.add(pathfinder::Plugin);

    bot.plugins = Arc::new(plugins.build());

    let state = options.state;

    while let Some(event) = rx.recv().await {
        let cloned_plugins = (*bot.plugins).clone();
        for plugin in cloned_plugins.into_iter() {
            tokio::spawn(plugin.handle(event.clone(), bot.clone()));
        }

        tokio::spawn((options.handle)(bot.clone(), event.clone(), state.clone()));
    }

    Ok(())
}
