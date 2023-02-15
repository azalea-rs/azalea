use azalea_buf::McBuf;
use azalea_core::BlockPos;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundBlockBreakAckPacket {
    pub pos: BlockPos,
    // TODO: {'field': 'cdk.i(b)', 'operation': 'write', 'type': 'varint'}
    pub action: Action,
    pub all_good: bool,
}

#[derive(McBuf, Clone, Copy, Debug)]
pub enum Action {
    StartDestroyBlock = 0,
    AbortDestroyBlock = 1,
    StopDestroyBlock = 2,
    DropAllItems = 3,
    DropItem = 4,
    ReleaseUseItem = 5,
    SwapItemWithOffhand = 6,
}
