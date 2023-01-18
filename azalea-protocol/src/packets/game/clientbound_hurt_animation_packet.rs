use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundHurtAnimationPacket {
    #[var]
    pub id: u32,
    pub yaw: f32,
}
