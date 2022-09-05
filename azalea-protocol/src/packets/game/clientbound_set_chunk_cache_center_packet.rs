use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundSetChunkCacheCenterPacket {
    #[var]
    pub x: i32,
    #[var]
    pub z: i32,
}
