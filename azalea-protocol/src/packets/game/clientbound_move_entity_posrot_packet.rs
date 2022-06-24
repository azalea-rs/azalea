use azalea_buf::McBuf;
use azalea_core::PositionDelta;
use packet_macros::GamePacket;

/// This packet is sent by the server when an entity moves less then 8 blocks.
#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundMoveEntityPosRotPacket {
    #[var]
    pub entity_id: u32,
    pub delta: PositionDelta,
    pub y_rot: i8,
    pub x_rot: i8,
    pub on_ground: bool,
}
