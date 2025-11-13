use azalea_buf::{AzBuf, UnsizedByteArray};
use azalea_core::identifier::Identifier;
use azalea_protocol_macros::ClientboundConfigPacket;

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundConfigPacket)]
pub struct ClientboundCustomPayload {
    pub identifier: Identifier,
    pub data: UnsizedByteArray,
}
