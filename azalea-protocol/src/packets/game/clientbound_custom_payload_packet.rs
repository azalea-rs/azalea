use super::GamePacket;
use crate::mc_buf::{Readable, Writable};
use azalea_core::{game_type::GameType, resource_location::ResourceLocation};

#[derive(Clone, Debug)]
pub struct ClientboundCustomPayloadPacket {
    pub identifier: ResourceLocation,
    pub data: Vec<u8>,
}

impl ClientboundCustomPayloadPacket {
    pub fn get(self) -> GamePacket {
        GamePacket::ClientboundCustomPayloadPacket(self)
    }

    pub fn write(&self, buf: &mut Vec<u8>) -> Result<(), std::io::Error> {
        buf.write_resource_location(&self.identifier)?;
        buf.write_bytes(&self.data)?;
        Ok(())
    }

    pub async fn read<T: tokio::io::AsyncRead + std::marker::Unpin + std::marker::Send>(
        buf: &mut T,
    ) -> Result<GamePacket, String> {
        let identifier = buf.read_resource_location().await?;
        let data = buf.read_bytes().await?;

        Ok(ClientboundCustomPayloadPacket { identifier, data }.get())
    }
}
