use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundHurtAnimation {
    #[var]
    pub id: u32,
    pub yaw: f32,
}
