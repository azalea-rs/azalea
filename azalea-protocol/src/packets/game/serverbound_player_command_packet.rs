use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundPlayerCommandPacket {
    #[var]
    pub id: u32,
    pub action: Action,
    #[var]
    pub data: u32,
}

#[derive(McBuf, Clone, Copy, Debug)]
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
