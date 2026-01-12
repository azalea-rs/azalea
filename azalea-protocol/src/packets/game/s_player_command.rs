use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundGamePacket;
use azalea_core::entity_id::MinecraftEntityId;

#[derive(AzBuf, Clone, Debug, PartialEq, ServerboundGamePacket)]
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
