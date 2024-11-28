use azalea_buf::AzBuf;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, AzBuf, ServerboundGamePacket)]
pub struct ServerboundClientCommand {
    pub action: Action,
}

#[derive(AzBuf, Clone, Copy, Debug)]
pub enum Action {
    PerformRespawn = 0,
    RequestStats = 1,
}
