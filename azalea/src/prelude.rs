//! The Azalea prelude. Things that are necessary for a bare-bones bot are re-exported here.

pub use crate::bot::BotTrait;
pub use crate::pathfinder::Trait;
pub use crate::{plugins, swarm_plugins, Plugin};
pub use azalea_client::{Account, Client, Event};
