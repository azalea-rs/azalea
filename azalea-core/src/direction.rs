use azalea_buf::AzBuf;

use crate::position::{BlockPos, Vec3, Vec3i};

#[derive(
    Clone, Copy, Debug, AzBuf, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub enum Direction {
    #[default]
    Down = 0,
    Up,
    North,
    South,
    West,
    East,
}

impl Direction {
    pub const HORIZONTAL: [Direction; 4] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];
    pub const VERTICAL: [Direction; 2] = [Direction::Down, Direction::Up];

    pub fn nearest(vec: Vec3) -> Direction {
        let mut best_direction = Direction::North;
        let mut best_direction_amount = 0.0;

        for dir in [
            Direction::Down,
            Direction::Up,
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ]
        .iter()
        {
            let amount = dir.normal_vec3().dot(vec);
            if amount > best_direction_amount {
                best_direction = *dir;
                best_direction_amount = amount;
            }
        }

        best_direction
    }

    #[inline]
    pub fn normal(self) -> BlockPos {
        match self {
            Direction::Down => BlockPos::new(0, -1, 0),
            Direction::Up => BlockPos::new(0, 1, 0),
            Direction::North => BlockPos::new(0, 0, -1),
            Direction::South => BlockPos::new(0, 0, 1),
            Direction::West => BlockPos::new(-1, 0, 0),
            Direction::East => BlockPos::new(1, 0, 0),
        }
    }

    #[inline]
    pub fn normal_vec3(self) -> Vec3 {
        self.normal().to_vec3_floored()
    }

    pub fn opposite(self) -> Direction {
        match self {
            Direction::Down => Direction::Up,
            Direction::Up => Direction::Down,
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
        }
    }

    pub fn x(self) -> i32 {
        self.normal().x
    }
    pub fn y(self) -> i32 {
        self.normal().y
    }
    pub fn z(self) -> i32 {
        self.normal().z
    }
}

/// The four cardinal directions.
///
/// Note that azalea_block has a similar enum named `FacingCardinal` that is
/// used for block states.
#[derive(Clone, Copy, Debug, AzBuf, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum CardinalDirection {
    North,
    South,
    West,
    East,
}

/// A 3D axis like x, y, z.
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
    pub fn x(self) -> i16 {
        match self {
            CardinalDirection::East => 1,
            CardinalDirection::West => -1,
            _ => 0,
        }
    }
    #[inline]
    pub fn z(self) -> i16 {
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
impl From<CardinalDirection> for Direction {
    fn from(value: CardinalDirection) -> Self {
        match value {
            CardinalDirection::North => Direction::North,
            CardinalDirection::South => Direction::South,
            CardinalDirection::West => Direction::West,
            CardinalDirection::East => Direction::East,
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
    pub fn cycle_xyz(self, pos: Vec3i, axis: Axis) -> i32 {
        match self {
            Self::None => axis.choose(pos.x, pos.y, pos.z),
            Self::Forward => axis.choose(pos.z, pos.x, pos.y),
            Self::Backward => axis.choose(pos.y, pos.z, pos.x),
        }
    }
}
