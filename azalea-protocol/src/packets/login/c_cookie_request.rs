use azalea_buf::AzBuf;
use azalea_core::identifier::Identifier;
use azalea_protocol_macros::ClientboundLoginPacket;

#[derive(AzBuf, ClientboundLoginPacket, Clone, Debug, PartialEq)]
pub struct ClientboundCookieRequest {
    pub key: Identifier,
}
