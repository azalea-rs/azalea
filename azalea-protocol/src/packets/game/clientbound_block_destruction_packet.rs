use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundBlockDestructionPacket {
    #[var]
    pub id: u32,
    pub pos: u64, // TODO: Does BlockPos::asLong, may not be implemented
    pub progress: u8,
}
