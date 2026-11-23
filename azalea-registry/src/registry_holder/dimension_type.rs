use std::collections::HashMap;

use azalea_buf::AzBuf;
use azalea_codec::Codec;
use azalea_core::identifier::Identifier;
use indexmap::IndexMap;
use simdnbt::owned::NbtTag;

use crate::{
    HolderSet,
    builtin::{BlockKind, EnvironmentAttribute, SoundEvent},
};

#[derive(Clone, Debug, Codec)]
pub struct TrimMaterialElement {
    pub asset_name: String,
    pub ingredient: Identifier,
    pub item_model_index: f32,
    pub override_armor_materials: HashMap<String, String>,
    pub description: Option<String>,
}

/// Data about a kind of chat message
#[derive(Clone, Debug, Codec)]
pub struct ChatTypeElement {
    pub chat: ChatTypeData,
    pub narration: ChatTypeData,
}

/// Data about a chat message.
#[derive(Clone, Debug, Codec)]
pub struct ChatTypeData {
    pub translation_key: String,
    pub parameters: Vec<String>,
    pub style: Option<ChatTypeStyle>,
}

/// The style of a chat message.
#[derive(Clone, Debug, Codec)]
pub struct ChatTypeStyle {
    pub color: Option<String>,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underlined: Option<bool>,
    pub strikethrough: Option<bool>,
    pub obfuscated: Option<bool>,
}

/// Dimension attributes.
#[derive(Clone, Debug, Codec, Default)]
pub struct DimensionKindElement {
    #[codec(optional)]
    pub has_fixed_time: bool,
    pub has_skylight: bool,
    pub has_ceiling: bool,
    pub has_ender_dragon_fight: bool,
    pub coordinate_scale: f32,
    pub min_y: i32,
    pub height: u32,
    pub logical_height: u32,
    pub infiniburn: HolderSet<BlockKind>,
    pub ambient_light: f32,
    #[codec(flatten)]
    pub monster_settings: MonsterSettings,
    #[codec(optional)]
    pub skybox: Option<String>,
    #[codec(optional)]
    pub cardinal_light: Option<CardinalLighting>,
    #[codec(optional)]
    pub attributes: EnvironmentAttributeMap,
    #[codec(optional)]
    pub timelines: Option<HolderSet<crate::data::Timeline>>,
    pub default_clock: crate::data::WorldClock,

    #[codec(flatten)]
    pub _extra: HashMap<String, NbtTag>,
}

#[derive(Clone, Debug, Codec)]
#[codec(default_snake_case_key)]
pub enum CardinalLighting {
    Default,
    Nether,
}
#[derive(Clone, Debug, Codec, Default)]
#[codec(transparent)]
pub struct EnvironmentAttributeMap {
    pub values: IndexMap<EnvironmentAttribute, EnvironmentAttributeEntry>,
}
azalea_codec::impl_defaultable!(EnvironmentAttributeMap, self, self.values.is_empty());

/// Unimplemented.
#[derive(Clone, Debug, Codec)]
#[codec(allow_unknown_keys)]
pub struct EnvironmentAttributeEntry {}

#[derive(Clone, Debug, Codec, Default)]
pub struct MonsterSettings {
    pub monster_spawn_light_level: IntProvider,
    pub monster_spawn_block_light_limit: u8,
}

/// Unimplemented.
#[derive(Clone, Debug, Codec, Default)]
#[codec(allow_unknown_keys)]
pub struct IntProvider {}

/// Biome attributes.
#[derive(Clone, Debug, Codec)]
pub struct WorldTypeElement {
    pub has_precipitation: bool,
    pub temperature: f32,
    pub temperature_modifier: Option<String>,
    pub downfall: f32,
    pub effects: BiomeEffects,
}

/// The effects of a biome.
///
/// This includes the sky, fog, water, and grass color,
/// as well as music and other sound effects.
#[derive(Clone, Debug, Codec)]
pub struct BiomeEffects {
    pub sky_color: u32,
    pub fog_color: u32,
    pub water_color: u32,
    pub water_fog_color: u32,
    pub foliage_color: Option<u32>,
    pub grass_color: Option<u32>,
    pub grass_color_modifier: Option<String>,
    pub music: Option<BiomeMusic>,
    pub mood_sound: BiomeMoodSound,
    pub additions_sound: Option<AdditionsSound>,
    pub ambient_sound: Option<Identifier>,
    pub particle: Option<BiomeParticle>,
}

/// The music of the biome.
///
/// Some biomes have unique music that only play when inside them.
#[derive(Clone, Debug, Codec)]
pub struct BiomeMusic {
    pub replace_current_music: bool,
    pub max_delay: u32,
    pub min_delay: u32,
    pub sound: SoundEvent,
}

#[derive(Clone, Debug, Codec)]
pub struct BiomeMoodSound {
    pub tick_delay: u32,
    pub block_search_extent: u32,
    pub offset: f32,
    pub sound: SoundEvent,
}

#[derive(Clone, Debug, Codec)]
pub struct AdditionsSound {
    pub tick_chance: f32,
    pub sound: SoundEvent,
}

/// Biome particles.
///
/// Some biomes have particles that spawn in the air.
#[derive(Clone, Debug, Codec)]
pub struct BiomeParticle {
    pub probability: f32,
    pub options: HashMap<String, String>,
}

#[derive(Clone, Debug, Codec)]
pub struct TrimPatternElement {
    #[codec(flatten)]
    pub pattern: HashMap<String, String>,
}

#[derive(Debug, Clone, AzBuf, PartialEq, Codec)]
pub struct DamageTypeElement {
    pub message_id: String,
    pub scaling: String,
    pub exhaustion: f32,
    #[codec(optional)]
    pub effects: Option<String>,
    #[codec(optional)]
    pub death_message_type: Option<String>,
}
