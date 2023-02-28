use crate::Tag;
use azalea_buf::{BufReadError, McBufReadable, McBufWritable};
use serde::{Deserialize, Serialize};
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
    pub chat_type: ChatType,
    #[serde(rename = "minecraft:dimension_type")]
    pub dimension_type: DimensionType,
    #[serde(rename = "minecraft:worldgen/biome")]
    pub worldgen: WorldgenBiomes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatType {
    #[serde(rename = "type")]
    pub type_: String,
    pub value: Vec<ChatTypeValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatTypeValue {
    pub id: u32,
    pub name: String,
    pub element: ChatTypeList,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatTypeList {
    pub chat: ChatTypeElement,
    pub narration: ChatTypeElement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatTypeElement {
    pub translation_key: String,
    pub parameters: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<ChatTypeStyle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatTypeStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bold: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub italic: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlined: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strikethrough: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obfuscated: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionType {
    #[serde(rename = "type")]
    pub type_: String,
    pub value: Vec<DimensionTypeValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionTypeValue {
    pub id: u32,
    pub name: String,
    pub element: DimensionTypeElement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionTypeElement {
    pub piglin_safe: u8,
    pub natural: u8,
    pub ambient_light: f32,
    pub infiniburn: String,
    pub respawn_anchor_works: u8,
    pub has_skylight: u8,
    pub bed_works: u8,
    pub effects: String,
    pub has_raids: u8,
    pub height: u32,
    pub logical_height: u32,
    pub coordinate_scale: f32,
    pub ultrawarm: u8,
    pub has_ceiling: u8,
    pub min_y: i32,
    pub monster_spawn_block_light_limit: u32,
    pub monster_spawn_light_level: MonsterSpawnLightLevel,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterSpawnLightLevelValues {
    #[serde(rename = "min_inclusive")]
    pub min: u32,
    #[serde(rename = "max_inclusive")]
    pub max: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldgenBiomes {
    #[serde(rename = "type")]
    pub type_: String,
    pub value: Vec<WorldgenBiomesValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldgenBiomesValue {
    pub id: u32,
    pub name: String,
    pub element: WorldgenBiomesElement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldgenBiomesElement {
    pub temperature: f32,
    pub downfall: f32,
    pub precipitation: BiomePrecipitation,
    pub effects: BiomeEffects,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foliage_color: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grass_color: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub music: Option<BiomeMusic>,
    pub mood_sound: BiomeMoodSound,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeMusic {
    pub replace_current_music: u8,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicId {
    pub sound_id: String,
}
