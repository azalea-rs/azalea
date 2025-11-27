use azalea_buf::{AzBuf, UnsizedByteArray};
use azalea_core::identifier::Identifier;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundGamePacket)]
pub struct ClientboundCustomPayload {
    pub identifier: Identifier,
    pub data: UnsizedByteArray,
}
