use azalea_core::resource_location::ResourceLocation;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_buf::McBuf;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundStoreCookiePacket {
pub key: ResourceLocation,
// TODO: {'field': 'd.length', 'operation': 'write', 'type': 'varint'}
pub payload: Vec<u8>,
}