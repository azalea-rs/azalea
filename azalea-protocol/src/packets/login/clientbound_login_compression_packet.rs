use std::{
    hash::Hash,
    io::{Read, Write},
};

use crate::mc_buf::{Readable, Writable};

use super::LoginPacket;

#[derive(Hash, Clone, Debug)]
pub struct ClientboundLoginCompressionPacket {
    pub compression_threshold: i32,
}

impl ClientboundLoginCompressionPacket {
    pub fn get(self) -> LoginPacket {
        LoginPacket::ClientboundLoginCompressionPacket(self)
    }

    pub fn write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        buf.write_varint(self.compression_threshold).unwrap();
        Ok(())
    }

    pub fn read(buf: &mut impl Read) -> Result<LoginPacket, String> {
        let compression_threshold = buf.read_varint()?;

        Ok(ClientboundLoginCompressionPacket {
            compression_threshold,
        }
        .get())
    }
}
