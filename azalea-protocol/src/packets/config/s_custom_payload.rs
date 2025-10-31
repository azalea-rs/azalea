use azalea_buf::{AzBuf, UnsizedByteArray};
use azalea_core::resource_location::ResourceLocation;
use azalea_protocol_macros::ServerboundConfigPacket;

#[derive(Clone, Debug, AzBuf, PartialEq, Eq, ServerboundConfigPacket)]
pub struct ServerboundCustomPayload {
    pub identifier: ResourceLocation,
    pub data: UnsizedByteArray,
}
