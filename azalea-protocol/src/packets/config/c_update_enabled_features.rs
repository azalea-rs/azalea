use azalea_buf::AzBuf;
use azalea_core::resource_location::ResourceLocation;
use azalea_protocol_macros::ClientboundConfigPacket;

#[derive(Clone, Debug, AzBuf, PartialEq, Eq, ClientboundConfigPacket)]
pub struct ClientboundUpdateEnabledFeatures {
    pub features: Vec<ResourceLocation>,
}
