use azalea_buf::McBuf;
use azalea_core::position::BlockPos;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundOpenSignEditorPacket {
    pub pos: BlockPos,
    pub is_front_text: bool,
}
