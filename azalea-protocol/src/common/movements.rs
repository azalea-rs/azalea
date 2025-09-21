use std::{
    io::{self, Cursor, Write},
    ops::Add,
};

use azalea_buf::{AzBuf, AzaleaRead, AzaleaWrite, BufReadError};
use azalea_core::{bitset::FixedBitSet, math, position::Vec3};
use azalea_entity::{LookDirection, Physics, Position};

/// The updated position, velocity, and rotations for an entity.
///
/// Often, this field comes alongside a [`RelativeMovements`] field, which
/// specifies which parts of this struct should be treated as relative.
#[derive(AzBuf, Clone, Debug, PartialEq)]
pub struct PositionMoveRotation {
    pub pos: Vec3,
    /// The updated delta movement (velocity).
    pub delta: Vec3,
    pub look_direction: LookDirection,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RelativeMovements {
    pub x: bool,
    pub y: bool,
    pub z: bool,
    pub y_rot: bool,
    pub x_rot: bool,
    pub delta_x: bool,
    pub delta_y: bool,
    pub delta_z: bool,
    pub rotate_delta: bool,
}
impl RelativeMovements {
    pub fn all_absolute() -> Self {
        RelativeMovements::default()
    }
    pub fn all_relative() -> Self {
        RelativeMovements {
            x: true,
            y: true,
            z: true,
            y_rot: true,
            x_rot: true,
            delta_x: true,
            delta_y: true,
            delta_z: true,
            rotate_delta: true,
        }
    }

    pub fn apply(
        &self,
        change: &PositionMoveRotation,
        position: &mut Position,
        direction: &mut LookDirection,
        physics: &mut Physics,
    ) {
        let new_position = Vec3::new(
            apply_change(position.x, self.x, change.pos.x),
            apply_change(position.y, self.y, change.pos.y),
            apply_change(position.z, self.z, change.pos.z),
        );

        let new_look_direction = LookDirection::new(
            apply_change(direction.y_rot(), self.y_rot, change.look_direction.y_rot()),
            apply_change(direction.x_rot(), self.x_rot, change.look_direction.x_rot()),
        );

        let mut new_delta = physics.velocity;
        if self.rotate_delta {
            let y_rot_delta = direction.y_rot() - new_look_direction.y_rot();
            let x_rot_delta = direction.x_rot() - new_look_direction.x_rot();
            new_delta = new_delta
                .x_rot(math::to_radians(x_rot_delta as f64) as f32)
                .y_rot(math::to_radians(y_rot_delta as f64) as f32);
        }
        let new_delta = Vec3::new(
            apply_change(new_delta.x, self.delta_x, change.delta.x),
            apply_change(new_delta.y, self.delta_y, change.delta.y),
            apply_change(new_delta.z, self.delta_z, change.delta.z),
        );

        **position = new_position;
        *direction = new_look_direction;
        physics.velocity = new_delta;
    }
}

fn apply_change<T: Add<Output = T>>(base: T, condition: bool, change: T) -> T {
    if condition { base + change } else { change }
}

impl AzaleaRead for RelativeMovements {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        // yes minecraft seriously wastes that many bits, smh
        let set = u32::azalea_read(buf)?;
        let set = FixedBitSet::<32>::new_with_data(set.swap_bytes().to_be_bytes());
        Ok(RelativeMovements {
            x: set.index(0),
            y: set.index(1),
            z: set.index(2),
            y_rot: set.index(3),
            x_rot: set.index(4),
            delta_x: set.index(5),
            delta_y: set.index(6),
            delta_z: set.index(7),
            rotate_delta: set.index(8),
        })
    }
}

impl AzaleaWrite for RelativeMovements {
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        let mut set = FixedBitSet::<32>::new();
        let mut set_bit = |index: usize, value: bool| {
            if value {
                set.set(index);
            }
        };

        set_bit(0, self.x);
        set_bit(1, self.y);
        set_bit(2, self.z);
        set_bit(3, self.y_rot);
        set_bit(4, self.x_rot);
        set_bit(5, self.delta_x);
        set_bit(6, self.delta_y);
        set_bit(7, self.delta_z);
        set_bit(8, self.rotate_delta);

        set.azalea_write(buf)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MoveFlags {
    pub on_ground: bool,
    pub horizontal_collision: bool,
}
impl AzaleaWrite for MoveFlags {
    fn azalea_write(&self, buf: &mut impl io::Write) -> Result<(), io::Error> {
        let mut bitset = FixedBitSet::<8>::new();
        if self.on_ground {
            bitset.set(0);
        }
        if self.horizontal_collision {
            bitset.set(1);
        }
        bitset.azalea_write(buf)?;
        Ok(())
    }
}
impl AzaleaRead for MoveFlags {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let bitset = FixedBitSet::<8>::azalea_read(buf)?;
        let on_ground = bitset.index(0);
        let horizontal_collision = bitset.index(1);
        Ok(Self {
            on_ground,
            horizontal_collision,
        })
    }
}
