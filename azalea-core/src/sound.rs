use azalea_buf::AzBuf;
use serde::Serialize;

use crate::resource_location::ResourceLocation;

#[derive(Clone, Debug, PartialEq, AzBuf, Serialize)]
pub struct CustomSound {
    pub location: ResourceLocation,
    pub fixed_range: Option<f32>,
}
