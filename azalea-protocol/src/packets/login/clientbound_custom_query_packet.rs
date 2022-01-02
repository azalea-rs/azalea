use azalea_core::resource_location::ResourceLocation;
use packet_macros::LoginPacket;
use std::hash::Hash;

#[derive(Hash, Clone, Debug, LoginPacket)]
pub struct ClientboundCustomQueryPacket {
    #[varint]
    pub transaction_id: u32,
    pub identifier: ResourceLocation,
    pub data: Vec<u8>,
}
