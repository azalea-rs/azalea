use azalea_buf::AzBuf;
use azalea_registry::identifier::Identifier;
use azalea_protocol_macros::ServerboundConfigPacket;

#[derive(AzBuf, Clone, Debug, PartialEq, ServerboundConfigPacket)]
pub struct ServerboundCookieResponse {
    pub key: Identifier,
    pub payload: Option<Vec<u8>>,
}
