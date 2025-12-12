use azalea_buf::AzBuf;
use azalea_registry::identifier::Identifier;
use azalea_protocol_macros::ClientboundLoginPacket;

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundLoginPacket)]
pub struct ClientboundCookieRequest {
    pub key: Identifier,
}
