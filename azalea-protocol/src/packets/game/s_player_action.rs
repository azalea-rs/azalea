use azalea_buf::AzBuf;
use azalea_core::direction::Direction;
use azalea_core::position::BlockPos;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, AzBuf, ServerboundGamePacket)]
pub struct ServerboundPlayerAction {
    pub action: Action,
    pub pos: BlockPos,
    pub direction: Direction,
    #[var]
    pub sequence: u32,
}

#[derive(AzBuf, Clone, Copy, Debug)]
pub enum Action {
    StartDestroyBlock = 0,
    AbortDestroyBlock = 1,
    StopDestroyBlock = 2,
    DropAllItems = 3,
    DropItem = 4,
    ReleaseUseItem = 5,
    SwapItemWithOffhand = 6,
}
