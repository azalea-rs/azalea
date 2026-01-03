use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundSetTitlesAnimation {
    pub fade_in: u32,
    pub stay: u32,
    pub fade_out: u32,
}
