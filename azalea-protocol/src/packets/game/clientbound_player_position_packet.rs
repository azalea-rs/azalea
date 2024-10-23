use std::io::{Cursor, Write};

use azalea_buf::{BufReadError, McBuf, McBufReadable, McBufWritable};
use azalea_core::{bitset::FixedBitSet, position::Vec3};
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundPlayerPositionPacket {
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

impl McBufReadable for RelativeMovements {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        // yes minecraft seriously wastes that many bits, smh
        let set = FixedBitSet::<32>::read_from(buf)?;
        Ok(RelativeMovements {
            x: set.index(0),
            y: set.index(1),
            z: set.index(2),
            y_rot: set.index(3),
            x_rot: set.index(4),
        })
    }
}

impl McBufWritable for RelativeMovements {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
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
        set.write_into(buf)
    }
}
