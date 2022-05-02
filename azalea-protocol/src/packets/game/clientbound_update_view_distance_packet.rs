use packet_macros::GamePacket;

#[derive(Clone, Debug, GamePacket)]
pub struct ClientboundUpdateViewDistancePacket {
    #[varint]
    pub view_distance: i32,
}
