use azalea_buf::McBuf;
use azalea_core::BlockPos;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundBlockDestructionPacket {
    /// The ID of the entity breaking the block.
    #[var]
    pub id: u32,
    pub pos: BlockPos,
    /// 0–9 to set it, any other value to remove it.
    pub progress: u8,
}
