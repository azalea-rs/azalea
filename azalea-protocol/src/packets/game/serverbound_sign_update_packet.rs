use azalea_buf::McBuf;
use azalea_core::BlockPos;
use packet_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundSignUpdatePacket {
    pub pos: BlockPos,
    pub lines: [String; 4],
}
