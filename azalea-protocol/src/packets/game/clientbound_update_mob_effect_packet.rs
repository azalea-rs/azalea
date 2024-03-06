use azalea_protocol_macros::ClientboundGamePacket;
use azalea_buf::McBuf;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundUpdateMobEffectPacket {
#[var]
pub entity_id: u32,
#[var]
pub effect_amplifier: u32,
#[var]
pub effect_duration_ticks: u32,
#[var]
pub flags: u32,
}