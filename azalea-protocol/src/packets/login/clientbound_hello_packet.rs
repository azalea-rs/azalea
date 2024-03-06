use azalea_protocol_macros::ClientboundLoginPacket;
use azalea_buf::McBuf;

#[derive(Clone, Debug, McBuf, ClientboundLoginPacket)]
pub struct ClientboundHelloPacket {
pub server_id: String,
// TODO: {'field': 'c.length', 'operation': 'write', 'type': 'varint'}
pub public_key: Vec<u8>,
// TODO: {'field': 'd.length', 'operation': 'write', 'type': 'varint'}
pub challenge: Vec<u8>,
pub should_authenticate: bool,
}