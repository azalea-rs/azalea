mod bot;
pub mod prelude;

use std::{
    future::Future,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
pub use azalea_client::*;
use azalea_protocol::ServerAddress;
use thiserror::Error;

/// Plugins can keep their own personal state, listen to events, and add new functions to Client.
#[async_trait]
pub trait Plugin: Send + Sync {
    async fn handle(&self, bot: Client, event: Arc<Event>);
}

pub struct Options<S, A, Fut>
where
    A: TryInto<ServerAddress>,
    Fut: Future<Output = Result<(), anyhow::Error>>,
{
    pub address: A,
    pub account: Account,
    pub plugins: Vec<&'static dyn Plugin>,
    pub state: Arc<Mutex<S>>,
    pub handle: Box<dyn Fn(Client, Arc<Event>, Arc<Mutex<S>>) -> Fut + Send + Sync>,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid address")]
    InvalidAddress,
}

/// ```no_run
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

    let (bot, mut rx) = options.account.join(&address).await.unwrap();

    let state = options.state;

    while let Some(event) = rx.recv().await {
        // we put it into an Arc so it's cheaper to clone
        let event = Arc::new(event);

        for &plugin in &options.plugins {
            tokio::spawn(plugin.handle(bot.clone(), event.clone()));
        }

        tokio::spawn((*options.handle)(bot.clone(), event.clone(), state.clone()));
    }

    Ok(())
}
