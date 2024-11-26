use azalea_buf::McBuf;
use azalea_buf::UnsizedByteArray;
use azalea_core::resource_location::ResourceLocation;
use azalea_protocol_macros::ServerboundConfigPacket;

#[derive(Clone, Debug, McBuf, ServerboundConfigPacket)]
pub struct ServerboundCustomPayload {
    pub identifier: ResourceLocation,
    pub data: UnsizedByteArray,
}
