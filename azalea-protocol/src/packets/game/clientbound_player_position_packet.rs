// i don't know the actual name of this packet, i couldn't find it in the source code

use crate::mc_buf::{McBufReadable, McBufWritable, Readable};
use async_trait::async_trait;
use packet_macros::GamePacket;
use tokio::io::AsyncRead;

#[derive(Clone, Debug, GamePacket)]
pub struct ClientboundPlayerPositionPacket {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub y_rot: f32,
    pub x_rot: f32,
    pub relative_arguments: RelativeArguments,
    /// Client should confirm this packet with Teleport Confirm containing the
    /// same Teleport ID.
    #[varint]
    pub id: i32,
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

#[async_trait]
impl McBufReadable for RelativeArguments {
    async fn read_into<R>(buf: &mut R) -> Result<Self, String>
    where
        R: AsyncRead + std::marker::Unpin + std::marker::Send,
    {
        let byte = buf.read_byte().await?;
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
    fn write_into(&self, buf: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let mut byte = 0;
        if self.x {
            byte = byte | 0b1;
        }
        if self.y {
            byte = byte | 0b10;
        }
        if self.z {
            byte = byte | 0b100;
        }
        if self.y_rot {
            byte = byte | 0b1000;
        }
        if self.x_rot {
            byte = byte | 0b10000;
        }
        u8::write_into(&byte, buf)
    }
}
