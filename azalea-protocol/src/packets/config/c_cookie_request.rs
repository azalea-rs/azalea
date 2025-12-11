use azalea_buf::AzBuf;
use azalea_registry::identifier::Identifier;
use azalea_protocol_macros::ClientboundConfigPacket;

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundConfigPacket)]
pub struct ClientboundCookieRequest {
    pub key: Identifier,
}
