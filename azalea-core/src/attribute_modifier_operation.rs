use std::str::FromStr;

use azalea_buf::AzBuf;
use serde::Serialize;
use simdnbt::{FromNbtTag, borrow::NbtTag};

#[derive(AzBuf, Clone, Copy, Debug, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
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
impl FromNbtTag for AttributeModifierOperation {
    fn from_nbt_tag(tag: NbtTag) -> Option<Self> {
        let v = tag.string()?;
        Self::from_str(&v.to_str()).ok()
    }
}
