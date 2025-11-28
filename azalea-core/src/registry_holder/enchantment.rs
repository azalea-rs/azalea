use std::{fmt::Debug, str::FromStr};

use azalea_registry::EnchantmentEffectComponentKind;
use indexmap::IndexMap;
use simdnbt::{DeserializeError, borrow::NbtCompound};

use crate::registry_holder::components::{EffectComponentUnion, EffectNbtTag};

pub struct EnchantmentData {
    // TODO: make these two deserializable
    // pub description: TextComponent,
    // pub exclusive_set: HolderSet<Enchantment, ResourceLocation>,
    pub effects: IndexMap<EnchantmentEffectComponentKind, Vec<EffectComponentUnion>>,
}

impl simdnbt::Deserialize for EnchantmentData {
    fn from_compound(nbt: NbtCompound) -> Result<Self, DeserializeError> {
        let mut effects: IndexMap<EnchantmentEffectComponentKind, Vec<EffectComponentUnion>> =
            IndexMap::new();
        for (key, list) in nbt
            .compound("effects")
            .ok_or(DeserializeError::MissingField)?
            .iter()
        {
            println!("key: {key}");
            let kind = EnchantmentEffectComponentKind::from_str(&key.to_str())
                .map_err(|_| DeserializeError::UnknownField(key.to_string()))?;
            println!("parsed kind: {kind}");
            println!("list: {list:?}");

            let mut components = Vec::new();
            if let Some(empty_list) = list.compound() {
                if !empty_list.is_empty() {
                    return Err(DeserializeError::MismatchedFieldType("effects".to_owned()));
                }
            } else {
                let list = list
                    .list()
                    .ok_or_else(|| DeserializeError::MismatchedFieldType("effects".to_owned()))?;

                println!("list type: {}", list.id());

                for tag in list
                    .compounds()
                    .ok_or_else(|| DeserializeError::MismatchedFieldType("effects".to_owned()))?
                {
                    println!("value: {:#?}", tag.to_owned());
                    let value = EffectComponentUnion::from_effect_nbt_tag_as(
                        kind,
                        EffectNbtTag::Compound(tag),
                    )?;
                    components.push(value);
                }
            }

            effects.insert(kind, components);
        }

        let value = Self { effects };
        Ok(value)
    }
}

impl Debug for EnchantmentData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EnchantmentData")
            .field("effects", &self.effects.keys())
            .finish()
    }
}

impl Clone for EnchantmentData {
    fn clone(&self) -> Self {
        let mut effects = IndexMap::with_capacity(self.effects.len());
        for (kind, effect) in &self.effects {
            effects.insert(
                *kind,
                effect
                    .iter()
                    .map(|e| unsafe { e.clone_as(*kind) })
                    .collect(),
            );
        }
        EnchantmentData { effects }
    }
}
