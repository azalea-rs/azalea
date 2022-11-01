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
mod start;
mod swarm;

pub use azalea_client::*;
pub use azalea_core::{BlockPos, Vec3};
pub use start::{start, Options};
pub use swarm::*;

pub type HandleFn<Fut, S> = fn(Client, Event, S) -> Fut;

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
            let mut plugins = azalea::Plugins::new();
            $(
                plugins.add($plugin);
            )*
            plugins
        }
    };
}
