use std::hash::Hash;

use super::LoginPacket;
use crate::mc_buf::{ByteArray, Readable};

#[derive(Hash, Clone, Debug)]
pub struct ClientboundHelloPacket {
    pub server_id: String,
    pub public_key: ByteArray,
    pub nonce: ByteArray,
}

impl ClientboundHelloPacket {
    pub fn get(self) -> LoginPacket {
        LoginPacket::ClientboundHelloPacket(self)
    }

    pub fn write(&self, _buf: &mut Vec<u8>) -> Result<(), std::io::Error> {
        panic!("ClientboundHelloPacket::write not implemented")
    }

    pub async fn read<T: tokio::io::AsyncRead + std::marker::Unpin + std::marker::Send>(
        buf: &mut T,
    ) -> Result<LoginPacket, String> {
        let server_id = buf.read_utf_with_len(20).await?;
        let public_key = buf.read_byte_array().await?;
        let nonce = buf.read_byte_array().await?;

        Ok(ClientboundHelloPacket {
            server_id,
            public_key,
            nonce,
        }
        .get())
    }
}
