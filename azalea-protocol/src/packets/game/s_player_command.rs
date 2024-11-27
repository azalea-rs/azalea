use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, AzBuf, ServerboundGamePacket)]
pub struct ServerboundPlayerCommand {
    #[var]
    pub id: u32,
    pub action: Action,
    #[var]
    pub data: u32,
}

#[derive(AzBuf, Clone, Copy, Debug)]
pub enum Action {
    PressShiftKey = 0,
    ReleaseShiftKey = 1,
    StopSleeping = 2,
    StartSprinting = 3,
    StopSprinting = 4,
    StartRidingJump = 5,
    StopRidingJump = 6,
    OpenInventory = 7,
    StartFallFlying = 8,
}
