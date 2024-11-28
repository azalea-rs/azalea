use azalea_buf::AzBuf;
use azalea_core::resource_location::ResourceLocation;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundUpdateEnabledFeatures {
    pub features: Vec<ResourceLocation>,
}
