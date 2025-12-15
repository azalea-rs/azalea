use azalea_buf::AzBuf;
use azalea_inventory::components::AttributeModifier;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_registry::builtin::Attribute;
use azalea_world::MinecraftEntityId;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundUpdateAttributes {
    #[var]
    pub entity_id: MinecraftEntityId,
    pub values: Vec<AttributeSnapshot>,
}

#[derive(AzBuf, Clone, Debug, PartialEq)]
pub struct AttributeSnapshot {
    pub attribute: Attribute,
    pub base: f64,
    pub modifiers: Vec<AttributeModifier>,
}
