use std::{any::Any, fmt::Debug, str::FromStr};

use azalea_codec::{Codec, MapLike};
use azalea_registry::builtin::EnchantmentEffectComponentKind;
use indexmap::IndexMap;
use simdnbt::{DeserializeError, borrow::NbtCompound};

use crate::registry_holder::components::{
    EffectComponentTrait, EffectComponentUnion, EffectNbtTag, ResolvedEffectComponent,
};

#[derive(Codec)]
pub struct EnchantmentData {
    // TODO: make these two deserializable
    // pub description: TextComponent,
    // pub exclusive_set: HolderSet<Enchantment, ResourceLocation>,
    #[codec(with = Effects)]
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
                    // SAFETY: we just got the component from the map, so it
                    // must be the correct kind
                    unsafe { c.as_kind(kind) }
                })
                .collect()
        })
    }
}

struct Effects(IndexMap<EnchantmentEffectComponentKind, Vec<EffectComponentUnion>>);
impl<O: azalea_codec::Ops> Codec<O> for Effects {
    fn codec_read(ops: &O) -> azalea_codec::error::ReadResult<Self> {
        let map = ops.get_map()?;
        let mut items = Vec::new();
        map.for_each(|k, v| {
            items.push(
                EnchantmentEffectComponentKind::codec_read(k)
                    .and_then(|k| EffectComponentUnion::codec_read_as(k, v).map(|v| (k, v))),
            );
        });
        Ok(Self(
            items
                .into_iter()
                .collect::<azalea_codec::error::ReadResult<_>>()?,
        ))
    }
    fn codec_write(&self, ctx: O::Context) -> azalea_codec::error::WriteResult<O> {
        let mut map = Vec::new();
        for (&k, v) in &self.0 {
            let v = v
                .into_iter()
                .map(|v| unsafe { v.clone_as(k) })
                .collect::<Vec<_>>();
            map.push((
                k.codec_write(ctx)?,
                EffectComponentUnion::codec_write_as(v, k, ctx)?,
            ));
        }
        O::create_map(ctx, &map)
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
