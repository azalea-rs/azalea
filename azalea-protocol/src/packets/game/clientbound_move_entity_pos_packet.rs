use azalea_core::EntityPos;
use packet_macros::{GamePacket, McBuf};

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundMoveEntityPosPacket {
    #[var]
    pub entity_id: i32,
    pub delta: PositionDelta,
    pub on_ground: bool,
}
