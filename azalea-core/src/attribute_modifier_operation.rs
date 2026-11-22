use std::str::FromStr;

use azalea_buf::AzBuf;
use azalea_codec::Codec;

#[derive(AzBuf, Clone, Copy, Debug, PartialEq, Codec)]
#[codec(default_snake_case_key)]
pub enum AttributeModifierOperation {
    AddValue,
    AddMultipliedBase,
    AddMultipliedTotal,
}

impl FromStr for AttributeModifierOperation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value: AttributeModifierOperation = match s {
            "add_value" => Self::AddValue,
            "add_multiplied_base" => Self::AddMultipliedBase,
            "add_multiplied_total" => Self::AddMultipliedTotal,
            _ => return Err(()),
        };
        Ok(value)
    }
}
