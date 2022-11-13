use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundAddExperienceOrbPacket {
    #[var]
    pub id: u32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub value: u16,
}
