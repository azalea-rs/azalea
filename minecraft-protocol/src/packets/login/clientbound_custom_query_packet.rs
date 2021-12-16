use std::hash::Hash;
use tokio::io::BufReader;

use crate::mc_buf;

use super::LoginPacket;

#[derive(Hash, Clone, Debug)]
pub struct ClientboundCustomQueryPacket {
    pub transacton_id: u32,
	// TODO: this should be a resource location
	pub identifier: String,
	pub data: Vec<u8>,
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
        // let server_id = mc_buf::read_utf_with_len(buf, 20).await?;
        // let public_key = mc_buf::read_byte_array(buf).await?;
        // let nonce = mc_buf::read_byte_array(buf).await?;

        // Ok(ClientboundHelloPacket {
        //     server_id,
        //     public_key,
        //     nonce,
        // }
        // .get())
    }
}
