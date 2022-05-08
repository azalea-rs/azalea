use packet_macros::GamePacket;

#[derive(Clone, Debug, GamePacket)]
pub struct ClientboundMoveEntityRotPacket {
    #[var]
    pub entity_id: i32,
    pub y_rot: i8,
    pub x_rot: i8,
    pub on_ground: bool,
}
