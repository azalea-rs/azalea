use std::hash::Hash;

use azalea_buf::{AzBuf, UnsizedByteArray};
use azalea_registry::identifier::Identifier;
use azalea_protocol_macros::ClientboundLoginPacket;

#[derive(AzBuf, ClientboundLoginPacket, Clone, Debug, Hash, PartialEq)]
pub struct ClientboundCustomQuery {
    #[var]
    pub transaction_id: u32,
    pub identifier: Identifier,
    pub data: UnsizedByteArray,
}
