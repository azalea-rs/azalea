use azalea_core::resource_location::ResourceLocation;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_buf::McBuf;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundRespawnPacket {
// TODO: {'field': 'e.a.a()', 'operation': 'write', 'type': 'identifier'}
// TODO: {'field': 'e.b.a()', 'operation': 'write', 'type': 'identifier'}
// TODO: {'field': 'e.c', 'operation': 'write', 'type': 'long'}
// TODO: {'field': 'e.d.a()', 'operation': 'write', 'type': 'byte'}
// TODO: {'field': 'cvk.a(e.e)', 'operation': 'write', 'type': 'byte'}
// TODO: {'field': 'e.f', 'operation': 'write', 'type': 'boolean'}
// TODO: {'field': 'e.g', 'operation': 'write', 'type': 'boolean'}
pub common_player_spawn_info: Option<(ResourceLocation, u64)>,
// TODO: {'field': 'e.i', 'operation': 'write', 'type': 'varint'}
pub data_to_keep: u8,
}