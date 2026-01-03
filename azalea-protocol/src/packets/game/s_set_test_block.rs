use azalea_buf::AzBuf;
use azalea_core::position::BlockPos;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(AzBuf, Clone, Debug, PartialEq, ServerboundGamePacket)]
pub struct ServerboundSetTestBlock {
    pub position: BlockPos,
    pub mode: TestBlockMode,
    pub message: String,
}

#[derive(AzBuf, Clone, Copy, Debug, Default, PartialEq)]
pub enum TestBlockMode {
    #[default]
    Start,
    Log,
    Fail,
    Accept,
}
