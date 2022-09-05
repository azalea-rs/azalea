use azalea_buf::McBuf;
use azalea_core::BlockPos;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundBlockUpdatePacket {
    pub pos: BlockPos,
    // TODO: in vanilla this is a BlockState, but here we just have it as a number.
    // perhaps we could make a crate that only handles block states? right now blockstates are handled in azalea-block
    #[var]
    pub block_state: u32,
}
