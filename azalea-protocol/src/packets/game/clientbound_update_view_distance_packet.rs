use packet_macros::{GamePacket, McBuf};

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundUpdateViewDistancePacket {
    #[var]
    pub view_distance: i32,
}
