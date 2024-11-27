use azalea_buf::AzBuf;
use azalea_core::resource_location::ResourceLocation;
use azalea_protocol_macros::ServerboundLoginPacket;

#[derive(Clone, Debug, AzBuf, ServerboundLoginPacket)]
pub struct ServerboundCookieResponse {
    pub key: ResourceLocation,
    pub payload: Option<Vec<u8>>,
}
