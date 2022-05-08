use packet_macros::GamePacket;

#[derive(Clone, Debug, GamePacket)]
pub struct ClientboundMoveEntityPosRotPacket {
    #[var]
    pub entity_id: i32,
    pub xa: i16,
    pub ya: i16,
    pub za: i16,
    pub y_rot: i8,
    pub x_rot: i8,
    pub on_ground: bool,
}
