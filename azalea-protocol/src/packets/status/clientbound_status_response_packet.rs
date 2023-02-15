use azalea_protocol_macros::ClientboundStatusPacket;
use azalea_buf::McBuf;

#[derive(Clone, Debug, McBuf, ClientboundStatusPacket)]
pub struct ClientboundStatusResponsePacket {
// TODO: {'field': 'acd.a.encode(a)', 'operation': 'write', 'type': 'nbtcompound'}
}