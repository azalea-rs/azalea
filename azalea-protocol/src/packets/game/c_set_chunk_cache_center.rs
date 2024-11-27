use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundSetChunkCacheCenter {
    #[var]
    pub x: i32,
    #[var]
    pub z: i32,
}
