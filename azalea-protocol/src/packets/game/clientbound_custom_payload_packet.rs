use crate::mc_buf::UnsizedByteArray;
use azalea_core::resource_location::ResourceLocation;
use packet_macros::{GamePacket, McBuf};

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundCustomPayloadPacket {
    pub identifier: ResourceLocation,
    pub data: UnsizedByteArray,
}
