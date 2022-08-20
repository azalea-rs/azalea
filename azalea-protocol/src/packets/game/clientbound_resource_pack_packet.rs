use azalea_buf::McBuf;
use packet_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundResourcePackPacket {
    pub url: String,
    pub hash: String,
    pub required: bool,
    pub prompt: Option<Component>,
}
