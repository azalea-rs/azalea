use std::io::{Cursor, Write};

use azalea_buf::{AzBuf, AzaleaRead, AzaleaWrite, BufReadError};
use azalea_core::{bitset::FixedBitSet, position::Vec3};
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundPlayerPosition {
    #[var]
    pub id: u32,
    pub pos: Vec3,
    pub delta_movement: Vec3,
    pub y_rot: f32,
    pub x_rot: f32,
    pub relative_arguments: RelativeMovements,
}

#[derive(Debug, Clone)]
pub struct RelativeMovements {
    pub x: bool,
    pub y: bool,
    pub z: bool,
    pub y_rot: bool,
    pub x_rot: bool,
}

impl AzaleaRead for RelativeMovements {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        // yes minecraft seriously wastes that many bits, smh
        let set = FixedBitSet::<32>::azalea_read(buf)?;
        Ok(RelativeMovements {
            x: set.index(0),
            y: set.index(1),
            z: set.index(2),
            y_rot: set.index(3),
            x_rot: set.index(4),
        })
    }
}

impl AzaleaWrite for RelativeMovements {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let mut set = FixedBitSet::<5>::new();
        if self.x {
            set.set(0);
        }
        if self.y {
            set.set(1);
        }
        if self.z {
            set.set(2);
        }
        if self.y_rot {
            set.set(3);
        }
        if self.x_rot {
            set.set(4);
        }
        set.azalea_write(buf)
    }
}
