use azalea_buf::McBuf;
use azalea_core::resource_location::ResourceLocation;
use azalea_protocol_macros::ServerboundConfigPacket;

#[derive(Clone, Debug, McBuf, ServerboundConfigPacket)]
pub struct ServerboundCookieResponse {
    pub key: ResourceLocation,
    pub payload: Option<Vec<u8>>,
}
