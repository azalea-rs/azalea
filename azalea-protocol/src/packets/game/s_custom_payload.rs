use azalea_buf::{AzBuf, UnsizedByteArray};
use azalea_core::identifier::Identifier;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(AzBuf, Clone, Debug, PartialEq, ServerboundGamePacket)]
pub struct ServerboundCustomPayload {
    pub identifier: Identifier,
    pub data: UnsizedByteArray,
}
