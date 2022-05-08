use packet_macros::GamePacket;

#[derive(Clone, Debug, GamePacket)]
pub struct ClientboundRotateHeadPacket {
    #[var]
    pub entity_id: u32,
    pub y_head_rot: i8,
}
