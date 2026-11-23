use azalea_codec::Codec;
use azalea_core::{
    attribute_modifier_operation::AttributeModifierOperation, identifier::Identifier,
};
use azalea_registry::builtin::Attribute;

use crate::builtin::{EnchantmentLevelBasedValueKind, EnchantmentValueEffectKind};

#[derive(Clone, Debug, PartialEq, Codec)]
#[codec(tag = "type", tag_as = EnchantmentValueEffectKind)]
#[codec(default_snake_case_key)]
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

#[derive(Clone, Debug, PartialEq, Codec)]
pub struct AttributeEffect {
    pub id: Identifier,
    pub attribute: Attribute,
    pub amount: LevelBasedValue,
    pub operation: AttributeModifierOperation,
}

#[derive(Clone, Debug, PartialEq, Codec)]
pub enum LevelBasedValue {
    Constant(f32),
    Dispatch(DispatchLevelBasedValue),
}

#[derive(Clone, Debug, PartialEq, Codec)]
#[codec(tag = "type", tag_as = EnchantmentLevelBasedValueKind)]
#[codec(default_snake_case_key)]
pub enum DispatchLevelBasedValue {
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
            Self::Constant(v) => *v,
            Self::Dispatch(v) => v.calculate(n),
        }
    }
}
impl DispatchLevelBasedValue {
    pub fn calculate(&self, n: i32) -> f32 {
        match self {
            Self::Exponent { base, power } => {
                (base.calculate(n) as f64).powf(power.calculate(n) as f64) as f32
            }
            Self::Linear {
                base,
                per_level_above_first,
            } => *base + *per_level_above_first * ((n - 1) as f32),
            Self::LevelsSquared { added } => (n * n) as f32 + *added,
            Self::Clamped { value, min, max } => value.calculate(n).clamp(*min, *max),
            Self::Fraction {
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
            Self::Lookup { values, fallback } => values
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
