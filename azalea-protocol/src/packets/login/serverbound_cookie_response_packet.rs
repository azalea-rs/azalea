use azalea_buf::McBuf;
use azalea_core::resource_location::ResourceLocation;
use azalea_protocol_macros::ServerboundLoginPacket;

#[derive(Clone, Debug, McBuf, ServerboundLoginPacket)]
pub struct ServerboundCookieResponsePacket {
    pub key: ResourceLocation,
    pub payload: Option<Vec<u8>>,
}
