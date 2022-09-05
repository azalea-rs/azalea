use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundRemoveMobEffectPacket {
    #[var]
    pub entity_id: u32,
    pub effect: azalea_registry::MobEffect,
}
