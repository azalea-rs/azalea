use azalea_buf::McBuf;
use azalea_core::BlockPos;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundBlockEntityTagQuery {
    #[var]
    pub transaction_id: i32,
    pub pos: BlockPos,
}
