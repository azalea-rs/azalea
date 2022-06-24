use azalea_buf::McBuf;
use azalea_core::BlockPos;
use packet_macros::GamePacket;

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundBlockUpdatePacket {
    pub pos: BlockPos,
    // TODO: in vanilla this is a BlockState, but here we just have it as a number.
    // however, we can't add azalea-world as a dependency because it depends on us.
    // we could have a crate that contains encoding/decoding and the definitions?
    #[var]
    pub block_state: u32,
}
