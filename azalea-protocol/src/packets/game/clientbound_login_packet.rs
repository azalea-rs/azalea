use azalea_core::resource_location::ResourceLocation;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_buf::McBuf;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundLoginPacket {
pub player_id: u32,
pub hardcore: bool,
pub levels: Vec<ResourceLocation>,
#[var]
pub max_players: u32,
#[var]
pub chunk_radius: u32,
#[var]
pub simulation_distance: u32,
pub reduced_debug_info: bool,
pub show_death_screen: bool,
pub do_limited_crafting: bool,
// TODO: {'field': 'k.a.a()', 'operation': 'write', 'type': 'identifier'}
// TODO: {'field': 'k.b.a()', 'operation': 'write', 'type': 'identifier'}
// TODO: {'field': 'k.c', 'operation': 'write', 'type': 'long'}
// TODO: {'field': 'k.d.a()', 'operation': 'write', 'type': 'byte'}
// TODO: {'field': 'cvk.a(k.e)', 'operation': 'write', 'type': 'byte'}
// TODO: {'field': 'k.f', 'operation': 'write', 'type': 'boolean'}
// TODO: {'field': 'k.g', 'operation': 'write', 'type': 'boolean'}
pub common_player_spawn_info: Option<(ResourceLocation, u64)>,
// TODO: {'field': 'k.i', 'operation': 'write', 'type': 'varint'}
pub enforces_secure_chat: bool,
}