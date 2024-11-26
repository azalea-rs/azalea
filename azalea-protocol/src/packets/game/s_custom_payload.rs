use azalea_buf::McBuf;
use azalea_buf::UnsizedByteArray;
use azalea_core::resource_location::ResourceLocation;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundCustomPayload {
    pub identifier: ResourceLocation,
    pub data: UnsizedByteArray,
}
