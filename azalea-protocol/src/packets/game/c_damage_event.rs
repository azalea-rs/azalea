use azalea_buf::AzBuf;
use azalea_core::{
    entity_id::{MinecraftEntityId, OptionalEntityId},
    position::Vec3,
};
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundDamageEvent {
    #[var]
    pub entity_id: MinecraftEntityId,
    #[var]
    pub source_type_id: u32,
    pub source_cause_id: OptionalEntityId,
    pub source_direct_id: OptionalEntityId,
    pub source_position: Option<Vec3>,
}
