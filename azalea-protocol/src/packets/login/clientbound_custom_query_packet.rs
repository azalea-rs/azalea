use std::hash::Hash;
use tokio::io::BufReader;

use crate::mc_buf::{Readable, Writable};

use super::LoginPacket;

#[derive(Hash, Clone, Debug)]
pub struct ClientboundCustomQueryPacket {
    pub transaction_id: u32,
    // TODO: this should be a resource location
    pub identifier: String,
    pub data: Vec<u8>,
}

impl ClientboundCustomQueryPacket {
    pub fn get(self) -> LoginPacket {
        LoginPacket::ClientboundCustomQueryPacket(self)
    }

    pub fn write(&self, buf: &mut Vec<u8>) {
        buf.write_varint(self.transaction_id as i32).unwrap();
        buf.write_utf(&self.identifier).unwrap();
        buf.write_bytes(&self.data).unwrap();
    }

    pub async fn read<T: tokio::io::AsyncRead + std::marker::Unpin + std::marker::Send>(
        buf: &mut BufReader<T>,
    ) -> Result<LoginPacket, String> {
        let transaction_id = buf.read_varint().await?.0 as u32;
        // TODO: this should be a resource location
        let identifier = buf.read_utf().await?;
        let data = buf.read_bytes(1048576).await?;
        Ok(ClientboundCustomQueryPacket {
            transaction_id,
            identifier,
            data,
        }
        .get())
    }
}
