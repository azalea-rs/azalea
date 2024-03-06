use azalea_protocol_macros::ClientboundGamePacket;
use azalea_buf::McBuf;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundSetEquipmentPacket {
#[var]
pub entity: u32,
// TODO: {'operation': 'store', 'type': 'int', 'value': 'this.d.size()', 'var': 'var2'}
// TODO: {'operation': 'store', 'type': 'int', 'value': '0', 'var': 'var3'}
// TODO: {'condition': 'var3 < var2', 'instructions': [{'operation': 'store', 'type': 'Object', 'value': '((com.mojang.datafixers.util.Pair)this.d.get(var3))', 'var': 'var4'}, {'operation': 'store', 'type': 'Object', 'value': '((bnv)var4.getFirst())', 'var': 'var5'}, {'operation': 'store', 'type': 'int', 'value': '((var3 != (var2 - 1)) ? 1 : 0)', 'var': 'var6'}, {'operation': 'store', 'type': 'int', 'value': 'var5.ordinal()', 'var': 'var7'}, {'field': '(var6) ? (var7 | -128) : var7', 'operation': 'write', 'type': 'varint'}, {'amount': '1', 'field': 'var3', 'operation': 'increment'}], 'operation': 'loop'}
}