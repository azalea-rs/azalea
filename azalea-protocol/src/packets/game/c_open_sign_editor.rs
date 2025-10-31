use azalea_buf::AzBuf;
use azalea_core::position::BlockPos;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, PartialEq, Eq, ClientboundGamePacket)]
pub struct ClientboundOpenSignEditor {
    pub pos: BlockPos,
    pub is_front_text: bool,
}
