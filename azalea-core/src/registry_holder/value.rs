use azalea_registry::{
    builtin::{
        Attribute, EnchantmentLevelBasedValueKind as LevelBasedValueKind,
        EnchantmentValueEffectKind as ValueEffectKind,
    },
    identifier::Identifier,
};
use simdnbt::{
    DeserializeError, FromNbtTag,
    borrow::{NbtCompound, NbtTag},
};

use crate::{
    attribute_modifier_operation::AttributeModifierOperation,
    registry_holder::{components::impl_from_effect_nbt_tag, get_in_compound},
};

#[derive(Clone, Debug)]
pub enum ValueEffect {
    Set {
        value: LevelBasedValue,
    },
    Add {
        value: LevelBasedValue,
    },
    Multiply {
        factor: LevelBasedValue,
    },
    RemoveBinomial {
        chance: LevelBasedValue,
    },
    AllOf {
        effects: Vec<ValueEffect>,
    },
    Exponential {
        base: LevelBasedValue,
        exponent: LevelBasedValue,
    },
}

impl simdnbt::Deserialize for ValueEffect {
    fn from_compound(nbt: NbtCompound) -> Result<Self, DeserializeError> {
        let kind = get_in_compound(&nbt, "type")?;
        let value = match kind {
            ValueEffectKind::Set => {
                let value = get_in_compound(&nbt, "value")?;
                Self::Set { value }
            }
            ValueEffectKind::Add => {
                let value = get_in_compound(&nbt, "value")?;
                Self::Add { value }
            }
            ValueEffectKind::Multiply => {
                let factor = get_in_compound(&nbt, "factor")?;
                Self::Multiply { factor }
            }
            ValueEffectKind::RemoveBinomial => {
                let chance = get_in_compound(&nbt, "chance")?;
                Self::RemoveBinomial { chance }
            }
            ValueEffectKind::AllOf => {
                let effects = get_in_compound(&nbt, "effects")?;
                Self::AllOf { effects }
            }
            ValueEffectKind::Exponential => {
                let base = get_in_compound(&nbt, "base")?;
                let exponent = get_in_compound(&nbt, "exponent")?;
                Self::Exponential { base, exponent }
            }
        };
        Ok(value)
    }
}
impl_from_effect_nbt_tag!(ValueEffect);

#[derive(Clone, Debug, simdnbt::Deserialize)]
pub struct AttributeEffect {
    pub id: Identifier,
    pub attribute: Attribute,
    pub amount: LevelBasedValue,
    pub operation: AttributeModifierOperation,
}
impl_from_effect_nbt_tag!(AttributeEffect);

#[derive(Clone, Debug)]
pub enum LevelBasedValue {
    Constant(f32),
    Exponent {
        base: Box<LevelBasedValue>,
        power: Box<LevelBasedValue>,
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
impl LevelBasedValue {
    pub fn calculate(&self, n: i32) -> f32 {
        match self {
            LevelBasedValue::Constant(v) => *v,
            LevelBasedValue::Exponent { base, power } => {
                (base.calculate(n) as f64).powf(power.calculate(n) as f64) as f32
            }
            LevelBasedValue::Linear {
                base,
                per_level_above_first,
            } => *base + *per_level_above_first * ((n - 1) as f32),
            LevelBasedValue::LevelsSquared { added } => (n * n) as f32 + *added,
            LevelBasedValue::Clamped { value, min, max } => value.calculate(n).clamp(*min, *max),
            LevelBasedValue::Fraction {
                numerator,
                denominator,
            } => {
                let value = denominator.calculate(n);
                if value == 0. {
                    0.
                } else {
                    numerator.calculate(n) / value
                }
            }
            LevelBasedValue::Lookup { values, fallback } => values
                .get((n - 1) as usize)
                .copied()
                .unwrap_or_else(|| fallback.calculate(n)),
        }
    }
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
            LevelBasedValueKind::Exponent => {
                let base = Box::new(get_in_compound(&nbt, "base")?);
                let power = Box::new(get_in_compound(&nbt, "power")?);
                return Ok(Self::Exponent { base, power });
            }
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
