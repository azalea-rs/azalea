use azalea_buf::AzBuf;
use azalea_registry::identifier::Identifier;
use serde::Serialize;

#[derive(AzBuf, Clone, Debug, simdnbt::Deserialize, PartialEq, Serialize)]
pub struct CustomSound {
    pub sound_id: Identifier,
    pub range: Option<f32>,
}
