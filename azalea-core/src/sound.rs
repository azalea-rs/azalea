use azalea_buf::AzBuf;

use crate::resource_location::ResourceLocation;

#[derive(Clone, Debug, PartialEq, AzBuf)]
pub struct CustomSound {
    pub location: ResourceLocation,
    pub fixed_range: Option<f32>,
}
