use azalea_buf::McBuf;
use azalea_chat::component::Component;
use azalea_protocol_macros::ClientboundGamePacket;
use uuid::Uuid;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundPlayerChatPacket {
    pub sender: Uuid,
    #[var]
    pub index: u32,
    pub signature: Option<Vec<u8>>,
    // TODO: {'field': 'd.a', 'operation': 'write', 'type': 'string'}
    // TODO: {'field': 'd.b.toEpochMilli()', 'operation': 'write', 'type': 'long'}
    // TODO: {'field': 'd.c', 'operation': 'write', 'type': 'long'}
    pub body: todo!(),
    pub unsigned_content: Option<Component>,
    // TODO: {'field': 'f.f', 'operation': 'write', 'type': 'enum'}
    // TODO: {'condition': 'f.f == sl$a.c', 'instructions': [{'field': 'f.e.toLongArray().length', 'operation': 'write', 'type': 'varint'}, {'field': 'f.e.toLongArray()', 'operation': 'write', 'type': 'long[]'}], 'operation': 'if'}
    // TODO: {'field': 'g.a', 'operation': 'write', 'type': 'varint'}
    // TODO: {'field': 'g.b', 'operation': 'write', 'type': 'chatcomponent'}
    pub chat_type: Option<Component>,
}
