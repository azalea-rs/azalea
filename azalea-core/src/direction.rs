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
    X = 0,
    Y = 1,
    Z = 2,
}

#[derive(Clone, Copy, Debug)]
pub enum AxisCycle {
    None = 0,
    Forward = 1,
    Backward = 2,
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

    pub fn from_ordinal(ordinal: u32) -> Self {
        match ordinal {
            0 => Axis::X,
            1 => Axis::Y,
            2 => Axis::Z,
            _ => panic!("Invalid ordinal {ordinal}"),
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
    pub fn cycle(self, axis: Axis) -> Axis {
        match self {
            Self::None => axis,
            Self::Forward => Axis::from_ordinal(floor_mod(axis as i32 + 1, 3)),
            Self::Backward => Axis::from_ordinal(floor_mod(axis as i32 - 1, 3)),
        }
    }
    pub fn cycle_xyz(self, x: i32, y: i32, z: i32, axis: Axis) -> i32 {
        match self {
            Self::None => axis.choose(x, y, z),
            Self::Forward => axis.choose(z, x, y),
            Self::Backward => axis.choose(y, z, x),
        }
    }
}
