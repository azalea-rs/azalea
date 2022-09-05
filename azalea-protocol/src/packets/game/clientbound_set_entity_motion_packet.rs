use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundSetEntityMotionPacket {
    #[var]
    pub id: u32,
    pub xa: i16,
    pub ya: i16,
    pub za: i16,
}
