//! Significantly abstract [`azalea_protocol`] so it's actually useable for
//! real clients. If you want to make bots, you should use the
//! [`azalea`] crate instead.
//!
//! [`azalea_protocol`]: https://crates.io/crates/azalea-protocol
//! [`azalea`]: https://crates.io/crates/azalea

#![feature(provide_any)]
#![allow(incomplete_features)]
#![feature(trait_upcasting)]
#![feature(error_generic_member_access)]
#![feature(type_alias_impl_trait)]

mod account;
mod chat;
mod client;
mod entity_query;
mod events;
mod get_mc_dir;
mod local_player;
mod movement;
pub mod packet_handling;
pub mod ping;
mod player;
pub mod task_pool;

pub use account::Account;
pub use azalea_ecs as ecs;
pub use client::{init_ecs_app, start_ecs, ChatPacket, Client, ClientInformation, JoinError};
pub use events::Event;
pub use local_player::{GameProfileComponent, LocalPlayer};
pub use movement::{SprintDirection, StartSprintEvent, StartWalkEvent, WalkDirection};
pub use player::PlayerInfo;
