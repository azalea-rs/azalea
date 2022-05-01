// i don't know the actual name of this packet, i couldn't find it in the source code

use crate::mc_buf::UnsizedByteArray;
use azalea_core::resource_location::ResourceLocation;
use packet_macros::GamePacket;

#[derive(Clone, Debug, GamePacket)]
pub struct ServerboundCustomPayloadPacket {
    pub identifier: ResourceLocation,
    pub data: UnsizedByteArray,
}
