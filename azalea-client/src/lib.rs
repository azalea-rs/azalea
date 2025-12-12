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
pub mod local_player;
pub mod ping;
pub mod player;
mod plugins;

#[cfg(feature = "log")]
#[doc(hidden)]
pub mod test_utils;

pub use account::{Account, AccountOpts};
pub use azalea_physics::local_player::{PhysicsState, SprintDirection, WalkDirection};
pub use azalea_protocol::common::client_information::ClientInformation;
// Re-export bevy-tasks so plugins can make sure that they're using the same
// version.
pub use bevy_tasks;
pub use client::{
    Client, InConfigState, InGameState, JoinedClientBundle, LocalPlayerBundle, StartClientOpts,
    start_ecs_runner,
};
pub use events::Event;
pub use movement::{StartSprintEvent, StartWalkEvent};
pub use plugins::*;
