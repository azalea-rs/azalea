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

// TODO: make this generic
pub fn binary_search(mut min: i32, max: i32, predicate: &dyn Fn(i32) -> bool) -> i32 {
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

pub fn lcm(a: u32, b: u32) -> u64 {
    let gcd = gcd(a, b);
    (a as u64) * (b / gcd) as u64
}
pub fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(0, 0), 0);
        assert_eq!(gcd(1, 1), 1);

        assert_eq!(gcd(0, 1), 1);
        assert_eq!(gcd(1, 0), 1);

        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(8, 12), 4);

        assert_eq!(gcd(12, 9), 3);
        assert_eq!(gcd(9, 12), 3);

        assert_eq!(gcd(12, 7), 1);
        assert_eq!(gcd(7, 12), 1);
    }
}
