use azalea_buf::McBuf;

use crate::floor_mod;

#[derive(Clone, Copy, Debug, McBuf)]
pub enum Direction {
    Down = 0,
    Up = 1,
    North = 2,
    South = 3,
    West = 4,
    East = 5,
}

#[derive(Clone, Copy, Debug)]
pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Clone, Copy, Debug)]
pub enum AxisCycle {
    None,
    Forward,
    Backward,
}

impl Axis {
    /// Pick x, y, or z from the arguments depending on the axis.
    #[inline]
    pub fn choose<T>(&self, x: T, y: T, z: T) -> T {
        match self {
            Axis::X => x,
            Axis::Y => y,
            Axis::Z => z,
        }
    }
}

impl AxisCycle {
    pub fn from_ordinal(ordinal: u32) -> Self {
        match ordinal {
            0 => Self::None,
            1 => Self::Forward,
            2 => Self::Backward,
            _ => panic!("invalid ordinal"),
        }
    }
    pub fn between(axis0: Axis, axis1: Axis) -> Self {
        Self::from_ordinal(floor_mod(axis1 as i32 - axis0 as i32, 3))
    }
    pub fn inverse(self) -> Self {
        match self {
            Self::None => Self::None,
            Self::Forward => Self::Backward,
            Self::Backward => Self::Forward,
        }
    }
    pub fn cycle(self, axis: Axis) -> Self {
        match self {
            Self::None => Self::None,
            Self::Forward => Self::from_ordinal(floor_mod(axis as i32 + 1, 3)),
            Self::Backward => Self::from_ordinal(floor_mod(axis as i32 - 1, 3)),
        }
    }
}
