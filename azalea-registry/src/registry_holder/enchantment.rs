use std::{
    any::Any,
    fmt::{self, Debug},
};

use azalea_codec::{Codec, MapLike};
use azalea_registry::builtin::EnchantmentEffectComponentKind;
use indexmap::IndexMap;

use crate::registry_holder::components::{
    EffectComponentTrait, EffectComponentUnion, ResolvedEffectComponent,
};

#[derive(Codec, Clone, Debug)]
pub struct EnchantmentData {
    // TODO: make these two deserializable
    // pub description: TextComponent,
    // pub exclusive_set: HolderSet<Enchantment, ResourceLocation>,
    pub effects: Effects,
}

impl Effects {
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
        self.0.get(&kind).map(|c| {
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

pub struct Effects(IndexMap<EnchantmentEffectComponentKind, Vec<EffectComponentUnion>>);
impl<O: azalea_codec::Ops> Codec<O> for Effects {
    fn codec_read(ops: &O, ctx: azalea_codec::Ctx<O>) -> azalea_codec::error::ReadResult<Self> {
        let map = ops.get_map(ctx)?;
        let mut items = Vec::new();
        map.for_each(|k, v| {
            items.push(
                EnchantmentEffectComponentKind::codec_read(k, ctx)
                    .and_then(|k| EffectComponentUnion::codec_read_as(k, v, ctx).map(|v| (k, v))),
            );
        });
        Ok(Self(
            items
                .into_iter()
                .collect::<azalea_codec::error::ReadResult<_>>()?,
        ))
    }
    fn codec_write(&self, ctx: azalea_codec::Ctx<O>) -> azalea_codec::error::WriteResult<O> {
        let mut map = Vec::new();
        for (&k, v) in &self.0 {
            let v = v
                .iter()
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

impl Clone for Effects {
    fn clone(&self) -> Self {
        let mut effects = IndexMap::with_capacity(self.0.len());
        for (kind, effect) in &self.0 {
            effects.insert(
                *kind,
                effect
                    .iter()
                    .map(|e| unsafe { e.clone_as(*kind) })
                    .collect(),
            );
        }
        Self(effects)
    }
}
impl Debug for Effects {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Effects").finish()
    }
}
