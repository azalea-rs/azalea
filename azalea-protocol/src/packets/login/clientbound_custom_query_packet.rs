use azalea_buf::{McBuf, UnsizedByteArray};
use azalea_core::ResourceLocation;
use azalea_protocol_macros::ClientboundLoginPacket;
use std::hash::Hash;

#[derive(Hash, Clone, Debug, McBuf, ClientboundLoginPacket)]
pub struct ClientboundCustomQueryPacket {
    #[var]
    pub transaction_id: u32,
    pub identifier: ResourceLocation,
    pub data: UnsizedByteArray,
}
