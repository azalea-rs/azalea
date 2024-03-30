use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundOpenSignEditorPacket {
    pub pos: u64, // TODO: Does BlockPos::asLong, may not be implemented
    pub is_front_text: bool,
}
