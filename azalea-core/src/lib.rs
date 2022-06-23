//! Random miscellaneous things like UUIDs that don't deserve their own crate.

#![feature(int_roundings)]

pub mod difficulty;
pub mod game_type;
pub mod resource_location;
pub mod serializable_uuid;

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
