use azalea_buf::{BufReadError, McBufReadable, McBufWritable};
use azalea_core::ResourceLocation;
use azalea_nbt::Tag;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::io::Cursor;

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
pub struct RegistryHolder {
    #[serde(rename = "")]
    pub root: RegistryRoot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryRoot {
    #[serde(rename = "minecraft:chat_type")]
    pub chat_type: RegistryType<ChatTypeElement>,
    #[serde(rename = "minecraft:dimension_type")]
    pub dimension_type: RegistryType<DimensionTypeElement>,
    #[serde(rename = "minecraft:worldgen/biome")]
    pub worldgen: RegistryType<WorldTypeElement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryType<T> {
    #[serde(rename = "type")]
    pub type_: String,
    pub value: Vec<TypeValue<T>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeValue<T> {
    pub id: u32,
    pub name: ResourceLocation,
    pub element: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatTypeElement {
    pub chat: ChatTypeData,
    pub narration: ChatTypeData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatTypeData {
    pub translation_key: String,
    pub parameters: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<ChatTypeStyle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatTypeStyle {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "Convert::from_u8")]
    #[serde(serialize_with = "Convert::to_u8")]
    pub bold: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "Convert::from_u8")]
    #[serde(serialize_with = "Convert::to_u8")]
    pub italic: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "Convert::from_u8")]
    #[serde(serialize_with = "Convert::to_u8")]
    pub underlined: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "Convert::from_u8")]
    #[serde(serialize_with = "Convert::to_u8")]
    pub strikethrough: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "Convert::from_u8")]
    #[serde(serialize_with = "Convert::to_u8")]
    pub obfuscated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionTypeElement {
    pub ambient_light: f32,
    #[serde(deserialize_with = "Convert::from_u8")]
    #[serde(serialize_with = "Convert::to_u8")]
    pub bed_works: bool,
    pub coordinate_scale: f32,
    pub effects: ResourceLocation,
    #[serde(deserialize_with = "Convert::from_u8")]
    #[serde(serialize_with = "Convert::to_u8")]
    pub has_ceiling: bool,
    #[serde(deserialize_with = "Convert::from_u8")]
    #[serde(serialize_with = "Convert::to_u8")]
    pub has_raids: bool,
    #[serde(deserialize_with = "Convert::from_u8")]
    #[serde(serialize_with = "Convert::to_u8")]
    pub has_skylight: bool,
    pub height: u32,
    pub infiniburn: ResourceLocation,
    pub logical_height: u32,
    pub min_y: i32,
    pub monster_spawn_block_light_limit: u32,
    pub monster_spawn_light_level: MonsterSpawnLightLevel,
    #[serde(deserialize_with = "Convert::from_u8")]
    #[serde(serialize_with = "Convert::to_u8")]
    pub natural: bool,
    #[serde(deserialize_with = "Convert::from_u8")]
    #[serde(serialize_with = "Convert::to_u8")]
    pub piglin_safe: bool,
    #[serde(deserialize_with = "Convert::from_u8")]
    #[serde(serialize_with = "Convert::to_u8")]
    pub respawn_anchor_works: bool,
    #[serde(deserialize_with = "Convert::from_u8")]
    #[serde(serialize_with = "Convert::to_u8")]
    pub ultrawarm: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonsterSpawnLightLevel {
    Simple(u32),
    Complex {
        #[serde(rename = "type")]
        type_: String,
        value: MonsterSpawnLightLevelValues,
    },
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct MonsterSpawnLightLevelValues {
    #[serde(rename = "min_inclusive")]
    pub min: u32,
    #[serde(rename = "max_inclusive")]
    pub max: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldTypeElement {
    pub temperature: f32,
    pub downfall: f32,
    pub precipitation: BiomePrecipitation,
    pub effects: BiomeEffects,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub enum BiomePrecipitation {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "rain")]
    Rain,
    #[serde(rename = "snow")]
    Snow,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub music: Option<BiomeMusic>,
    pub mood_sound: BiomeMoodSound,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeMusic {
    #[serde(deserialize_with = "Convert::from_u8")]
    #[serde(serialize_with = "Convert::to_u8")]
    pub replace_current_music: bool,
    pub max_delay: u32,
    pub min_delay: u32,
    pub sound: MusicId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeMoodSound {
    pub tick_delay: u32,
    pub block_search_extent: u32,
    pub offset: f32,
    pub sound: MusicId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MusicId {
    pub sound_id: ResourceLocation,
}

// Using a trait because you can't implement methods for
// types you don't own, in this case Option<bool> and bool.
trait Convert: Sized {
    fn to_u8<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;

    fn from_u8<'de, D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>;
}

// Convert between bool and u8
impl Convert for bool {
    fn to_u8<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(if *self { 1 } else { 0 })
    }

    fn from_u8<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        convert::<D>(u8::deserialize(deserializer)?)
    }
}

// Convert between Option<bool> and u8
impl Convert for Option<bool> {
    fn to_u8<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(value) = self {
            value.to_u8(serializer)
        } else {
            serializer.serialize_none()
        }
    }

    fn from_u8<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
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
fn convert<'de, D>(val: u8) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match val {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(de::Error::invalid_value(
            de::Unexpected::Unsigned(other as u64),
            &"zero or one",
        )),
    }
}
