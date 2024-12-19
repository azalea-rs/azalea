use std::io::{self, Cursor, Write};

use azalea_buf::{AzBuf, AzaleaRead, AzaleaWrite, BufReadError};
use azalea_core::{bitset::FixedBitSet, position::Vec3};
use azalea_entity::LookDirection;

/// The updated position, velocity, and rotations for an entity.
///
/// Often, this field comes alongside a [`RelativeMovements`] field, which
/// specifies which parts of this struct should be treated as relative.
#[derive(AzBuf, Clone, Debug)]
pub struct PositionMoveRotation {
    pub pos: Vec3,
    /// The updated delta movement (velocity).
    pub delta: Vec3,
    pub look_direction: LookDirection,
}

#[derive(Debug, Clone)]
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

impl AzaleaRead for RelativeMovements {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        // yes minecraft seriously wastes that many bits, smh
        let set = FixedBitSet::<{ 32_usize.div_ceil(8) }>::azalea_read(buf)?;
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
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), io::Error> {
        let mut set = FixedBitSet::<{ 32_usize.div_ceil(8) }>::new();
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
