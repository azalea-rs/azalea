//! The Azalea prelude. Things that are necessary for a bare-bones bot are
//! re-exported here.

pub use azalea_client::{Account, Event};
pub use azalea_core::tick::GameTick;
pub use bevy_app::AppExit;

// this is necessary to make the macros that reference bevy_ecs work
pub use crate::ecs as bevy_ecs;
pub use crate::{
    Client, ClientBuilder,
    ecs::{component::Component, resource::Resource},
    pathfinder::PathfinderClientExt,
};
