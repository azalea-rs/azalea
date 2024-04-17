use azalea_buf::McBuf;
use azalea_entity::attributes::AttributeModifier;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_registry::Attribute;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundUpdateAttributesPacket {
    #[var]
    pub entity_id: u32,
    pub values: Vec<AttributeSnapshot>,
}

#[derive(Clone, Debug, McBuf)]
pub struct AttributeSnapshot {
    pub attribute: Attribute,
    pub base: f64,
    pub modifiers: Vec<AttributeModifier>,
}
