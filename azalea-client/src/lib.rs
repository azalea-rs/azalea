//! Significantly abstract [`azalea_protocol`] so it's actually useable for
//! real clients. If you want to make bots, you should use the
//! [`azalea`] crate instead.
//!
//! [`azalea_protocol`]: https://docs.rs/azalea-protocol
//! [`azalea`]: https://docs.rs/azalea

#![feature(provide_any)]

#![allow(incomplete_features)]
#![feature(trait_upcasting)]

mod account;
mod chat;
mod client;
mod get_mc_dir;
mod movement;
pub mod ping;
mod player;
mod plugins;

pub use account::Account;
pub use client::{ChatPacket, Client, ClientInformation, Event, JoinError};
pub use movement::{SprintDirection, WalkDirection};
pub use player::Player;
pub use plugins::{Plugin, Plugins};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
