use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(AzBuf, Clone, Debug, PartialEq, ServerboundGamePacket)]
pub struct ServerboundClientCommand {
    pub action: Action,
}

#[derive(AzBuf, Clone, Copy, Debug, PartialEq)]
pub enum Action {
    PerformRespawn = 0,
    RequestStats = 1,
}
