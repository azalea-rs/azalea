//! Significantly abstract [`azalea_protocol`] so it's actually useable for
//! real clients. If you want to make bots, you should use the
//! [`azalea`] crate instead.
//!
//! [`azalea_protocol`]: https://crates.io/crates/azalea-protocol
//! [`azalea`]: https://crates.io/crates/azalea

mod account;
mod chat;
mod client;
mod get_mc_dir;
mod movement;
pub mod ping;
mod player;

pub use account::Account;
pub use client::{ChatPacket, Client, ClientInformation, Event, JoinError};
pub use movement::MoveDirection;
pub use player::Player;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
