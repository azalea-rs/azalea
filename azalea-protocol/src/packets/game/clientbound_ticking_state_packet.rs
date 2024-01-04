use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundTickingStatePacket {
    pub tick_rate: f32,
    pub is_frozen: bool,
}
