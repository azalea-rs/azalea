//! The Azalea prelude. Things that are necessary for a bare-bones bot are
//! re-exported here.

pub use crate::{bot::BotClientExt, pathfinder::PathfinderClientExt, ClientBuilder};
pub use azalea_client::{Account, Client, Event};
pub use azalea_ecs::{component::Component, system::Resource};
