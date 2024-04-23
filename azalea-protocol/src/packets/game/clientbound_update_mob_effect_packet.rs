use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_registry::MobEffect;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundUpdateMobEffectPacket {
    #[var]
    pub entity_id: u32,
    pub mob_effect: MobEffect,
    #[var]
    pub effect_amplifier: u32,
    #[var]
    pub effect_duration_ticks: u32,
    pub flags: u8,
}
