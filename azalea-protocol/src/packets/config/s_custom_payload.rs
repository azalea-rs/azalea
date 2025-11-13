use azalea_buf::{AzBuf, UnsizedByteArray};
use azalea_core::identifier::Identifier;
use azalea_protocol_macros::ServerboundConfigPacket;

#[derive(Clone, Debug, AzBuf, PartialEq, ServerboundConfigPacket)]
pub struct ServerboundCustomPayload {
    pub identifier: Identifier,
    pub data: UnsizedByteArray,
}
