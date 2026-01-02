#![doc = include_str!("../README.md")]
#![feature(error_generic_member_access)]
#![feature(never_type)]

pub mod account;
mod client;
pub mod local_player;
pub mod ping;
pub mod player;
mod plugins;

#[cfg(feature = "log")]
#[doc(hidden)]
pub mod test_utils;

#[deprecated = "moved to `account::Account`."]
pub type Account = account::Account;

pub use azalea_physics::local_player::{PhysicsState, SprintDirection, WalkDirection};
pub use azalea_protocol::common::client_information::ClientInformation;
// Re-export bevy-tasks so plugins can make sure that they're using the same
// version.
pub use bevy_tasks;
pub use client::{
    InConfigState, InGameState, JoinedClientBundle, LocalPlayerBundle, start_ecs_runner,
};
pub use movement::{StartSprintEvent, StartWalkEvent};
pub use plugins::*;
