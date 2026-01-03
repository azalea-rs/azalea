use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundSetChunkCacheCenter {
    #[var]
    pub x: i32,
    #[var]
    pub z: i32,
}
