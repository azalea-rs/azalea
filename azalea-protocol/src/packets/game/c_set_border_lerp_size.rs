use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundSetBorderLerpSize {
    pub old_size: f64,
    pub new_size: f64,
    #[var]
    pub lerp_time: u64,
}
