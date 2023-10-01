use azalea_buf::McBuf;
use azalea_core::resource_location::ResourceLocation;
use azalea_protocol_macros::ClientboundConfigurationPacket;

#[derive(Clone, Debug, McBuf, ClientboundConfigurationPacket)]
pub struct ClientboundUpdateEnabledFeaturesPacket {
    pub features: Vec<ResourceLocation>,
}
