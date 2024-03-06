use azalea_protocol_macros::ClientboundGamePacket;
use azalea_buf::McBuf;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundExplodePacket {
pub x: f64,
pub y: f64,
pub z: f64,
pub power: f32,
// TODO: {'operation': 'store', 'type': 'int', 'value': 'awh.a(this.b)', 'var': 'var2'}
// TODO: {'operation': 'store', 'type': 'int', 'value': 'awh.a(this.c)', 'var': 'var3'}
// TODO: {'operation': 'store', 'type': 'int', 'value': 'awh.a(this.d)', 'var': 'var4'}
pub to_blow: todo!(),
pub knockback_x: f32,
pub knockback_y: f32,
pub knockback_z: f32,
pub block_interaction: BlockInteraction,
}

#[derive(McBuf, Clone, Copy, Debug)]
pub enum BlockInteraction {
    Keep=0,
    Destroy=1,
    DestroyWithDecay=2,
    TriggerBlock=3,
}