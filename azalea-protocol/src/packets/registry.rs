//! ClientboundLoginPacket Registry Structure

use azalea_buf::{BufReadError, McBufReadable, McBufWritable};
use azalea_core::ResourceLocation;
use azalea_nbt::Tag;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::{collections::HashMap, io::Cursor};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_registry", serde(deny_unknown_fields))]
pub struct RegistryHolder {
    #[serde(rename = "")]
    pub root: RegistryRoot,
}

impl TryFrom<Tag> for RegistryHolder {
    type Error = serde_json::Error;

    fn try_from(value: Tag) -> Result<Self, Self::Error> {
        serde_json::from_value(serde_json::to_value(value)?)
    }
}

impl TryInto<Tag> for RegistryHolder {
    type Error = serde_json::Error;

    fn try_into(self) -> Result<Tag, Self::Error> {
        serde_json::from_value(serde_json::to_value(self)?)
    }
}

impl McBufReadable for RegistryHolder {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        RegistryHolder::try_from(Tag::read_from(buf)?)
            .map_err(|e| BufReadError::Deserialization { source: e })
    }
}

impl McBufWritable for RegistryHolder {
    fn write_into(&self, buf: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        TryInto::<Tag>::try_into(self.clone())?.write_into(buf)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_registry", serde(deny_unknown_fields))]
pub struct RegistryRoot {
    #[serde(rename = "minecraft:chat_type")]
    pub chat_type: RegistryType<ChatTypeElement>,
    #[serde(rename = "minecraft:dimension_type")]
    pub dimension_type: RegistryType<DimensionTypeElement>,
    #[serde(rename = "minecraft:worldgen/biome")]
    pub world_type: RegistryType<WorldTypeElement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_registry", serde(deny_unknown_fields))]
pub struct RegistryType<T> {
    #[serde(rename = "type")]
    pub type_: ResourceLocation,
    pub value: Vec<TypeValue<T>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_registry", serde(deny_unknown_fields))]
pub struct TypeValue<T> {
    pub id: u32,
    pub name: ResourceLocation,
    pub element: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_registry", serde(deny_unknown_fields))]
pub struct ChatTypeElement {
    pub chat: ChatTypeData,
    pub narration: ChatTypeData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_registry", serde(deny_unknown_fields))]
pub struct ChatTypeData {
    pub translation_key: String,
    pub parameters: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<ChatTypeStyle>,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_registry", serde(deny_unknown_fields))]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[cfg_attr(feature = "strict_registry", serde(deny_unknown_fields))]
pub enum MonsterSpawnLightLevel {
    Simple(u32),
    Complex {
        #[serde(rename = "type")]
        type_: String,
        value: MonsterSpawnLightLevelValues,
    },
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_registry", serde(deny_unknown_fields))]
pub struct MonsterSpawnLightLevelValues {
    #[serde(rename = "min_inclusive")]
    pub min: u32,
    #[serde(rename = "max_inclusive")]
    pub max: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_registry", serde(deny_unknown_fields))]
pub struct WorldTypeElement {
    pub temperature: f32,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature_modifier: Option<String>,
    pub downfall: f32,
    pub precipitation: BiomePrecipitation,
    pub effects: BiomeEffects,
}

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
    pub ambient_sound: Option<MusicId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub particle: Option<BiomeParticle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_registry", serde(deny_unknown_fields))]
pub struct BiomeMusic {
    #[serde(with = "Convert")]
    pub replace_current_music: bool,
    pub max_delay: u32,
    pub min_delay: u32,
    pub sound: MusicId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_registry", serde(deny_unknown_fields))]
pub struct BiomeMoodSound {
    pub tick_delay: u32,
    pub block_search_extent: u32,
    pub offset: f32,
    pub sound: MusicId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_registry", serde(deny_unknown_fields))]
pub struct AdditionsSound {
    pub tick_chance: f32,
    pub sound: MusicId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_registry", serde(deny_unknown_fields))]
pub struct MusicId {
    pub sound_id: ResourceLocation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_registry", serde(deny_unknown_fields))]
pub struct BiomeParticle {
    pub probability: f32,
    pub options: HashMap<String, String>,
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

// Deserializing logic here for deduplicating the code
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

#[cfg(test)]
mod tests {
    use azalea_core::ResourceLocation;
    use azalea_nbt::Tag;

    use super::{
        ChatTypeElement, DimensionTypeElement, RegistryHolder, RegistryRoot, RegistryType,
        WorldTypeElement,
    };

    #[test]
    fn test_convert() {
        let registry = RegistryHolder {
            root: RegistryRoot {
                chat_type: RegistryType::<ChatTypeElement> {
                    type_: ResourceLocation::new("minecraft:chat_type").unwrap(),
                    value: Vec::new(),
                },
                dimension_type: RegistryType::<DimensionTypeElement> {
                    type_: ResourceLocation::new("minecraft:dimension_type").unwrap(),
                    value: Vec::new(),
                },
                world_type: RegistryType::<WorldTypeElement> {
                    type_: ResourceLocation::new("minecraft:worldgen/biome").unwrap(),
                    value: Vec::new(),
                },
            },
        };

        let tag: Tag = registry.try_into().unwrap();
        let root = tag
            .as_compound()
            .unwrap()
            .get("")
            .unwrap()
            .as_compound()
            .unwrap();

        let chat = root
            .get("minecraft:chat_type")
            .unwrap()
            .as_compound()
            .unwrap();
        let chat_type = chat.get("type").unwrap().as_string().unwrap();
        assert!(chat_type == "minecraft:chat_type");

        let dimension = root
            .get("minecraft:dimension_type")
            .unwrap()
            .as_compound()
            .unwrap();
        let dimension_type = dimension.get("type").unwrap().as_string().unwrap();
        assert!(dimension_type == "minecraft:dimension_type");

        let world = root
            .get("minecraft:worldgen/biome")
            .unwrap()
            .as_compound()
            .unwrap();
        let world_type = world.get("type").unwrap().as_string().unwrap();
        assert!(world_type == "minecraft:worldgen/biome");
    }
}
