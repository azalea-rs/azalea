use std::hash::Hash;
use tokio::io::BufReader;

use crate::mc_buf::Readable;

use super::LoginPacket;

#[derive(Hash, Clone, Debug)]
pub struct ClientboundHelloPacket {
    pub server_id: String,
    pub public_key: Vec<u8>,
    pub nonce: Vec<u8>,
}

impl ClientboundHelloPacket {
    pub fn get(self) -> LoginPacket {
        LoginPacket::ClientboundHelloPacket(self)
    }

    pub fn write(&self, _buf: &mut Vec<u8>) {
        panic!("ClientboundHelloPacket::write not implemented")
    }

    pub async fn read<T: tokio::io::AsyncRead + std::marker::Unpin + std::marker::Send>(
        buf: &mut BufReader<T>,
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
