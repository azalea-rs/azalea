use azalea_buf::AzBuf;
use azalea_core::identifier::Identifier;
use azalea_protocol_macros::ClientboundConfigPacket;

#[derive(AzBuf, ClientboundConfigPacket, Clone, Debug, PartialEq)]
pub struct ClientboundStoreCookie {
    pub key: Identifier,
    pub payload: Vec<u8>,
}
