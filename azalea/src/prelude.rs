//! The Azalea prelude. Things that are necessary for a bare-bones bot are
//! re-exported here.

pub use crate::{
    bot::BotClientExt, container::ContainerClientExt, pathfinder::PathfinderClientExt,
    ClientBuilder,
};
pub use azalea_client::{Account, Client, Event};
// this is necessary to make the macros that reference bevy_ecs work
pub use crate::ecs as bevy_ecs;
pub use crate::ecs::{component::Component, system::Resource};
