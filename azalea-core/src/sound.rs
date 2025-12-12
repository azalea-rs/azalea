use azalea_buf::AzBuf;
use serde::Serialize;

use crate::identifier::Identifier;

#[derive(Clone, Debug, PartialEq, AzBuf, Serialize, simdnbt::Deserialize)]
pub struct CustomSound {
    pub sound_id: Identifier,
    pub range: Option<f32>,
}
