use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundClientCommandPacket {
    pub action: Action,
}

#[derive(McBuf, Clone, Copy, Debug)]
pub enum Action {
    PerformRespawn = 0,
    RequestStats = 1,
}
