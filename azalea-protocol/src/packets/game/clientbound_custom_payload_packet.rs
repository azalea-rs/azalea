use azalea_buf::McBuf;
use azalea_buf::UnsizedByteArray;
use azalea_core::ResourceLocation;
use packet_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundCustomPayloadPacket {
    pub identifier: ResourceLocation,
    pub data: UnsizedByteArray,
}
