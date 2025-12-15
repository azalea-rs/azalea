use azalea_buf::AzBuf;
use azalea_core::position::BlockPos;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_world::MinecraftEntityId;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundBlockDestruction {
    /// The ID of the entity breaking the block.
    #[var]
    pub id: MinecraftEntityId,
    pub pos: BlockPos,
    /// 0–9 to set it, any other value to remove it.
    pub progress: u8,
}
