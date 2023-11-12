use azalea_buf::McBuf;
use azalea_core::position::BlockPos;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundExplodePacket {
    pub x: BlockPos,
    pub y: BlockPos,
    pub z: BlockPos,
    pub power: f32,
    // TODO: {'operation': 'store', 'type': 'int', 'value': 'aty.a(this.a)', 'var': 'var2'}
    // TODO: {'operation': 'store', 'type': 'int', 'value': 'aty.a(this.b)', 'var': 'var3'}
    // TODO: {'operation': 'store', 'type': 'int', 'value': 'aty.a(this.c)', 'var': 'var4'}
    pub to_blow: todo!(),
    pub knockback_x: f32,
    pub knockback_y: f32,
    pub knockback_z: f32,
    pub block_interaction: BlockInteraction,
    // TODO: {'field': 'kb.j.getId(i.b())', 'operation': 'write', 'type': 'varint'}
    // TODO: {'field': 'kb.j.getId(j.b())', 'operation': 'write', 'type': 'varint'}
    // TODO: {'field': 'l.d', 'operation': 'write', 'type': 'identifier'}
    pub explosion_sound: Option<f32>,
}

#[derive(McBuf, Clone, Copy, Debug)]
pub enum BlockInteraction {
    Keep = 0,
    Destroy = 1,
    DestroyWithDecay = 2,
    TriggerBlock = 3,
}
