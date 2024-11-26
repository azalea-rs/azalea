use azalea_buf::McBuf;
use azalea_core::resource_location::ResourceLocation;
use azalea_protocol_macros::ClientboundConfigPacket;

#[derive(Clone, Debug, McBuf, ClientboundConfigPacket)]
pub struct ClientboundUpdateEnabledFeatures {
    pub features: Vec<ResourceLocation>,
}
