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
//! Then, add one of the following lines to your Cargo.toml:
//!
//! Latest bleeding-edge version:
//! `azalea = { git="https://github.com/mat-1/azalea" }`\
//! Latest "stable" release:
//! `azalea = "0.5.0"`
//!
//! ## Optimization
//!
//! For faster compile times, make a `.cargo/config.toml` file in your project
//! and copy
//! [this file](https://github.com/mat-1/azalea/blob/main/.cargo/config.toml)
//! into it. You may have to install the LLD linker.
//!
//! For faster performance in debug mode, add the following code to your
//! Cargo.toml:
//! ```toml
//! [profile.dev]
//! opt-level = 1
//! [profile.dev.package."*"]
//! opt-level = 3
//! ```
//!
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
//! #[derive(Default, Clone, Component)]
//! pub struct State {}
//!
//! async fn handle(bot: Client, event: Event, state: State) -> anyhow::Result<()> {
//!     match event {
//!         Event::Chat(m) => {
//!             println!("{}", m.message().to_ansi());
//!         }
//!         _ => {}
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! [`azalea_client`]: https://docs.rs/azalea-client

#![feature(trait_upcasting)]
#![feature(async_closure)]
#![allow(incomplete_features)]

mod bot;
pub mod pathfinder;
pub mod prelude;
mod start;
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
use start::StartError;
pub use start::{start, Options};
pub use swarm::*;

pub type HandleFn<Fut, S> = fn(Client, Event, S) -> Fut;

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
