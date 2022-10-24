use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundExplodePacket {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub power: f32,
    // TODO: {'operation': 'store', 'type': 'int', 'value': 'anp.b(this.a)', 'var': 'var2'}
    // TODO: {'operation': 'store', 'type': 'int', 'value': 'anp.b(this.b)', 'var': 'var3'}
    // TODO: {'operation': 'store', 'type': 'int', 'value': 'anp.b(this.c)', 'var': 'var4'}
    pub to_blow: todo!(),
    pub knockback_x: f32,
    pub knockback_y: f32,
    pub knockback_z: f32,
}
