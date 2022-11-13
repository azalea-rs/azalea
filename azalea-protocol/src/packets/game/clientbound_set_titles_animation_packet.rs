use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundSetTitlesAnimationPacket {
    pub fade_in: u32,
    pub stay: u32,
    pub fade_out: u32,
}
