//! The Azalea prelude. Things that are necessary for a bare-bones bot are
//! re-exported here.

pub use crate::bot::BotClientExt;
pub use crate::pathfinder::PathfinderClientExt;
pub use azalea_client::{Account, Client, Event};
pub use azalea_ecs::component::Component;
