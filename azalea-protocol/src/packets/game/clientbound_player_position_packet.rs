use azalea_buf::{BufReadError, McBuf};
use azalea_buf::{McBufReadable, McBufWritable};
use azalea_protocol_macros::ClientboundGamePacket;
use std::io::{Cursor, Write};

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundPlayerPositionPacket {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub y_rot: f32,
    pub x_rot: f32,
    pub relative_arguments: RelativeArguments,
    /// Client should confirm this packet with Teleport Confirm containing the
    /// same Teleport ID.
    #[var]
    pub id: u32,
    pub dismount_vehicle: bool,
}

#[derive(Debug, Clone)]
pub struct RelativeArguments {
    pub x: bool,
    pub y: bool,
    pub z: bool,
    pub y_rot: bool,
    pub x_rot: bool,
}

impl McBufReadable for RelativeArguments {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let byte = u8::read_from(buf)?;
        Ok(RelativeArguments {
            x: byte & 0b1 != 0,
            y: byte & 0b10 != 0,
            z: byte & 0b100 != 0,
            y_rot: byte & 0b1000 != 0,
            x_rot: byte & 0b10000 != 0,
        })
    }
}

impl McBufWritable for RelativeArguments {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let mut byte = 0;
        if self.x {
            byte |= 0b1;
        }
        if self.y {
            byte |= 0b10;
        }
        if self.z {
            byte |= 0b100;
        }
        if self.y_rot {
            byte |= 0b1000;
        }
        if self.x_rot {
            byte |= 0b10000;
        }
        u8::write_into(&byte, buf)
    }
}
