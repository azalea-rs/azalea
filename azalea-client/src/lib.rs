//! Significantly abstract [`azalea_protocol`] so it's actually useable for
//! real clients. If you want to make bots, you should use the
//! [`azalea`] crate instead.
//!
//! [`azalea_protocol`]: https://docs.rs/azalea-protocol
//! [`azalea`]: https://docs.rs/azalea

#![feature(error_generic_member_access)]
#![feature(never_type)]

mod account;
mod client;
mod entity_query;
mod local_player;
pub mod ping;
mod player;
mod plugins;

#[doc(hidden)]
pub mod test_simulation;

pub use account::{Account, AccountOpts};
pub use azalea_protocol::common::client_information::ClientInformation;
pub use client::{
    Client, InConfigState, InGameState, JoinError, JoinedClientBundle, LocalPlayerBundle,
    StartClientOpts, TickBroadcast, start_ecs_runner,
};
pub use events::Event;
pub use local_player::{GameProfileComponent, Hunger, InstanceHolder, TabList};
pub use movement::{
    PhysicsState, SprintDirection, StartSprintEvent, StartWalkEvent, WalkDirection,
};
pub use player::PlayerInfo;
pub use plugins::*;
