use azalea_core::resource_location::ResourceLocation;
use azalea_protocol_macros::ServerboundConfigurationPacket;
use azalea_buf::McBuf;

#[derive(Clone, Debug, McBuf, ServerboundConfigurationPacket)]
pub struct ServerboundCookieResponsePacket {
pub key: ResourceLocation,
pub payload: Option<(u32, Vec<u8>)>,
}