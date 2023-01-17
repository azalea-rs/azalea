//! Significantly abstract [`azalea_protocol`] so it's actually useable for
//! real clients. If you want to make bots, you should use the
//! [`azalea`] crate instead.
//!
//! [`azalea_protocol`]: https://docs.rs/azalea-protocol
//! [`azalea`]: https://docs.rs/azalea

#![feature(provide_any)]
#![allow(incomplete_features)]
#![feature(trait_upcasting)]
#![feature(error_generic_member_access)]

mod account;
mod chat;
mod client;
mod entity_query;
mod get_mc_dir;
mod local_player;
mod movement;
pub mod packet_handling;
pub mod ping;
mod player;
mod plugins;

pub use account::Account;
pub use bevy_ecs as ecs;
pub use client::{start_ecs, ChatPacket, Client, ClientInformation, Event, JoinError};
pub use local_player::LocalPlayer;
pub use movement::{SprintDirection, StartSprintEvent, StartWalkEvent, WalkDirection};
pub use player::PlayerInfo;
pub use plugins::{Plugin, PluginState, PluginStates, Plugins};
