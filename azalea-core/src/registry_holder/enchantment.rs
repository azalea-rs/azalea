use std::{any::Any, fmt::Debug, str::FromStr};

use azalea_registry::builtin::EnchantmentEffectComponentKind;
use indexmap::IndexMap;
use simdnbt::{DeserializeError, borrow::NbtCompound};

use crate::registry_holder::components::{
    EffectComponentTrait, EffectComponentUnion, EffectNbtTag, ResolvedEffectComponent,
};

pub struct EnchantmentData {
    // TODO: make these two deserializable
    // pub description: TextComponent,
    // pub exclusive_set: HolderSet<Enchantment, ResourceLocation>,
    effects: IndexMap<EnchantmentEffectComponentKind, Vec<EffectComponentUnion>>,
}

impl EnchantmentData {
    pub fn get<T: EffectComponentTrait>(&self) -> Option<Vec<&T>> {
        let components = self.get_kind(T::KIND)?;
        let components_any = components
            .into_iter()
            .map(|c| (c as &dyn Any).downcast_ref::<T>())
            .collect::<Option<_>>()?;
        Some(components_any)
    }

    pub fn get_kind(
        &self,
        kind: EnchantmentEffectComponentKind,
    ) -> Option<Vec<&dyn ResolvedEffectComponent>> {
        self.effects.get(&kind).map(|c| {
            c.iter()
                .map(|c| {
                    // SAFETY: we just got the component from the map, so it must be the correct
                    // kind
                    unsafe { c.as_kind(kind) }
                })
                .collect()
        })
    }
}

impl simdnbt::Deserialize for EnchantmentData {
    fn from_compound(nbt: NbtCompound) -> Result<Self, DeserializeError> {
        let mut effects: IndexMap<EnchantmentEffectComponentKind, Vec<EffectComponentUnion>> =
            IndexMap::new();

        if let Some(effects_tag) = nbt.compound("effects") {
            for (key, list) in effects_tag.iter() {
                let kind = EnchantmentEffectComponentKind::from_str(&key.to_str())
                    .map_err(|_| DeserializeError::UnknownField(key.to_string()))?;

                let mut components = Vec::new();
                if let Some(tag) = list.compound() {
                    if !tag.is_empty() {
                        let value = EffectComponentUnion::from_effect_nbt_tag_as(
                            kind,
                            EffectNbtTag::Compound(tag),
                        )?;
                        components.push(value);
                    }
                } else {
                    let list = list.list().ok_or_else(|| {
                        DeserializeError::MismatchedFieldType("effects".to_owned())
                    })?;

                    if let Some(tags) = list.compounds() {
                        for tag in tags {
                            let value = EffectComponentUnion::from_effect_nbt_tag_as(
                                kind,
                                EffectNbtTag::Compound(tag),
                            )?;
                            components.push(value);
                        }
                    } else {
                        let value = EffectComponentUnion::from_effect_nbt_tag_as(
                            kind,
                            EffectNbtTag::List(list),
                        )?;
                        components.push(value);
                    }
                }

                effects.insert(kind, components);
            }
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
