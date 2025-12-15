use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundSetHealth {
    pub health: f32,
    #[var]
    pub food: u32,
    pub saturation: f32,
}
