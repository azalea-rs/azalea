use azalea_buf::McBuf;
use packet_macros::GamePacket;

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundSetChunkCacheCenterPacket {
    #[var]
    pub x: i32,
    #[var]
    pub z: i32,
}
