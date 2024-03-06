use azalea_protocol_macros::ClientboundGamePacket;
use azalea_buf::McBuf;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundSetObjectivePacket {
pub objective_name: String,
#[var]
pub method: u32,
// TODO: {'condition': 'i', 'instructions': [{'field': 'f', 'operation': 'write', 'type': 'chatcomponent'}, {'field': 'g', 'operation': 'write', 'type': 'enum'}], 'operation': 'if'}
}