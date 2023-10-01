use azalea_buf::McBuf;
use azalea_core::resource_location::ResourceLocation;
use azalea_entity::attributes::AttributeModifier;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundUpdateAttributesPacket {
    #[var]
    pub entity_id: u32,
    pub attributes: Vec<AttributeSnapshot>,
}

#[derive(Clone, Debug, McBuf)]
pub struct AttributeSnapshot {
    pub attribute: ResourceLocation,
    pub base: f64,
    pub modifiers: Vec<AttributeModifier>,
}
