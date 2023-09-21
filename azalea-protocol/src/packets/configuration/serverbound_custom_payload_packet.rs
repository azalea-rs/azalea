use azalea_buf::McBuf;
use azalea_buf::UnsizedByteArray;
use azalea_core::ResourceLocation;
use azalea_protocol_macros::ServerboundConfigurationPacket;

#[derive(Clone, Debug, McBuf, ServerboundConfigurationPacket)]
pub struct ServerboundCustomPayloadPacket {
    pub identifier: ResourceLocation,
    pub data: UnsizedByteArray,
}
