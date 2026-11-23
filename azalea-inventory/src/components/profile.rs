use azalea_auth::game_profile::{
    GameProfile, GameProfileProperties, SerializableProfileProperties,
};
use azalea_buf::AzBuf;
use azalea_codec::Codec;
use azalea_core::{codec_utils::*, identifier::Identifier};
use serde::{Serialize, Serializer};
use uuid::Uuid;

#[derive(AzBuf, Clone, Debug, Default, PartialEq, Serialize, Codec)]
#[doc(alias = "ResolvableProfile")]
pub struct Profile {
    #[serde(flatten)]
    #[codec(flatten)]
    pub unpack: Box<PartialOrFullProfile>,
    #[serde(flatten)]
    #[codec(flatten)]
    pub skin_patch: Box<PlayerSkinPatch>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(untagged)]
#[codec(is_map)]
pub enum PartialOrFullProfile {
    Partial(PartialProfile),
    Full(GameProfile),
}
impl Default for PartialOrFullProfile {
    fn default() -> Self {
        Self::Partial(PartialProfile::default())
    }
}

#[derive(AzBuf, Clone, Debug, Default, PartialEq, Serialize, Codec)]
pub struct PartialProfile {
    #[limit(16)]
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub id: Option<Uuid>,
    #[serde(serialize_with = "serialize_properties")]
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub properties: GameProfileProperties,
}
fn serialize_properties<S: Serializer>(
    properties: &GameProfileProperties,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let serializable = SerializableProfileProperties::from(properties.clone());
    serializable.serialize(serializer)
}

#[derive(AzBuf, Clone, Debug, Default, PartialEq, Serialize, Codec)]
pub struct PlayerSkinPatch {
    #[serde(rename = "texture")]
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub body: Option<ResourceTexture>,
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub cape: Option<ResourceTexture>,
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub elytra: Option<ResourceTexture>,
    #[serde(skip_serializing_if = "is_default")]
    #[codec(optional)]
    pub model: Option<PlayerModelType>,
}

#[derive(AzBuf, Clone, Copy, Debug, Default, PartialEq, Serialize, Codec)]
#[serde(rename_all = "snake_case")]
#[codec(default_snake_case_key)]
pub enum PlayerModelType {
    #[default]
    Wide,
    Slim,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Serialize, Codec)]
#[serde(transparent)]
#[codec(transparent)]
pub struct ResourceTexture {
    pub id: Identifier,
}
