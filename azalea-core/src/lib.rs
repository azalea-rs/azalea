//! Random miscellaneous things like UUIDs that don't deserve their own crate.

#![feature(int_roundings)]

mod difficulty;
pub use difficulty::*;

mod resource_location;
pub use resource_location::*;

mod game_type;
pub use game_type::*;

mod slot;
pub use slot::{Slot, SlotData};

mod position;
pub use position::*;

mod direction;
pub use direction::Direction;

mod delta;
pub use delta::*;

mod particle;
pub use particle::*;
