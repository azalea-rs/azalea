use azalea_protocol_macros::ClientboundConfigurationPacket;
use azalea_buf::McBuf;

#[derive(Clone, Debug, McBuf, ClientboundConfigurationPacket)]
pub struct ClientboundRegistryDataPacket {
// TODO: {'field': 'hw.a.encode(wj.b, a)', 'operation': 'write', 'type': 'nbtcompound'}
}