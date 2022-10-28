use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundSetBorderLerpSizePacket {
    pub old_size: f64,
    pub new_size: f64,
    #[var]
    pub lerp_time: u64,
}
