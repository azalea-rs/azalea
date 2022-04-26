use crate::mc_buf::UnsizedByteArray;
use azalea_core::resource_location::ResourceLocation;
use packet_macros::GamePacket;

#[derive(Clone, Debug, GamePacket)]
pub struct ClientboundCustomPayloadPacket {
    pub identifier: ResourceLocation,
    pub data: UnsizedByteArray,
}
