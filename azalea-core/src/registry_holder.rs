//! The data sent to the client in the `ClientboundRegistryDataPacket`.
//!
//! This module contains the structures used to represent the registry
//! sent to the client upon login. This contains a lot of information about
//! the game, including the types of chat messages, dimensions, and
//! biomes.

use azalea_buf::{BufReadError, McBufReadable, McBufWritable};
use azalea_nbt::Nbt;
use serde::{
    de::{self, DeserializeOwned},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::{collections::HashMap, io::Cursor};

use crate::resource_location::ResourceLocation;

/// The base of the registry.
///
/// This is the registry that is sent to the client upon login.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct RegistryHolder {
    pub map: HashMap<ResourceLocation, Nbt>,
}

impl RegistryHolder {
    fn get<T: DeserializeOwned>(&self, name: &ResourceLocation) -> Option<T> {
        let nbt = self.map.get(name)?;
        serde_json::from_value(serde_json::to_value(nbt).ok()?).ok()
    }

    /// Get the dimension type registry, or `None` if it doesn't exist. You
    /// should do some type of error handling if this returns `None`.
    pub fn dimension_type(&self) -> Option<RegistryType<DimensionTypeElement>> {
        self.get(&ResourceLocation::new("minecraft:dimension_type"))
    }
}

impl TryFrom<Nbt> for RegistryHolder {
    type Error = serde_json::Error;

    fn try_from(value: Nbt) -> Result<Self, Self::Error> {
        Ok(RegistryHolder {
            map: serde_json::from_value(serde_json::to_value(value)?)?,
        })
    }
}

impl TryInto<Nbt> for RegistryHolder {
    type Error = serde_json::Error;

    fn try_into(self) -> Result<Nbt, Self::Error> {
        serde_json::from_value(serde_json::to_value(self.map)?)
    }
}

impl McBufReadable for RegistryHolder {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        RegistryHolder::try_from(Nbt::read_from(buf)?)
            .map_err(|e| BufReadError::Deserialization { source: e })
    }
}

impl McBufWritable for RegistryHolder {
    fn write_into(&self, buf: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        TryInto::<Nbt>::try_into(self.clone())?.write_into(buf)
    }
}

/// A collection of values for a certain type of registry data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_registry", serde(deny_unknown_fields))]
pub struct RegistryType<T> {
    #[serde(rename = "type")]
    pub kind: ResourceLocation,
    pub value: Vec<TypeValue<T>>,
}

/// A value for a certain type of registry data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_registry", serde(deny_unknown_fields))]
pub struct TypeValue<T> {
    pub id: u32,
    pub name: ResourceLocation,
    pub element: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_registry", serde(deny_unknown_fields))]
pub struct TrimMaterialElement {
    pub asset_name: String,
    pub ingredient: ResourceLocation,
    pub item_model_index: f32,
    pub override_armor_materials: HashMap<String, String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Data about a kind of chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_registry", serde(deny_unknown_fields))]
pub struct ChatTypeElement {
    pub chat: ChatTypeData,
    pub narration: ChatTypeData,
}

/// Data about a chat message.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_registry", serde(deny_unknown_fields))]
pub struct ChatTypeData {
    pub translation_key: String,
    pub parameters: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<ChatTypeStyle>,
}

/// The style of a chat message.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_registry", serde(deny_unknown_fields))]
pub struct ChatTypeStyle {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "Convert")]
    pub bold: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "Convert")]
    pub italic: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "Convert")]
    pub underlined: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "Convert")]
    pub strikethrough: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "Convert")]
    pub obfuscated: Option<bool>,
}

/// Dimension attributes.
#[cfg(feature = "strict_registry")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DimensionTypeElement {
    pub ambient_light: f32,
    #[serde(with = "Convert")]
    pub bed_works: bool,
    pub coordinate_scale: f32,
    pub effects: ResourceLocation,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_time: Option<u32>,
    #[serde(with = "Convert")]
    pub has_ceiling: bool,
    #[serde(with = "Convert")]
    pub has_raids: bool,
    #[serde(with = "Convert")]
    pub has_skylight: bool,
    pub height: u32,
    pub infiniburn: ResourceLocation,
    pub logical_height: u32,
    pub min_y: i32,
    pub monster_spawn_block_light_limit: u32,
    pub monster_spawn_light_level: MonsterSpawnLightLevel,
    #[serde(with = "Convert")]
    pub natural: bool,
    #[serde(with = "Convert")]
    pub piglin_safe: bool,
    #[serde(with = "Convert")]
    pub respawn_anchor_works: bool,
    #[serde(with = "Convert")]
    pub ultrawarm: bool,
}

