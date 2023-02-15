use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundAddVibrationSignalPacket {
    // TODO: {'field': 'a.b', 'operation': 'write', 'type': 'position'}
    // TODO: {'field': 'gz.ak.b(a.c.a())', 'operation': 'write', 'type': 'identifier'}
    // TODO: {'args': 'packetbuffer, this.a.c', 'field': 'a.c.a()', 'method': 'a(Lpu;Lcsw;)V',
    // 'name': 'a', 'operation': 'interfacecall', 'target': 'csx', 'type': 'interface'}
    // TODO: {'field': 'a.d', 'operation': 'write', 'type': 'varint'}
}
