//! The Azalea prelude. Things that are necessary for a bare-bones bot are
//! re-exported here.

pub use crate::bot::BotClientExt;
pub use crate::pathfinder::PathfinderClientExt;
pub use crate::{ClientBuilder, SwarmBuilder};
pub use azalea_client::{Account, Client, Event};
pub use azalea_ecs::component::Component;
pub use azalea_ecs::system::Resource;
