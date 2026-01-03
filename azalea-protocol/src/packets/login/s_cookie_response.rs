use azalea_buf::AzBuf;
use azalea_registry::identifier::Identifier;
use azalea_protocol_macros::ServerboundLoginPacket;

#[derive(AzBuf, Clone, Debug, PartialEq, ServerboundLoginPacket)]
pub struct ServerboundCookieResponse {
    pub key: Identifier,
    pub payload: Option<Vec<u8>>,
}
