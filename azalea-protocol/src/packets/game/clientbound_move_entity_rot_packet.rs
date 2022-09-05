use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundMoveEntityRotPacket {
    #[var]
    pub entity_id: u32,
    pub y_rot: i8,
    pub x_rot: i8,
    pub on_ground: bool,
}
