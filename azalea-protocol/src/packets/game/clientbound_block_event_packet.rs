use azalea_buf::McBuf;
use azalea_core::BlockPos;
use packet_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundBlockEventPacket {
    pub pos: BlockPos,
    pub b0: u8,
    pub b1: u8,
    // TODO: this is a BlockState, see ClientboundBlockUpdatePacket for more info
    #[var]
    pub block: u32,
}
