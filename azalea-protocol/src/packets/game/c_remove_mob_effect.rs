use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_world::MinecraftEntityId;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundRemoveMobEffect {
    #[var]
    pub entity_id: MinecraftEntityId,
    pub effect: azalea_registry::MobEffect,
}
