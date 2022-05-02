use std::{hash::Hash, io::Read};

use super::LoginPacket;
use crate::mc_buf::Readable;

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

    pub fn write(&self, _buf: &mut Vec<u8>) -> Result<(), std::io::Error> {
        panic!("ClientboundHelloPacket::write not implemented")
    }

    pub  fn read(buf: &mut impl Read) -> Result<LoginPacket, String> {
        let server_id = buf.read_utf_with_len(20)?;
        let public_key = buf.read_byte_array()?;
        let nonce = buf.read_byte_array()?;

        Ok(ClientboundHelloPacket {
            server_id,
            public_key,
            nonce,
        }
        .get())
    }
}
