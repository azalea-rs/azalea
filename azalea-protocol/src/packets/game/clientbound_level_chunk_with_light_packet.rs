use azalea_protocol_macros::ClientboundGamePacket;
use azalea_buf::McBuf;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundLevelChunkWithLightPacket {
#[var]
pub x: i32,
#[var]
pub z: i32,
// TODO: {'field': 'e.a.toLongArray().length', 'operation': 'write', 'type': 'varint'}
// TODO: {'field': 'e.a.toLongArray()', 'operation': 'write', 'type': 'long[]'}
// TODO: {'field': 'e.b.toLongArray().length', 'operation': 'write', 'type': 'varint'}
// TODO: {'field': 'e.b.toLongArray()', 'operation': 'write', 'type': 'long[]'}
// TODO: {'field': 'e.c.toLongArray().length', 'operation': 'write', 'type': 'varint'}
// TODO: {'field': 'e.c.toLongArray()', 'operation': 'write', 'type': 'long[]'}
// TODO: {'field': 'e.d.toLongArray().length', 'operation': 'write', 'type': 'varint'}
// TODO: {'field': 'e.d.toLongArray()', 'operation': 'write', 'type': 'long[]'}
pub light_data: todo!(),
pub light_data: todo!(),
}