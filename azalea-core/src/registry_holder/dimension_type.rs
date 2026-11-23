use std::collections::HashMap;

use azalea_buf::AzBuf;
#[cfg(not(feature = "strict_registry"))]
use azalea_codec::Codec;
use azalea_registry::{
    HolderSet,
    builtin::{BlockKind, EnvironmentAttribute, SoundEvent},
    identifier::Identifier,
};
use indexmap::IndexMap;
use simdnbt::{Deserialize, FromNbtTag, Serialize, ToNbtTag, owned::NbtTag};

use crate::codec_utils::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "strict_registry", simdnbt(deny_unknown_fields))]
pub struct TrimMaterialElement {
    pub asset_name: String,
    pub ingredient: Identifier,
    pub item_model_index: f32,
    pub override_armor_materials: HashMap<String, String>,
    pub description: Option<String>,
}

/// Data about a kind of chat message
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "strict_registry", simdnbt(deny_unknown_fields))]
pub struct ChatTypeElement {
    pub chat: ChatTypeData,
    pub narration: ChatTypeData,
}

/// Data about a chat message.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "strict_registry", simdnbt(deny_unknown_fields))]
pub struct ChatTypeData {
    pub translation_key: String,
    pub parameters: Vec<String>,
    pub style: Option<ChatTypeStyle>,
}

/// The style of a chat message.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "strict_registry", simdnbt(deny_unknown_fields))]
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
    pub infiniburn: HolderSet<BlockKind, Identifier>,
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
    pub timelines: Option<HolderSet<azalea_registry::data::Timeline, Identifier>>,
    pub default_clock: azalea_registry::data::WorldClock,

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
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "strict_registry", simdnbt(deny_unknown_fields))]
pub struct WorldTypeElement {
    pub has_precipitation: bool,
    pub temperature: f32,
    pub temperature_modifier: Option<String>,
    pub downfall: f32,
    pub effects: BiomeEffects,
}

/// The precipitation of a biome.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BiomePrecipitation {
    None,
    Rain,
    Snow,
}
impl FromNbtTag for BiomePrecipitation {
    fn from_nbt_tag(tag: simdnbt::borrow::NbtTag) -> Option<Self> {
        match tag.string()?.to_str().as_ref() {
            "none" => Some(Self::None),
            "rain" => Some(Self::Rain),
            "snow" => Some(Self::Snow),
            _ => None,
        }
    }
}
impl ToNbtTag for BiomePrecipitation {
    fn to_nbt_tag(self) -> NbtTag {
        match self {
            Self::None => NbtTag::String("none".into()),
            Self::Rain => NbtTag::String("rain".into()),
            Self::Snow => NbtTag::String("snow".into()),
        }
    }
}

/// The effects of a biome.
///
/// This includes the sky, fog, water, and grass color,
/// as well as music and other sound effects.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "strict_registry", simdnbt(deny_unknown_fields))]
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
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "strict_registry", simdnbt(deny_unknown_fields))]
pub struct BiomeMusic {
    pub replace_current_music: bool,
    pub max_delay: u32,
    pub min_delay: u32,
    pub sound: SoundEvent,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "strict_registry", simdnbt(deny_unknown_fields))]
pub struct BiomeMoodSound {
    pub tick_delay: u32,
    pub block_search_extent: u32,
    pub offset: f32,
    pub sound: SoundEvent,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "strict_registry", simdnbt(deny_unknown_fields))]
pub struct AdditionsSound {
    pub tick_chance: f32,
    pub sound: SoundEvent,
}

/// Biome particles.
///
/// Some biomes have particles that spawn in the air.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "strict_registry", simdnbt(deny_unknown_fields))]
pub struct BiomeParticle {
    pub probability: f32,
    pub options: HashMap<String, String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "strict_registry", simdnbt(deny_unknown_fields))]
pub struct TrimPatternElement {
    #[simdnbt(flatten)]
    pub pattern: HashMap<String, String>,
}

#[derive(
    Debug, Clone, serde::Serialize, simdnbt::Serialize, simdnbt::Deserialize, AzBuf, PartialEq,
)]
#[cfg_attr(feature = "strict_registry", simdnbt(deny_unknown_fields))]
pub struct DamageTypeElement {
    pub message_id: String,
    pub scaling: String,
    pub exhaustion: f32,
    #[serde(skip_serializing_if = "is_default")]
    pub effects: Option<String>,
    #[serde(skip_serializing_if = "is_default")]
    pub death_message_type: Option<String>,
}
