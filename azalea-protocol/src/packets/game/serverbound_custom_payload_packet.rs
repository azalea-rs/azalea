use azalea_buf::McBuf;
use azalea_buf::UnsizedByteArray;
use azalea_core::ResourceLocation;
use packet_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundCustomPayloadPacket {
    pub identifier: ResourceLocation,
    pub data: UnsizedByteArray,
}
