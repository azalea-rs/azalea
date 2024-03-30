use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundBlockEntityTagQueryPacket {
    #[var]
    pub transaction_id: u32,
    pub pos: u64, // TODO: Does BlockPos::asLong, may not be implemented
}
