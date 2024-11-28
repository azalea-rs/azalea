use azalea_buf::AzBuf;
use azalea_core::position::BlockPos;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, AzBuf, ServerboundGamePacket)]
pub struct ServerboundJigsawGenerate {
    pub pos: BlockPos,
    #[var]
    pub levels: u32,
    pub keep_jigsaws: bool,
}
