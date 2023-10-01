use azalea_buf::McBuf;
use azalea_buf::UnsizedByteArray;
use azalea_core::resource_location::ResourceLocation;
use azalea_protocol_macros::ClientboundConfigurationPacket;

#[derive(Clone, Debug, McBuf, ClientboundConfigurationPacket)]
pub struct ClientboundCustomPayloadPacket {
    pub identifier: ResourceLocation,
    pub data: UnsizedByteArray,
}
