use azalea_buf::McBuf;
use packet_macros::GamePacket;

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundMoveEntityRotPacket {
    #[var]
    pub entity_id: i32,
    pub y_rot: i8,
    pub x_rot: i8,
    pub on_ground: bool,
}
