use azalea_buf::McBuf;
use azalea_chat::Component;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundSetActionBarTextPacket {
    pub text: Component,
}
