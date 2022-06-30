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

mod aabb;
pub use aabb::*;

mod block_hit_result;
pub use block_hit_result::*;

// java moment
// TODO: add tests and optimize/simplify this
pub fn floor_mod(x: i32, y: u32) -> u32 {
    if x < 0 {
        y - ((-x) as u32 % y)
    } else {
        x as u32 % y
    }
}

// TODO: make this generic
pub fn binary_search(mut min: u32, max: u32, predicate: &dyn Fn(u32) -> bool) -> u32 {
    let mut diff = max - min;
    while diff > 0 {
        let diff_mid = diff / 2;
        let mid = min + diff_mid;
        if predicate(mid) {
            diff = diff_mid;
        } else {
            min = mid + 1;
            diff -= diff_mid + 1;
        }
    }

    min
}
