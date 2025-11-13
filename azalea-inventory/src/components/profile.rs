use azalea_auth::game_profile::{
    GameProfile, GameProfileProperties, SerializableProfileProperties,
};
use azalea_buf::AzBuf;
use azalea_core::{codec_utils::*, identifier::Identifier};
use serde::{Serialize, Serializer};
use uuid::Uuid;

#[derive(Clone, Debug, AzBuf, Default, PartialEq, Serialize)]
#[doc(alias = "ResolvableProfile")]
pub struct Profile {
    #[serde(flatten)]
    pub unpack: Box<PartialOrFullProfile>,
    #[serde(flatten)]
    pub skin_patch: Box<PlayerSkinPatch>,
}

#[derive(Clone, Debug, AzBuf, PartialEq, Serialize)]
#[serde(untagged)]
pub enum PartialOrFullProfile {
    Partial(PartialProfile),
    Full(GameProfile),
}
impl Default for PartialOrFullProfile {
    fn default() -> Self {
        Self::Partial(PartialProfile::default())
    }
}

#[derive(Clone, Debug, AzBuf, Default, PartialEq, Serialize)]
pub struct PartialProfile {
    #[limit(16)]
    #[serde(skip_serializing_if = "is_default")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "is_default")]
    pub id: Option<Uuid>,
    #[serde(serialize_with = "serialize_properties")]
    #[serde(skip_serializing_if = "is_default")]
    pub properties: GameProfileProperties,
}
fn serialize_properties<S: Serializer>(
    properties: &GameProfileProperties,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let serializable = SerializableProfileProperties::from(properties.clone());
    serializable.serialize(serializer)
}

#[derive(Clone, Debug, AzBuf, Default, PartialEq, Serialize)]
pub struct PlayerSkinPatch {
    #[serde(rename = "texture")]
    #[serde(skip_serializing_if = "is_default")]
    pub body: Option<ResourceTexture>,
    #[serde(skip_serializing_if = "is_default")]
    pub cape: Option<ResourceTexture>,
    #[serde(skip_serializing_if = "is_default")]
    pub elytra: Option<ResourceTexture>,
    #[serde(skip_serializing_if = "is_default")]
    pub model: Option<PlayerModelType>,
}

#[derive(Clone, Debug, Copy, AzBuf, Default, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PlayerModelType {
    #[default]
    Wide,
    Slim,
}

#[derive(Clone, Debug, AzBuf, PartialEq, Serialize)]
#[serde(transparent)]
pub struct ResourceTexture {
    pub id: Identifier,
}
