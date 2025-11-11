use azalea_buf::AzBuf;
use azalea_core::resource_location::Identifier;
use azalea_protocol_macros::ClientboundConfigPacket;

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundConfigPacket)]
pub struct ClientboundCookieRequest {
    pub key: Identifier,
}
