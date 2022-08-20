use azalea_buf::McBuf;
use azalea_chat::component::Component;
use packet_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundSetActionBarTextPacket {
    pub text: Component,
}
