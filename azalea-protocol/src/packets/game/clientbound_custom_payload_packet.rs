use crate::mc_buf::{Readable, Writable};
use azalea_core::resource_location::ResourceLocation;
use packet_macros::GamePacket;

#[derive(Clone, Debug, GamePacket)]
pub struct ClientboundCustomPayloadPacket {
    pub identifier: ResourceLocation,
    pub data: Vec<u8>,
}
