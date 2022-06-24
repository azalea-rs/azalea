use azalea_buf::McBuf;
use azalea_buf::UnsizedByteArray;
use azalea_core::ResourceLocation;
use packet_macros::GamePacket;

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundCustomPayloadPacket {
    pub identifier: ResourceLocation,
    pub data: UnsizedByteArray,
}
