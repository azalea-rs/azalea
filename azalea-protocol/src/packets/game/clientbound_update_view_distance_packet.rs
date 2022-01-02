// i don't know the actual name of this packet, i couldn't find it in the source code!

use super::GamePacket;
use crate::mc_buf::{Readable, Writable};

#[derive(Clone, Debug)]
pub struct ClientboundUpdateViewDistancePacket {
    pub view_distance: i32,
}

impl ClientboundUpdateViewDistancePacket {
    pub fn get(self) -> GamePacket {
        GamePacket::ClientboundUpdateViewDistancePacket(self)
    }

    pub fn write(&self, buf: &mut Vec<u8>) -> Result<(), std::io::Error> {
        buf.write_varint(self.view_distance)?;
        Ok(())
    }

    pub async fn read<T: tokio::io::AsyncRead + std::marker::Unpin + std::marker::Send>(
        buf: &mut T,
    ) -> Result<GamePacket, String> {
        let view_distance = buf.read_varint().await?;

        Ok(ClientboundUpdateViewDistancePacket { view_distance }.get())
    }
}
