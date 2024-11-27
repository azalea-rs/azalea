use azalea_buf::AzBuf;
use azalea_core::resource_location::ResourceLocation;
use azalea_protocol_macros::ServerboundConfigPacket;

#[derive(Clone, Debug, AzBuf, ServerboundConfigPacket)]
pub struct ServerboundCookieResponse {
    pub key: ResourceLocation,
    pub payload: Option<Vec<u8>>,
}
