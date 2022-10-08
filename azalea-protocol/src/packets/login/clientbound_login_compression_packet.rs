use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundLoginPacket;
use std::hash::Hash;

#[derive(Hash, Clone, Debug, ClientboundLoginPacket, McBuf)]
pub struct ClientboundLoginCompressionPacket {
    #[var]
    pub compression_threshold: i32,
}
