#![doc = include_str!("../README.md")]
#![feature(trait_upcasting)]
#![feature(async_closure)]
#![allow(incomplete_features)]

mod bot;
pub mod pathfinder;
pub mod prelude;
mod swarm;

pub use azalea_block as blocks;
pub use azalea_client::*;
pub use azalea_core::{BlockPos, Vec3};
pub use azalea_protocol as protocol;
pub use azalea_registry::EntityKind;
pub use azalea_world::{entity, World};
use bevy_ecs::prelude::Component;
use futures::Future;
use protocol::ServerAddress;
pub use swarm::*;
use thiserror::Error;

pub type HandleFn<Fut, S> = fn(Client, Event, S) -> Fut;

#[derive(Error, Debug)]
pub enum StartError {
    #[error("Invalid address")]
    InvalidAddress,
    #[error("Join error: {0}")]
    Join(#[from] azalea_client::JoinError),
}

pub struct ClientBuilder<S, Fut>
where
    S: Default + Send + Sync + Clone + 'static,
    Fut: Future<Output = Result<(), anyhow::Error>>,
{
    /// The function that's called every time a bot receives an [`Event`].
    handler: Option<HandleFn<Fut, S>>,
    state: S,
}
impl<S, Fut> ClientBuilder<S, Fut>
where
    S: Default + Send + Sync + Clone + Component + 'static,
    Fut: Future<Output = Result<(), anyhow::Error>> + Send + 'static,
{
    pub fn new() -> Self {
        Self {
            handler: None,
            state: S::default(),
        }
    }
    pub fn set_handler(mut self, handler: HandleFn<Fut, S>) -> Self {
        self.handler = Some(handler);
        self
    }
    pub async fn start(
        self,
        account: Account,
        address: impl TryInto<ServerAddress>,
    ) -> Result<(), StartError> {
        let address = match address.try_into() {
            Ok(address) => address,
            Err(_) => return Err(StartError::InvalidAddress),
        };

        let (bot, mut rx) = Client::join(&account, address).await?;

        while let Some(event) = rx.recv().await {
            if let Some(handler) = self.handler {
                tokio::spawn((handler)(bot.clone(), event.clone(), self.state.clone()));
            }
        }

        Ok(())
    }
}
impl<S, Fut> Default for ClientBuilder<S, Fut>
where
    S: Default + Send + Sync + Clone + Component + 'static,
    Fut: Future<Output = Result<(), anyhow::Error>> + Send + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}
