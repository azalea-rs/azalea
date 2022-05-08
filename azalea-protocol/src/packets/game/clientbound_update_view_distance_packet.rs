use packet_macros::GamePacket;

#[derive(Clone, Debug, GamePacket)]
pub struct ClientboundUpdateViewDistancePacket {
    #[var]
    pub view_distance: i32,
}
