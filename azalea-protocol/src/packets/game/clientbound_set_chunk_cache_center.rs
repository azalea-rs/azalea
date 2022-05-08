use packet_macros::GamePacket;

#[derive(Clone, Debug, GamePacket)]
pub struct ClientboundSetChunkCacheCenterPacket {
    #[var]
    pub x: i32,
    #[var]
    pub z: i32,
}
