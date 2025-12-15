use azalea_buf::AzBuf;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_registry::builtin::MobEffect;
use azalea_world::MinecraftEntityId;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundRemoveMobEffect {
    #[var]
    pub entity_id: MinecraftEntityId,
    pub effect: MobEffect,
}
