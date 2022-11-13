use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundRotateHeadPacket {
    #[var]
    pub entity_id: u32,
    pub y_head_rot: i8,
}
