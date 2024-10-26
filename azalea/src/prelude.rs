//! The Azalea prelude. Things that are necessary for a bare-bones bot are
//! re-exported here.

pub use azalea_client::{Account, Client, Event};
pub use azalea_core::tick::GameTick;

// this is necessary to make the macros that reference bevy_ecs work
pub use crate::ecs as bevy_ecs;
pub use crate::ecs::{component::Component, system::Resource};
pub use crate::{
    bot::BotClientExt, container::ContainerClientExt, pathfinder::PathfinderClientExt,
    ClientBuilder,
};
