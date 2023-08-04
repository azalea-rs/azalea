use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundMoveEntityPosRotPacket {
#[var]
pub entity_id: u32,
#[var]
pub xa: i32,
#[var]
pub ya: i32,
#[var]
pub za: i32,
#[var]
pub y_rot: i32,
#[var]
pub x_rot: i32,
pub on_ground: bool,
}