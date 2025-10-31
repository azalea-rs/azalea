use azalea_buf::AzBuf;
use azalea_core::resource_location::ResourceLocation;
use azalea_protocol_macros::ClientboundLoginPacket;

#[derive(Clone, Debug, AzBuf, PartialEq, Eq, ClientboundLoginPacket)]
pub struct ClientboundCookieRequest {
    pub key: ResourceLocation,
}
