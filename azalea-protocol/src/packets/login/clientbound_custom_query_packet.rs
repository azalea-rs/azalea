use super::LoginPacket;
use crate::mc_buf::{Readable, Writable};
use azalea_core::resource_location::ResourceLocation;
use std::hash::Hash;

#[derive(Hash, Clone, Debug)]
pub struct ClientboundCustomQueryPacket {
    pub transaction_id: u32,
    pub identifier: ResourceLocation,
    pub data: Vec<u8>,
}

impl ClientboundCustomQueryPacket {
    pub fn get(self) -> LoginPacket {
        LoginPacket::ClientboundCustomQueryPacket(self)
    }

    pub fn write(&self, buf: &mut Vec<u8>) -> Result<(), std::io::Error> {
        buf.write_varint(self.transaction_id as i32).unwrap();
        buf.write_utf(self.identifier.to_string().as_str()).unwrap();
        buf.write_bytes(&self.data).unwrap();
        Ok(())
    }

    pub async fn read<T: tokio::io::AsyncRead + std::marker::Unpin + std::marker::Send>(
        buf: &mut T,
    ) -> Result<LoginPacket, String> {
        let transaction_id = buf.read_varint().await? as u32;
        let identifier = ResourceLocation::new(&buf.read_utf().await?)?;
        let data = buf.read_bytes_with_len(1048576).await?;
        Ok(ClientboundCustomQueryPacket {
            transaction_id,
            identifier,
            data,
        }
        .get())
    }
}
