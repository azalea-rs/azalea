use std::{fmt::Debug, str::FromStr};

use azalea_registry::EnchantmentEffectComponentKind;
use indexmap::IndexMap;
use simdnbt::{
    DeserializeError, FromNbtTag,
    borrow::{NbtCompound, NbtTag},
};

use crate::registry_holder::components::EffectComponentUnion;

pub struct EnchantmentData {
    // TODO: make these two deserializable
    // pub description: TextComponent,
    // pub exclusive_set: HolderSet<Enchantment, ResourceLocation>,
    pub effects: IndexMap<EnchantmentEffectComponentKind, EffectComponentUnion>,
}

impl simdnbt::Deserialize for EnchantmentData {
    fn from_compound(nbt: NbtCompound) -> Result<Self, DeserializeError> {
        let mut effects: IndexMap<EnchantmentEffectComponentKind, EffectComponentUnion> =
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
