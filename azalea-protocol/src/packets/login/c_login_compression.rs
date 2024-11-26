use std::hash::Hash;

use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundLoginPacket;

#[derive(Hash, Clone, Debug, ClientboundLoginPacket, McBuf)]
pub struct ClientboundLoginCompression {
    #[var]
    pub compression_threshold: i32,
}
