use std::ops::Range;

use azalea_registry::builtin::FloatProviderKind;
use simdnbt::{
    DeserializeError, FromNbtTag,
    borrow::{NbtCompound, NbtTag},
};

use crate::registry_holder::get_in_compound;

#[derive(Clone, Debug)]
pub enum FloatProvider {
    Constant(f32),
    Uniform {
        range: Range<f32>,
    },
    ClampedNormal {
        mean: f32,
        deviation: f32,
        min: f32,
        max: f32,
    },
    Trapezoid {
        min: f32,
        max: f32,
        plateau: f32,
    },
}
impl FromNbtTag for FloatProvider {
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
impl FloatProvider {
    fn from_compound(nbt: NbtCompound) -> Result<Self, DeserializeError> {
        let kind = get_in_compound(&nbt, "type")?;
        match kind {
            FloatProviderKind::Constant => Ok(Self::Constant(get_in_compound(&nbt, "value")?)),
            FloatProviderKind::Uniform => {
                let min_inclusive = get_in_compound(&nbt, "min_inclusive")?;
                let max_exclusive = get_in_compound(&nbt, "max_exclusive")?;
                Ok(Self::Uniform {
                    range: min_inclusive..max_exclusive,
                })
            }
            FloatProviderKind::ClampedNormal => {
                let mean = get_in_compound(&nbt, "mean")?;
                let deviation = get_in_compound(&nbt, "deviation")?;
                let min = get_in_compound(&nbt, "min")?;
                let max = get_in_compound(&nbt, "max")?;
                Ok(Self::ClampedNormal {
                    mean,
                    deviation,
                    min,
                    max,
                })
            }
            FloatProviderKind::Trapezoid => {
                let min = get_in_compound(&nbt, "min")?;
                let max = get_in_compound(&nbt, "max")?;
                let plateau = get_in_compound(&nbt, "plateau")?;
                Ok(Self::Trapezoid { min, max, plateau })
            }
        }
    }
}
