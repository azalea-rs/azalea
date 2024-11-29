use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundSetTime {
    pub game_time: u64,
    pub day_time: u64,
    pub tick_day_time: bool,
}
