use azalea_buf::McBuf;
use azalea_core::PositionDelta8;
use azalea_protocol_macros::ClientboundGamePacket;

/// This packet is sent by the server when an entity moves less then 8 blocks.
#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundMoveEntityPosRotPacket {
    #[var]
    pub entity_id: u32,
    pub delta: PositionDelta8,
    pub y_rot: i8,
    pub x_rot: i8,
    pub on_ground: bool,
}
