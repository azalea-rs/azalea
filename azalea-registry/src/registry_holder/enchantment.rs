use std::fmt::Debug;

use azalea_chat::FormattedText;
use azalea_codec::{Codec, MapLike};
use indexmap::IndexMap;

use crate::{
    HolderSet,
    builtin::EnchantmentEffectComponentKind,
    data::Enchantment,
    registry_holder::components::{EffectComponentTrait, EffectComponents},
};

#[derive(Codec, Clone, Debug)]
#[codec(allow_unknown_keys)]
pub struct EnchantmentData {
    pub description: FormattedText,
    #[codec(optional)]
    pub exclusive_set: HolderSet<Enchantment>,
    #[codec(optional)]
    pub effects: Effects,
}
/// Container for the effects in [`EnchantmentData`].
#[derive(Default, Clone, Debug, PartialEq)]
pub struct Effects(IndexMap<EnchantmentEffectComponentKind, EffectComponents>);

impl Effects {
    pub fn get<T: EffectComponentTrait>(&self) -> Option<&[T]>
    where
        for<'a> &'a [T]: TryFrom<&'a EffectComponents>,
    {
        let components = self.get_kind(T::KIND)?;
        Some(
            components
                .as_kind()
                .expect("Effects should not have mismatching components"),
        )
    }

    pub fn get_kind(&self, kind: EnchantmentEffectComponentKind) -> Option<&EffectComponents> {
        if let Some(r) = self.0.get(&kind) {
            assert_eq!(r.kind(), kind);
            Some(r)
        } else {
            None
        }
    }
}

impl<O: azalea_codec::Ops> Codec<O> for Effects {
    fn codec_read(ops: &O) -> azalea_codec::error::ReadResult<Self> {
        let map = ops.get_map()?;
        let mut items = Vec::new();
        map.for_each(|k, v| {
            items.push(
                EnchantmentEffectComponentKind::codec_read(k)
                    .and_then(|k| EffectComponents::codec_read_as(k, v).map(|v| (k, v))),
            );
        });
        Ok(Self(
            items
                .into_iter()
                .collect::<azalea_codec::error::ReadResult<_>>()?,
        ))
    }
    fn codec_write(&self) -> azalea_codec::error::WriteResult<O> {
        let mut map = Vec::new();
        for (&k, v) in &self.0 {
            assert_eq!(v.kind(), k);
            map.push((k.codec_write()?, EffectComponents::codec_write_values(v)?));
        }
        O::create_map(&map)
    }
}

azalea_codec::impl_defaultable!(Effects);
