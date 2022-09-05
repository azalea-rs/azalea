use azalea_buf::McBuf;
use azalea_core::BlockPos;
use azalea_core::Direction;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundPlayerActionPacket {
    pub action: Action,
    pub pos: BlockPos,
    pub direction: Direction,
    #[var]
    pub sequence: u32,
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
