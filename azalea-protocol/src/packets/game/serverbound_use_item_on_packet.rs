use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundUseItemOnPacket {
pub hand: InteractionHand,
// TODO: {'operation': 'store', 'type': 'Object', 'value': 'this.b.a()', 'var': 'var2'}
pub var2: u64, // TODO: Does TODO::a, may not be implemented
pub block_hit: Direction, // TODO: Does BlockHitResult::getDirection, may not be implemented
// TODO: {'operation': 'store', 'type': 'Object', 'value': 'this.b.e()', 'var': 'var3'}
// TODO: {'field': '(float)(var3.c - ((double)var2.u()))', 'operation': 'write', 'type': 'float'}
// TODO: {'field': '(float)(var3.d - ((double)var2.v()))', 'operation': 'write', 'type': 'float'}
// TODO: {'field': '(float)(var3.e - ((double)var2.w()))', 'operation': 'write', 'type': 'float'}
pub block_hit: bool, // TODO: Does BlockHitResult::isInside, may not be implemented
#[var]
pub sequence: u32,
}

#[derive(McBuf, Clone, Copy, Debug)]
pub enum InteractionHand {
    MainHand=0,
    OffHand=1,
}

#[derive(McBuf, Clone, Copy, Debug)]
pub enum Direction {
    Down=0,
    Up=1,
    North=2,
    South=3,
    West=4,
    East=5,
}