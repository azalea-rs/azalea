use packet_macros::{GamePacket, McBuf};

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundRotateHeadPacket {
    #[var]
    pub entity_id: u32,
    pub y_head_rot: i8,
}
