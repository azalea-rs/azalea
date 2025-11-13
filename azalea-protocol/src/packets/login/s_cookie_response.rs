use azalea_buf::AzBuf;
use azalea_core::identifier::Identifier;
use azalea_protocol_macros::ServerboundLoginPacket;

#[derive(Clone, Debug, AzBuf, PartialEq, ServerboundLoginPacket)]
pub struct ServerboundCookieResponse {
    pub key: Identifier,
    pub payload: Option<Vec<u8>>,
}
