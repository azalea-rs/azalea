use azalea_buf::AzBuf;
use serde::Serialize;

use crate::resource_location::ResourceLocation;

#[derive(Clone, Debug, PartialEq, AzBuf, Serialize)]
pub struct CustomSound {
    pub sound_id: ResourceLocation,
    pub range: Option<f32>,
}
