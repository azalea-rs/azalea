use azalea_buf::AzBuf;
use serde::Serialize;

use crate::identifier::Identifier;

#[derive(Clone, Debug, PartialEq, AzBuf, Serialize, simdnbt::Deserialize)]
pub struct CustomSound {
    pub location: Identifier,
    pub fixed_range: Option<f32>,
}
