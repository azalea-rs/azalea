use azalea_buf::AzBuf;
use azalea_core::position::BlockPos;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, AzBuf, PartialEq, Eq, ServerboundGamePacket)]
pub struct ServerboundSetTestBlock {
    pub position: BlockPos,
    pub mode: TestBlockMode,
    pub message: String,
}

#[derive(Clone, Copy, Debug, AzBuf, Default, PartialEq, Eq)]
pub enum TestBlockMode {
    #[default]
    Start,
    Log,
    Fail,
    Accept,
}
