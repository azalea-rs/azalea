use std::str::FromStr;

use azalea_registry::EnchantmentEffectComponentKind;
use indexmap::IndexMap;
use simdnbt::{
    DeserializeError, FromNbtTag,
    borrow::{NbtCompound, NbtTag},
};

#[derive(Debug, Clone)]
pub struct EnchantmentData {
    // TODO: make these two deserializable
    // pub description: TextComponent,
    // pub exclusive_set: HolderSet<Enchantment, ResourceLocation>,
    pub effects: IndexMap<EnchantmentEffectComponentKind, EffectComponent>,
}

impl simdnbt::Deserialize for EnchantmentData {
    fn from_compound(nbt: NbtCompound) -> Result<Self, DeserializeError> {
        let mut effects: IndexMap<EnchantmentEffectComponentKind, EffectComponent> =
            IndexMap::new();
        for (key, value) in nbt
            .compound("effects")
            .ok_or(DeserializeError::MissingField)?
            .iter()
        {
            println!("key: {key}");
            println!("value: {:#?}", value.to_owned());
            let key = EnchantmentEffectComponentKind::from_str(&key.to_str())
                .map_err(|_| DeserializeError::UnknownField(key.to_string()))?;
            let value =
                EffectComponent::from_nbt_tag(value).ok_or(DeserializeError::MissingField)?;
            effects.insert(key, value);
        }

        let value = Self { effects };
        Ok(value)
    }
}

#[derive(Debug, Clone, simdnbt::Deserialize)]
pub struct EffectComponent {
    pub effect: ValueEffect,
}

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
        let kind = nbt.string("kind").ok_or(DeserializeError::MissingField)?;
        match kind.to_str().as_ref() {
            "minecraft:set" => {
                let value = get_in_compound(&nbt, "value")?;
                return Ok(Self::Set { value });
            }
            "minecraft:add" => {
                let value = get_in_compound(&nbt, "value")?;
                return Ok(Self::Add { value });
            }
            "minecraft:multiply" => {
                let factor = get_in_compound(&nbt, "factor")?;
                return Ok(Self::Multiply { factor });
            }
            "minecraft:remove_binomial" => {
                let chance = get_in_compound(&nbt, "chance")?;
                return Ok(Self::RemoveBinomial { chance });
            }
            "minecraft:all_of" => {
                let effects = get_in_compound(&nbt, "effects")?;
                return Ok(Self::AllOf { effects });
            }
            _ => return Err(DeserializeError::MismatchedFieldType("kind".to_owned())),
        }
    }
}

fn get_in_compound<T: FromNbtTag>(
    compound: &NbtCompound,
    key: &str,
) -> Result<T, DeserializeError> {
    T::from_nbt_tag(compound.get(key).ok_or(DeserializeError::MissingField)?)
        .ok_or(DeserializeError::MissingField)
}

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
        let kind = nbt.string("kind").ok_or(DeserializeError::MissingField)?;
        match kind.to_str().as_ref() {
            "minecraft:exponent" => {
                let base = get_in_compound(&nbt, "base")?;
                let power = get_in_compound(&nbt, "power")?;
                return Ok(Self::Exponent { base, power });
            }
            "minecraft:lienar" => {
                let base = get_in_compound(&nbt, "base")?;
                let per_level_above_first = get_in_compound(&nbt, "per_level_above_first")?;
                return Ok(Self::Linear {
                    base,
                    per_level_above_first,
                });
            }
            "minecraft:levels_squared" => {
                let added = get_in_compound(&nbt, "added")?;
                return Ok(Self::LevelsSquared { added });
            }
            "minecraft:clamped" => {
                let value = Box::new(get_in_compound(&nbt, "value")?);
                let min = get_in_compound(&nbt, "min")?;
                let max = get_in_compound(&nbt, "max")?;
                return Ok(Self::Clamped { value, min, max });
            }
            "minecraft:fraction" => {
                let numerator = Box::new(get_in_compound(&nbt, "numerator")?);
                let denominator = Box::new(get_in_compound(&nbt, "denominator")?);
                return Ok(Self::Fraction {
                    numerator,
                    denominator,
                });
            }
            "minecraft:lookup" => {
                let values = nbt
                    .list("values")
                    .ok_or(DeserializeError::MissingField)?
                    .floats()
                    .ok_or(DeserializeError::MissingField)?;
                let fallback = Box::new(get_in_compound(&nbt, "fallback")?);
                return Ok(Self::Lookup { values, fallback });
            }
            _ => return Err(DeserializeError::MismatchedFieldType("kind".to_owned())),
        }
    }
}
