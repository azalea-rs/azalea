use std::hash::Hash;

use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundLoginPacket;

#[derive(AzBuf, ClientboundLoginPacket, Clone, Debug, Hash, PartialEq)]
pub struct ClientboundLoginCompression {
    #[var]
    pub compression_threshold: i32,
}
