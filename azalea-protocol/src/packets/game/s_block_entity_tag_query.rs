use azalea_buf::AzBuf;
use azalea_core::position::BlockPos;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, AzBuf, ServerboundGamePacket)]
pub struct ServerboundBlockEntityTagQuery {
    #[var]
    pub transaction_id: u32,
    pub pos: BlockPos,
}
