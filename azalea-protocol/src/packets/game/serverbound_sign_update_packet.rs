use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundSignUpdatePacket {
    pub pos: u64, // TODO: Does BlockPos::asLong, may not be implemented
    pub is_front_text: bool,
    // TODO: {'operation': 'store', 'type': 'int', 'value': '0', 'var': 'var2'}
    // TODO: {'condition': 'var2 < 4', 'instructions': [{'field': 'd[var2]', 'operation': 'write',
    // 'type': 'string'}, {'amount': '1', 'field': 'var2', 'operation': 'increment'}], 'operation':
    // 'loop'}
}