/// Dimension attributes.
#[cfg(not(feature = "strict_registry"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionTypeElement {
    pub height: u32,
    pub min_y: i32,
    #[serde(flatten)]
    pub _extra: HashMap<String, Nbt>,
}

/// The light level at which monsters can spawn.
///
/// This can be either a single minimum value, or a formula with a min and
/// max.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[cfg_attr(feature = "strict_registry", serde(deny_unknown_fields))]
pub enum MonsterSpawnLightLevel {
    /// A simple minimum value.
    Simple(u32),
    /// A complex value with a type, minimum, and maximum.
    /// Vanilla minecraft only uses one type, "minecraft:uniform".
    Complex {
        #[serde(rename = "type")]
        kind: ResourceLocation,
        value: MonsterSpawnLightLevelValues,
    },
}

/// The min and max light levels at which monsters can spawn.
///
/// Values are inclusive.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_registry", serde(deny_unknown_fields))]
pub struct MonsterSpawnLightLevelValues {
    #[serde(rename = "min_inclusive")]
    pub min: u32,
    #[serde(rename = "max_inclusive")]
    pub max: u32,
}

/// Biome attributes.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_registry", serde(deny_unknown_fields))]
pub struct WorldTypeElement {
    #[serde(with = "Convert")]
    pub has_precipitation: bool,
    pub temperature: f32,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature_modifier: Option<String>,
    pub downfall: f32,
    pub effects: BiomeEffects,
}

/// The precipitation of a biome.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_registry", serde(deny_unknown_fields))]
pub enum BiomePrecipitation {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "rain")]
    Rain,
    #[serde(rename = "snow")]
    Snow,
}

/// The effects of a biome.
///
/// This includes the sky, fog, water, and grass color,
/// as well as music and other sound effects.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_registry", serde(deny_unknown_fields))]
pub struct BiomeEffects {
    pub sky_color: u32,
    pub fog_color: u32,
    pub water_color: u32,
    pub water_fog_color: u32,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foliage_color: Option<u32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grass_color: Option<u32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grass_color_modifier: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub music: Option<BiomeMusic>,
    pub mood_sound: BiomeMoodSound,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additions_sound: Option<AdditionsSound>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ambient_sound: Option<ResourceLocation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub particle: Option<BiomeParticle>,
}

/// The music of the biome.
///
/// Some biomes have unique music that only play when inside them.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_registry", serde(deny_unknown_fields))]
pub struct BiomeMusic {
    #[serde(with = "Convert")]
    pub replace_current_music: bool,
    pub max_delay: u32,
    pub min_delay: u32,
    pub sound: azalea_registry::SoundEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_registry", serde(deny_unknown_fields))]
pub struct BiomeMoodSound {
    pub tick_delay: u32,
    pub block_search_extent: u32,
    pub offset: f32,
    pub sound: azalea_registry::SoundEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_registry", serde(deny_unknown_fields))]
pub struct AdditionsSound {
    pub tick_chance: f32,
    pub sound: azalea_registry::SoundEvent,
}

/// Biome particles.
///
/// Some biomes have particles that spawn in the air.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_registry", serde(deny_unknown_fields))]
pub struct BiomeParticle {
    pub probability: f32,
    pub options: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_registry", serde(deny_unknown_fields))]
pub struct TrimPatternElement {
    #[serde(flatten)]
    pub pattern: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_registry", serde(deny_unknown_fields))]
pub struct DamageTypeElement {
    pub message_id: String,
    pub scaling: String,
    pub exhaustion: f32,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effects: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub death_message_type: Option<String>,
}

// Using a trait because you can't implement methods for
// types you don't own, in this case Option<bool> and bool.
trait Convert: Sized {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;

    fn deserialize<'de, D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>;
}

// Convert between bool and u8
impl Convert for bool {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(if *self { 1 } else { 0 })
    }

    fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        convert::<D>(u8::deserialize(deserializer)?)
    }
}

// Convert between Option<bool> and u8
impl Convert for Option<bool> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(value) = self {
            Convert::serialize(value, serializer)
        } else {
            serializer.serialize_none()
        }
    }

    fn deserialize<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
    where
        D: Deserializer<'de>,
    {
        if let Some(value) = Option::<u8>::deserialize(deserializer)? {
            Ok(Some(convert::<D>(value)?))
        } else {
            Ok(None)
        }
    }
}

// Deserializing logic here to deduplicate code
fn convert<'de, D>(value: u8) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match value {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(de::Error::invalid_value(
            de::Unexpected::Unsigned(other as u64),
            &"zero or one",
        )),
    }
}
