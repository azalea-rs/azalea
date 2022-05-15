use crate::mc_buf::UnsizedByteArray;
use azalea_core::resource_location::ResourceLocation;
use packet_macros::{LoginPacket, McBuf};
use std::hash::Hash;

#[derive(Hash, Clone, Debug, McBuf, LoginPacket)]
pub struct ClientboundCustomQueryPacket {
    #[var]
    pub transaction_id: u32,
    pub identifier: ResourceLocation,
    pub data: UnsizedByteArray,
}
