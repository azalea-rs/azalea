use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundTickingState {
    pub tick_rate: f32,
    pub is_frozen: bool,
}
