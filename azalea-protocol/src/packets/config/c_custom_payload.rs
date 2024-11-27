use azalea_buf::AzBuf;
use azalea_buf::UnsizedByteArray;
use azalea_core::resource_location::ResourceLocation;
use azalea_protocol_macros::ClientboundConfigPacket;

#[derive(Clone, Debug, AzBuf, ClientboundConfigPacket)]
pub struct ClientboundCustomPayload {
    pub identifier: ResourceLocation,
    pub data: UnsizedByteArray,
}
