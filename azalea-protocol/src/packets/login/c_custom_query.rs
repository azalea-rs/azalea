use std::hash::Hash;

use azalea_buf::{AzBuf, UnsizedByteArray};
use azalea_core::resource_location::ResourceLocation;
use azalea_protocol_macros::ClientboundLoginPacket;

#[derive(Hash, Clone, Debug, AzBuf, ClientboundLoginPacket)]
pub struct ClientboundCustomQuery {
    #[var]
    pub transaction_id: u32,
    pub identifier: ResourceLocation,
    pub data: UnsizedByteArray,
}
