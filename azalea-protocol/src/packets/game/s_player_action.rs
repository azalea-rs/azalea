use azalea_buf::AzBuf;
use azalea_core::{direction::Direction, position::BlockPos};
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, AzBuf, PartialEq, ServerboundGamePacket)]
pub struct ServerboundPlayerAction {
    pub action: Action,
    pub pos: BlockPos,
    pub direction: Direction,
    #[var]
    pub seq: u32,
}

#[derive(AzBuf, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    StartDestroyBlock = 0,
    AbortDestroyBlock = 1,
    StopDestroyBlock = 2,
    DropAllItems = 3,
    DropItem = 4,
    ReleaseUseItem = 5,
    SwapItemWithOffhand = 6,
}
