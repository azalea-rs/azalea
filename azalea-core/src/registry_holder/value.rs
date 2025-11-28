use azalea_registry::{
    Attribute, EnchantmentLevelBasedValueKind as LevelBasedValueKind,
    EnchantmentValueEffectKind as ValueEffectKind,
};
use simdnbt::{
    DeserializeError, FromNbtTag,
    borrow::{NbtCompound, NbtTag},
};

use crate::{
    attribute_modifier_operation::AttributeModifierOperation,
    identifier::Identifier,
    registry_holder::{components::impl_from_effect_nbt_tag, get_in_compound},
};

#[derive(Debug, Clone)]
pub enum ValueEffect {
    Set { value: LevelBasedValue },
    Add { value: LevelBasedValue },
    Multiply { factor: LevelBasedValue },
    RemoveBinomial { chance: LevelBasedValue },
    AllOf { effects: Vec<ValueEffect> },
}

impl simdnbt::Deserialize for ValueEffect {
    fn from_compound(nbt: NbtCompound) -> Result<Self, DeserializeError> {
        let kind = get_in_compound(&nbt, "type")?;
        match kind {
            ValueEffectKind::Set => {
                let value = get_in_compound(&nbt, "value")?;
                return Ok(Self::Set { value });
            }
            ValueEffectKind::Add => {
                let value = get_in_compound(&nbt, "value")?;
                return Ok(Self::Add { value });
            }
            ValueEffectKind::Multiply => {
                let factor = get_in_compound(&nbt, "factor")?;
                return Ok(Self::Multiply { factor });
            }
            ValueEffectKind::RemoveBinomial => {
                let chance = get_in_compound(&nbt, "chance")?;
                return Ok(Self::RemoveBinomial { chance });
            }
            ValueEffectKind::AllOf => {
                let effects = get_in_compound(&nbt, "effects")?;
                return Ok(Self::AllOf { effects });
            }
        }
    }
}
impl_from_effect_nbt_tag!(ValueEffect);

#[derive(Debug, Clone, simdnbt::Deserialize)]
pub struct AttributeEffect {
    pub id: Identifier,
    pub attribute: Attribute,
    pub amount: LevelBasedValue,
    pub operation: AttributeModifierOperation,
}
impl_from_effect_nbt_tag!(AttributeEffect);

#[derive(Debug, Clone)]
pub enum LevelBasedValue {
    Constant(f32),
    Exponent {
        base: f32,
        power: f32,
    },
    Linear {
        base: f32,
        per_level_above_first: f32,
    },
    LevelsSquared {
        added: f32,
    },
    Clamped {
        value: Box<LevelBasedValue>,
        min: f32,
        max: f32,
    },
    Fraction {
        numerator: Box<LevelBasedValue>,
        denominator: Box<LevelBasedValue>,
    },
    Lookup {
        values: Vec<f32>,
        fallback: Box<LevelBasedValue>,
    },
}

impl Default for LevelBasedValue {
    fn default() -> Self {
        Self::Constant(0.)
    }
}

impl FromNbtTag for LevelBasedValue {
    fn from_nbt_tag(tag: NbtTag) -> Option<Self> {
        if let Some(f) = tag.float() {
            return Some(Self::Constant(f));
        }

        if let Some(c) = tag.compound() {
            return Self::from_compound(c).ok();
        }

        None
    }
}
impl LevelBasedValue {
    fn from_compound(nbt: NbtCompound) -> Result<Self, DeserializeError> {
        let kind = get_in_compound(&nbt, "type")?;
        let value = match kind {
            // LevelBasedValueKind::Exponent => {
            //     let base = get_in_compound(&nbt, "base")?;
            //     let power = get_in_compound(&nbt, "power")?;
            //     return Ok(Self::Exponent { base, power });
            // }
            LevelBasedValueKind::Linear => {
                let base = get_in_compound(&nbt, "base")?;
                let per_level_above_first = get_in_compound(&nbt, "per_level_above_first")?;
                Self::Linear {
                    base,
                    per_level_above_first,
                }
            }
            LevelBasedValueKind::LevelsSquared => {
                let added = get_in_compound(&nbt, "added")?;
                Self::LevelsSquared { added }
            }
            LevelBasedValueKind::Clamped => {
                let value = Box::new(get_in_compound(&nbt, "value")?);
                let min = get_in_compound(&nbt, "min")?;
                let max = get_in_compound(&nbt, "max")?;
                Self::Clamped { value, min, max }
            }
            LevelBasedValueKind::Fraction => {
                let numerator = Box::new(get_in_compound(&nbt, "numerator")?);
                let denominator = Box::new(get_in_compound(&nbt, "denominator")?);
                Self::Fraction {
                    numerator,
                    denominator,
                }
            }
            LevelBasedValueKind::Lookup => {
                let values = nbt
                    .list("values")
                    .ok_or(DeserializeError::MissingField)?
                    .floats()
                    .ok_or(DeserializeError::MissingField)?;
                let fallback = Box::new(get_in_compound(&nbt, "fallback")?);
                Self::Lookup { values, fallback }
            }
        };
        Ok(value)
    }
}
