use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundMoveEntityRot {
    #[var]
    pub entity_id: u32,
    pub y_rot: i8,
    pub x_rot: i8,
    pub on_ground: bool,
}
