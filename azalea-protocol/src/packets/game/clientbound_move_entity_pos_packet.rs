use azalea_core::{EntityPos, PositionDelta};
use packet_macros::GamePacket;
use azalea_buf::McBuf;

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundMoveEntityPosPacket {
    #[var]
    pub entity_id: i32,
    pub delta: PositionDelta,
    pub on_ground: bool,
}
