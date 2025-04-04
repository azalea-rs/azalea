use std::hash::Hash;

use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundLoginPacket;

#[derive(Hash, Clone, Debug, ClientboundLoginPacket, AzBuf)]
pub struct ClientboundLoginCompression {
    #[var]
    pub compression_threshold: i32,
}
