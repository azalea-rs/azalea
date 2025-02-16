use azalea_buf::AzBuf;
use azalea_entity::attributes::AttributeModifier;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_registry::Attribute;
use azalea_world::MinecraftEntityId;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundUpdateAttributes {
    #[var]
    pub entity_id: MinecraftEntityId,
    pub values: Vec<AttributeSnapshot>,
}

#[derive(Clone, Debug, AzBuf)]
pub struct AttributeSnapshot {
    pub attribute: Attribute,
    pub base: f64,
    pub modifiers: Vec<AttributeModifier>,
}
