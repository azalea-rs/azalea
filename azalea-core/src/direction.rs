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

pub enum Axis {
    X,
    Y,
    Z,
}

pub enum AxisCycle {
    None,
    Forward,
    Backward,
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
}
