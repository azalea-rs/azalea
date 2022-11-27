use azalea_buf::McBuf;

#[derive(Clone, Copy, Debug, McBuf, Default)]
pub enum Direction {
    #[default]
    Down = 0,
    Up,
    North,
    South,
    West,
    East,
}

// TODO: make azalea_block use this instead of FacingCardinal
#[derive(Clone, Copy, Debug, McBuf)]
pub enum CardinalDirection {
    North,
    South,
    West,
    East,
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

impl CardinalDirection {
    #[inline]
    pub fn x(self) -> i32 {
        match self {
            CardinalDirection::East => 1,
            CardinalDirection::West => -1,
            _ => 0,
        }
    }
    #[inline]
    pub fn z(self) -> i32 {
        match self {
            CardinalDirection::South => 1,
            CardinalDirection::North => -1,
            _ => 0,
        }
    }

    pub fn iter() -> impl Iterator<Item = CardinalDirection> {
        [
            CardinalDirection::North,
            CardinalDirection::South,
            CardinalDirection::West,
            CardinalDirection::East,
        ]
        .iter()
        .copied()
    }

    #[inline]
    pub fn right(self) -> CardinalDirection {
        match self {
            CardinalDirection::North => CardinalDirection::East,
            CardinalDirection::South => CardinalDirection::West,
            CardinalDirection::West => CardinalDirection::North,
            CardinalDirection::East => CardinalDirection::South,
        }
    }
    #[inline]
    pub fn left(self) -> CardinalDirection {
        match self {
            CardinalDirection::North => CardinalDirection::West,
            CardinalDirection::South => CardinalDirection::East,
            CardinalDirection::West => CardinalDirection::South,
            CardinalDirection::East => CardinalDirection::North,
        }
    }
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
        Self::from_ordinal(i32::rem_euclid(axis1 as i32 - axis0 as i32, 3) as u32)
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
            Self::Forward => Axis::from_ordinal(i32::rem_euclid(axis as i32 + 1, 3) as u32),
            Self::Backward => Axis::from_ordinal(i32::rem_euclid(axis as i32 - 1, 3) as u32),
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
