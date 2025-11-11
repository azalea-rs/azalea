use azalea_buf::AzBuf;
use serde::Serialize;

use crate::resource_location::Identifier;

#[derive(Clone, Debug, PartialEq, AzBuf, Serialize)]
pub struct CustomSound {
    pub sound_id: Identifier,
    pub range: Option<f32>,
}
