use packet_macros::GamePacket;

#[derive(Clone, Debug, GamePacket)]
pub struct ClientboundSetChunkCacheCenterPacket {
    #[varint]
    pub x: i32,
    #[varint]
    pub z: i32,
}
