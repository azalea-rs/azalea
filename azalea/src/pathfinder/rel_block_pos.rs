use std::{
    hash::{Hash, Hasher},
    mem::transmute,
    ops::{Add, Mul},
};

use azalea_core::position::BlockPos;

/// An offset from a block position.
///
/// This fits in 64 bits, so it's more efficient than a BlockPos in some cases.
///
/// The X and Z are limited to ±32k.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct RelBlockPos {
    /// The actual non-relative Y coordinate of the block.
    pub y: i32,
    /// The X coordinate of the block, relative to some origin.
    pub x: i16,
    /// The Y coordinate of the block, relative to some origin.
    pub z: i16,
}

impl RelBlockPos {
    pub fn get_origin(origin: BlockPos) -> Self {
        Self::new(0, origin.y, 0)
    }

    #[inline]
    pub const fn new(x: i16, y: i32, z: i16) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub fn apply(self, origin: BlockPos) -> BlockPos {
        BlockPos::new(origin.x + self.x as i32, self.y, origin.z + self.z as i32)
    }

    /// Create a new [`RelBlockPos`] from a given origin and new position.
    #[inline]
    pub fn from_origin(origin: BlockPos, new: BlockPos) -> Self {
        Self {
            x: (new.x - origin.x) as i16,
            y: new.y,
            z: (new.z - origin.z) as i16,
        }
    }

    #[inline]
    pub fn up(&self, y: i32) -> Self {
        Self {
            x: self.x,
            y: self.y + y,
            z: self.z,
        }
    }
    #[inline]
    pub fn down(&self, y: i32) -> Self {
        Self {
            x: self.x,
            y: self.y - y,
            z: self.z,
        }
    }
    #[inline]
    pub fn north(&self, z: i16) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z - z,
        }
    }
    #[inline]
    pub fn south(&self, z: i16) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z + z,
        }
    }
    #[inline]
    pub fn east(&self, x: i16) -> Self {
        Self {
            x: self.x + x,
            y: self.y,
            z: self.z,
        }
    }
    #[inline]
    pub fn west(&self, x: i16) -> Self {
        Self {
            x: self.x - x,
            y: self.y,
            z: self.z,
        }
    }

    #[inline]
    pub fn as_u64(self) -> u64 {
        // SAFETY: RelBlockPos can be represented as a u64
        unsafe { transmute::<Self, u64>(self) }
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

impl Hash for RelBlockPos {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_u64().hash(state);
    }
}
