use azalea_buf::McBuf;
use azalea_core::position::BlockPos;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundSignUpdatePacket {
    pub pos: BlockPos,
    pub is_front_text: bool,
    pub lines: [String; 4],
}
