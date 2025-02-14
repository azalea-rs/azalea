//! Significantly abstract [`azalea_protocol`] so it's actually useable for
//! real clients. If you want to make bots, you should use the
//! [`azalea`] crate instead.
//!
//! [`azalea_protocol`]: https://docs.rs/azalea-protocol
//! [`azalea`]: https://docs.rs/azalea

#![feature(error_generic_member_access)]

mod account;
pub mod attack;
pub mod chat;
pub mod chunks;
mod client;
pub mod configuration;
pub mod disconnect;
mod entity_query;
pub mod events;
pub mod interact;
pub mod inventory;
mod local_player;
pub mod mining;
pub mod movement;
pub mod packet_handling;
pub mod ping;
mod player;
pub mod raw_connection;
pub mod respawn;
pub mod send_client_end;
pub mod task_pool;

pub use account::{Account, AccountOpts};
pub use azalea_protocol::common::client_information::ClientInformation;
pub use client::{
    start_ecs_runner, Client, DefaultPlugins, InConfigState, JoinError, JoinedClientBundle,
    LocalPlayerBundle, StartClientOpts, TickBroadcast,
};
pub use events::Event;
pub use local_player::{GameProfileComponent, Hunger, InstanceHolder, TabList};
pub use movement::{
    PhysicsState, SprintDirection, StartSprintEvent, StartWalkEvent, WalkDirection,
};
pub use player::PlayerInfo;
