use azalea_buf::AzBuf;
use azalea_entity::MobEffectData;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_registry::MobEffect;
use azalea_world::MinecraftEntityId;

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundGamePacket)]
pub struct ClientboundUpdateMobEffect {
    #[var]
    pub entity_id: MinecraftEntityId,
    pub mob_effect: MobEffect,
    pub data: MobEffectData,
}
