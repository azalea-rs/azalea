//! Random miscellaneous things like UUIDs that don't deserve their own crate.

#![feature(int_roundings)]

mod difficulty;
pub use difficulty::*;

mod resource_location;
pub use resource_location::*;

mod game_type;
pub use game_type::*;

mod slot;
pub use slot::*;

mod position;
pub use position::*;

mod direction;
pub use direction::*;

mod delta;
pub use delta::*;

mod particle;
pub use particle::*;

mod cursor3d;
pub use cursor3d::*;

mod bitset;
pub use bitset::*;

// java moment
// TODO: add tests and optimize/simplify this
pub fn floor_mod(x: i32, y: u32) -> u32 {
    if x < 0 {
        y - ((-x) as u32 % y)
    } else {
        x as u32 % y
    }
}
