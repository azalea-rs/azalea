use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundSetCommandBlockPacket {
pub pos: u64, // TODO: Does BlockPos::asLong, may not be implemented
pub command: String,
pub mode: Mode,
// TODO: {'operation': 'store', 'type': 'int', 'value': '0', 'var': 'var2'}
// TODO: {'condition': 'g', 'instructions': [{'operation': 'store', 'type': 'int', 'value': '(var2 | 1)', 'var': 'var2'}], 'operation': 'if'}
// TODO: {'condition': 'h', 'instructions': [{'operation': 'store', 'type': 'int', 'value': '(var2 | 2)', 'var': 'var2'}], 'operation': 'if'}
// TODO: {'condition': 'i', 'instructions': [{'operation': 'store', 'type': 'int', 'value': '(var2 | 4)', 'var': 'var2'}], 'operation': 'if'}
// TODO: unknown field {'field': 'var2', 'operation': 'write', 'type': 'byte'}
}

#[derive(McBuf, Clone, Copy, Debug)]
pub enum Mode {
    Sequence=0,
    Auto=1,
    Redstone=2,
}