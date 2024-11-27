use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundSetEntityMotion {
    #[var]
    pub id: u32,
    pub xa: i16,
    pub ya: i16,
    pub za: i16,
}
