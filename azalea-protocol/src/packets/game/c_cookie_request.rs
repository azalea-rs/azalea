use azalea_buf::AzBuf;
use azalea_core::resource_location::Identifier;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundGamePacket)]
pub struct ClientboundCookieRequest {
    pub key: Identifier,
}
