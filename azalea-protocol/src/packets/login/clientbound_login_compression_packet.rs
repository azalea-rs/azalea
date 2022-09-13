use super::ClientboundLoginPacket;
use azalea_buf::{BufReadError, McBufVarReadable, McBufVarWritable};
use std::{
    hash::Hash,
    io::{Read, Write},
};

#[derive(Hash, Clone, Debug)]
pub struct ClientboundLoginCompressionPacket {
    pub compression_threshold: i32,
}

impl ClientboundLoginCompressionPacket {
    pub fn get(self) -> ClientboundLoginPacket {
        ClientboundLoginPacket::LoginCompression(self)
    }

    pub fn write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        self.compression_threshold.var_write_into(buf)?;
        Ok(())
    }

    pub fn read(buf: &mut &[u8]) -> Result<ClientboundLoginPacket, BufReadError> {
        let compression_threshold = i32::var_read_from(buf)?;

        Ok(ClientboundLoginCompressionPacket {
            compression_threshold,
        }
        .get())
    }
}
