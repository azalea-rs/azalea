use azalea_auth::game_profile::{GameProfile, GameProfileProperties};
use azalea_buf::AzBuf;
use azalea_codec::Codec;
use azalea_core::identifier::Identifier;
use uuid::Uuid;

#[derive(AzBuf, Clone, Debug, Default, PartialEq, Codec)]
#[doc(alias = "ResolvableProfile")]
pub struct Profile {
    #[codec(flatten)]
    pub unpack: Box<PartialOrFullProfile>,
    #[codec(flatten)]
    pub skin_patch: Box<PlayerSkinPatch>,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
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

#[derive(AzBuf, Clone, Debug, Default, PartialEq, Codec)]
pub struct PartialProfile {
    #[limit(16)]
    #[codec(optional)]
    pub name: Option<String>,
    #[codec(optional)]
    pub id: Option<Uuid>,
    #[codec(optional)]
    pub properties: GameProfileProperties,
}

#[derive(AzBuf, Clone, Debug, Default, PartialEq, Codec)]
pub struct PlayerSkinPatch {
    #[codec(rename = "texture")]
    #[codec(optional)]
    pub body: Option<ResourceTexture>,
    #[codec(optional)]
    pub cape: Option<ResourceTexture>,
    #[codec(optional)]
    pub elytra: Option<ResourceTexture>,
    #[codec(optional)]
    pub model: Option<PlayerModelType>,
}

#[derive(AzBuf, Clone, Copy, Debug, Default, PartialEq, Codec)]
#[codec(default_snake_case_key)]
pub enum PlayerModelType {
    #[default]
    Wide,
    Slim,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(transparent)]
pub struct ResourceTexture {
    pub id: Identifier,
}
