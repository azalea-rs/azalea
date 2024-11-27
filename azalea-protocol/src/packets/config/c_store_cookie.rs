use azalea_buf::AzBuf;
use azalea_core::resource_location::ResourceLocation;
use azalea_protocol_macros::ClientboundConfigPacket;

#[derive(Clone, Debug, AzBuf, ClientboundConfigPacket)]
pub struct ClientboundStoreCookie {
    pub key: ResourceLocation,
    pub payload: Vec<u8>,
}
