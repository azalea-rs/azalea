use azalea_buf::AzBuf;
use azalea_codec::Codec;

use crate::identifier::Identifier;

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub struct CustomSound {
    pub sound_id: Identifier,
    pub range: Option<f32>,
}
