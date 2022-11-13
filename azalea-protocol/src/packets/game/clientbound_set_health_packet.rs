use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundSetHealthPacket {
    pub health: f32,
    #[var]
    pub food: u32,
    pub saturation: f32,
}
