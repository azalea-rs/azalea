use packet_macros::GamePacket;

#[derive(Clone, Debug, GamePacket)]
pub struct ClientboundMoveEntityPacket {
    #[var]
    pub entity_id: u32,
    pub y_rot: i16,
    pub x_rot: i16,
    pub z_rot: i16,
    pub on_ground: bool,
}
