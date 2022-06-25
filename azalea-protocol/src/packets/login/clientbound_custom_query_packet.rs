use azalea_buf::{McBuf, UnsizedByteArray};
use azalea_core::ResourceLocation;
use packet_macros::LoginPacket;
use std::hash::Hash;

#[derive(Hash, Clone, Debug, McBuf, LoginPacket)]
pub struct ClientboundCustomQueryPacket {
    #[var]
    pub transaction_id: u32,
    pub identifier: ResourceLocation,
    pub data: UnsizedByteArray,
}
