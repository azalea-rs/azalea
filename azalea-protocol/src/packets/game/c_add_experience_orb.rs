use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundAddExperienceOrb {
    #[var]
    pub id: u32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub value: u16,
}
