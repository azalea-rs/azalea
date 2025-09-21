use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundGamePacket;
use azalea_world::MinecraftEntityId;

#[derive(Clone, Debug, AzBuf, PartialEq, ServerboundGamePacket)]
pub struct ServerboundPlayerCommand {
    #[var]
    pub id: MinecraftEntityId,
    pub action: Action,
    #[var]
    pub data: u32,
}

#[derive(AzBuf, Clone, Copy, Debug, PartialEq)]
pub enum Action {
    StopSleeping,
    StartSprinting,
    StopSprinting,
    StartRidingJump,
    StopRidingJump,
    OpenInventory,
    StartFallFlying,
}
