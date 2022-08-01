use std::{
    hash::Hash,
    io::{Read, Write},
};

use azalea_buf::{Readable, Writable};

use super::ClientboundLoginPacket;

#[derive(Hash, Clone, Debug)]
pub struct ClientboundLoginCompressionPacket {
    pub compression_threshold: i32,
}

impl ClientboundLoginCompressionPacket {
    pub fn get(self) -> ClientboundLoginPacket {
        ClientboundLoginPacket::ClientboundLoginCompressionPacket(self)
    }

    pub fn write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        buf.write_varint(self.compression_threshold).unwrap();
        Ok(())
    }

    pub fn read(buf: &mut impl Read) -> Result<ClientboundLoginPacket, String> {
        let compression_threshold = buf.read_varint()?;

        Ok(ClientboundLoginCompressionPacket {
            compression_threshold,
        }
        .get())
    }
}
