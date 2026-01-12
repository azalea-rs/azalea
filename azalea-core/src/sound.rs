use azalea_buf::AzBuf;
use azalea_registry::identifier::Identifier;

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(AzBuf, Clone, Debug, simdnbt::Deserialize, PartialEq)]
pub struct CustomSound {
    pub sound_id: Identifier,
    pub range: Option<f32>,
}
