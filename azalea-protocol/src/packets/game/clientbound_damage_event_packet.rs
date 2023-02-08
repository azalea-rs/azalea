use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundDamageEventPacket {
    #[var]
    pub entity_id: u32,
    #[var]
    pub source_type_id: u32,
    // TODO: {'field': 'c + 1', 'operation': 'write', 'type': 'varint'}
    // TODO: {'field': 'd + 1', 'operation': 'write', 'type': 'varint'}
    pub source_position: Option<(f64, f64, f64)>,
}
