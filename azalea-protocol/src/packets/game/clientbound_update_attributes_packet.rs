use azalea_buf::McBuf;
use azalea_core::ResourceLocation;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_world::entity::attributes::AttributeModifier;

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
