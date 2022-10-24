use azalea_buf::McBuf;
use azalea_chat::component::Component;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundDisguisedChatPacket {
    pub message: Component,
    // TODO: {'field': 'b.a', 'operation': 'write', 'type': 'varint'}
    // TODO: {'field': 'b.b', 'operation': 'write', 'type': 'chatcomponent'}
    pub chat_type: Option<Component>,
}
