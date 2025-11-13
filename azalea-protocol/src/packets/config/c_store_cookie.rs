use azalea_buf::AzBuf;
use azalea_core::identifier::Identifier;
use azalea_protocol_macros::ClientboundConfigPacket;

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundConfigPacket)]
pub struct ClientboundStoreCookie {
    pub key: Identifier,
    pub payload: Vec<u8>,
}
