use packet_macros::{GamePacket, McBuf};

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundSetChunkCacheCenterPacket {
    #[var]
    pub x: i32,
    #[var]
    pub z: i32,
}
