use std::ops::{Add, Mul};

use azalea_core::position::BlockPos;

/// An offset from a block position.
///
/// This fits in 64 bits, so it's more efficient than a `BlockPos` in some
/// cases.
///
/// The X and Z are limited to ±32k.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct RelBlockPos {
    pub x: i16,
    pub z: i16,
    /// The actual non-relative Y coordinate of the block.
    pub y: i32,
}

impl RelBlockPos {
    #[must_use]
    pub const fn get_origin(origin: BlockPos) -> Self {
        Self::new(0, origin.y, 0)
    }

    #[inline]
    #[must_use]
    pub const fn new(x: i16, y: i32, z: i16) -> Self {
        Self { x, z, y }
    }

    #[inline]
    #[must_use]
    pub const fn apply(self, origin: BlockPos) -> BlockPos {
        BlockPos::new(origin.x + self.x as i32, self.y, origin.z + self.z as i32)
    }

    /// Create a new [`RelBlockPos`] from a given origin and new position.
    #[inline]
    #[must_use]
    pub const fn from_origin(origin: BlockPos, new: BlockPos) -> Self {
        Self {
            x: (new.x - origin.x) as i16,
            y: new.y,
            z: (new.z - origin.z) as i16,
        }
    }

    #[inline]
    #[must_use]
    pub const fn up(&self, y: i32) -> Self {
        Self {
            x: self.x,
            y: self.y + y,
            z: self.z,
        }
    }
    #[inline]
    #[must_use]
    pub const fn down(&self, y: i32) -> Self {
        Self {
            x: self.x,
            y: self.y - y,
            z: self.z,
        }
    }
    #[inline]
    #[must_use]
    pub const fn north(&self, z: i16) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z - z,
        }
    }
    #[inline]
    #[must_use]
    pub const fn south(&self, z: i16) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z + z,
        }
    }
    #[inline]
    #[must_use]
    pub const fn east(&self, x: i16) -> Self {
        Self {
            x: self.x + x,
            y: self.y,
            z: self.z,
        }
    }
    #[inline]
    #[must_use]
    pub const fn west(&self, x: i16) -> Self {
        Self {
            x: self.x - x,
            y: self.y,
            z: self.z,
        }
    }
}

impl Add<RelBlockPos> for RelBlockPos {
    type Output = RelBlockPos;

    fn add(self, rhs: RelBlockPos) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl Mul<i16> for RelBlockPos {
    type Output = RelBlockPos;

    fn mul(self, rhs: i16) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs as i32,
            z: self.z * rhs,
        }
    }
}
