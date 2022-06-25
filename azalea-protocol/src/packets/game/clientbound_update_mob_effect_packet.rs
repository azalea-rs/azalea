use azalea_buf::McBuf;
use packet_macros::GamePacket;

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundUpdateMobEffectPacket {
    #[var]
    pub entity_id: u32,
    // TODO: have an enum for this
    #[var]
    pub effect: u32,
    pub effect_amplifier: u8,
    #[var]
    pub effect_duration_ticks: u32,
    pub flags: u8,
    pub factor_data: Option<azalea_nbt::Tag>,
}
