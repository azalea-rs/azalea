use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundChatSessionUpdatePacket {
    // TODO: {'field': 'a.a', 'operation': 'write', 'type': 'uuid'}
    // TODO: {'field': 'a.b.b.toEpochMilli()', 'operation': 'write', 'type': 'long'}
    // TODO: {'field': 'a.b.c.getEncoded().length', 'operation': 'write', 'type': 'varint'}
    // TODO: {'field': 'a.b.c.getEncoded()', 'operation': 'write', 'type': 'byte[]'}
    // TODO: {'field': 'a.b.d.length', 'operation': 'write', 'type': 'varint'}
    // TODO: {'field': 'a.b.d', 'operation': 'write', 'type': 'byte[]'}
}
