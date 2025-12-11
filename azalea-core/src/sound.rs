use azalea_buf::AzBuf;
use azalea_registry::identifier::Identifier;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, AzBuf, Serialize, simdnbt::Deserialize)]
pub struct CustomSound {
    pub sound_id: Identifier,
    pub range: Option<f32>,
}
