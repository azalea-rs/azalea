use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundPlayerActionPacket {
pub action: Action,
pub pos: u64, // TODO: Does BlockPos::asLong, may not be implemented
pub direction: u8, // TODO: Does Direction::get3DDataValue, may not be implemented
#[var]
pub sequence: u32,
}

#[derive(McBuf, Clone, Copy, Debug)]
pub enum Action {
    StartDestroyBlock=0,
    AbortDestroyBlock=1,
    StopDestroyBlock=2,
    DropAllItems=3,
    DropItem=4,
    ReleaseUseItem=5,
    SwapItemWithOffhand=6,
}