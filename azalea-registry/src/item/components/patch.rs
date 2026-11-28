use std::{
    fmt::Debug,
    io::{self, Cursor, Write},
    sync::LazyLock,
};

use azalea_buf::{AzBuf, AzBufVar, BufReadError};
use azalea_codec::{Codec, MapLike};
use azalea_core::identifier::Identifier;
use indexmap::IndexMap;

use crate::{
    builtin::DataComponentKind,
    item::components::{self, DataComponent},
};

/// An update to an item's data components.
///
/// Note that in vanilla items come with their own set of default components,
/// and Azalea does not implement that yet.
#[derive(Default, Clone, Debug, PartialEq)]
pub struct DataComponentPatch(Box<IndexMap<DataComponentKind, Option<DataComponent>>>);

pub static EMPTY_DATA_COMPONENT_PATCH: LazyLock<DataComponentPatch> =
    LazyLock::new(DataComponentPatch::default);

impl DataComponentPatch {
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the value of the component in the generic argument for this
    /// item.
    ///
    /// ```
    /// # use azalea_registry::{
    /// #     builtin::ItemKind,
    /// #     item::{ItemStackData, components, components::patch::DataComponentPatch},
    /// # };
    /// # fn example(item: &ItemStackData) -> Option<()> {
    /// let item_nutrition = item.component_patch.get::<components::Food>()?.nutrition;
    /// # Some(())
    /// # }
    /// ```
    pub fn get<T: components::DataComponentTrait>(&self) -> Option<&T> {
        let component = self.get_kind(T::KIND)?;
        Some(T::try_from_data_component(component).expect("mismatching component kind"))
    }

    pub fn get_kind(&self, kind: DataComponentKind) -> Option<&DataComponent> {
        if let Some(Some(r)) = self.0.get(&kind) {
            assert_eq!(r.kind(), kind);
            Some(r)
        } else {
            None
        }
    }
    /// Returns whether the component in the generic argument is present for
    /// this item.
    ///
    /// ```
    /// # use azalea_registry::{
    /// #     builtin::ItemKind,
    /// #     item::{ItemStackData, components, components::patch::DataComponentPatch},
    /// # };
    /// # let item = ItemStackData::from(ItemKind::Stone);
    /// let is_edible = item.component_patch.has::<components::Food>();
    /// # assert!(!is_edible);
    /// ```
    pub fn has<T: components::DataComponentTrait>(&self) -> bool {
        self.has_kind(T::KIND)
    }

    pub fn has_kind(&self, kind: DataComponentKind) -> bool {
        self.get_kind(kind).is_some()
    }

    pub fn iter<'a>(
        &'a self,
    ) -> impl Iterator<Item = (&'a DataComponentKind, &'a Option<DataComponent>)> + 'a {
        self.0.iter()
    }
    /// Insert a new component into this patch, or mark a component as removed.
    ///
    /// # Panics
    ///
    /// The [`DataComponent`] must be of the correct kind.
    pub fn insert_component(&mut self, kind: DataComponentKind, value: Option<DataComponent>) {
        if let Some(value) = &value {
            assert_eq!(kind, value.kind());
        }
        self.0.insert(kind, value);
    }
}

impl AzBuf for DataComponentPatch {
    fn azalea_read(ctx: azalea_buf::Ctx, buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let components_with_data_count = u32::azalea_read_var(ctx, buf)?;
        let components_without_data_count = u32::azalea_read_var(ctx, buf)?;

        if components_without_data_count == 0 && components_with_data_count == 0 {
            return Ok(DataComponentPatch::default());
        }

        let mut components = DataComponentPatch::new();
        for _ in 0..components_with_data_count {
            let component_kind = DataComponentKind::azalea_read(ctx, buf)?;
            let component_data = DataComponent::azalea_read_as(component_kind, ctx, buf)?;
            components.insert_component(component_kind, Some(component_data));
        }

        for _ in 0..components_without_data_count {
            let component_kind = DataComponentKind::azalea_read(ctx, buf)?;
            components.insert_component(component_kind, None);
        }

        Ok(components)
    }
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        let mut components_with_data_count: u32 = 0;
        let mut components_without_data_count: u32 = 0;
        for component in self.0.values() {
            if component.is_some() {
                components_with_data_count += 1;
            } else {
                components_without_data_count += 1;
            }
        }

        components_with_data_count.azalea_write_var(buf)?;
        components_without_data_count.azalea_write_var(buf)?;

        let mut component_buf = Vec::new();
        for (&kind, component) in self.0.iter() {
            if let Some(component) = component {
                kind.azalea_write(buf)?;

                component_buf.clear();
                assert_eq!(component.kind(), kind);
                component.azalea_write_value(&mut component_buf)?;
                buf.write_all(&component_buf)?;
            }
        }

        for (kind, component) in self.0.iter() {
            if component.is_none() {
                kind.azalea_write(buf)?;
            }
        }

        Ok(())
    }
}

impl<O: azalea_codec::Ops> Codec<O> for DataComponentPatch {
    fn codec_read(ops: &O, ctx: azalea_codec::Ctx<O>) -> azalea_codec::error::ReadResult<Self> {
        let map = ops.get_map(ctx)?;
        let mut patch = DataComponentPatch::new();

        for (kind, value) in map.to_values() {
            let kind = InvertableDataComponentKind::codec_read(&kind, ctx)?;
            match kind {
                InvertableDataComponentKind::Present(kind) => {
                    let value = DataComponent::codec_read_as(kind, &value, ctx)
                        .map_err(|e| e.while_reading_key(kind.to_str()))?;
                    patch.insert_component(kind, Some(value));
                }
                InvertableDataComponentKind::Removed(kind) => patch.insert_component(kind, None),
            }
        }

        Ok(patch)
    }
    fn codec_write(&self, ctx: azalea_codec::Ctx<O>) -> azalea_codec::error::WriteResult<O> {
        let mut entries = Vec::<(O, O)>::with_capacity(self.0.len());
        for (&kind, component) in self.0.iter() {
            if let Some(component) = component {
                assert_eq!(component.kind(), kind);
                let k = InvertableDataComponentKind::Present(kind).codec_write(ctx)?;
                let v = component.codec_write_value::<O>(ctx)?;
                entries.push((k, v));
            } else {
                let k = InvertableDataComponentKind::Removed(kind).codec_write(ctx)?;
                let v = ().codec_write(ctx)?;
                entries.push((k, v))
            }
        }
        O::create_map(ctx, &entries)
    }
}
azalea_codec::impl_defaultable!(DataComponentPatch, self, self.0.is_empty());

enum InvertableDataComponentKind {
    Present(DataComponentKind),
    Removed(DataComponentKind),
}
impl<O: azalea_codec::Ops> Codec<O> for InvertableDataComponentKind {
    fn codec_read(ops: &O, ctx: azalea_codec::Ctx<O>) -> azalea_codec::error::ReadResult<Self> {
        let ident = Identifier::codec_read(ops, ctx)?;
        if ident.namespace().starts_with('!') {
            let ident = Identifier::new(&ident.to_string()[1..]);
            DataComponentKind::try_from(&ident)
                .map_err(|_| azalea_codec::error::ReadError::custom("not a DataComponentKind"))
                .map(Self::Removed)
        } else {
            DataComponentKind::codec_read(ops, ctx).map(Self::Present)
        }
    }
    fn codec_write(&self, ctx: azalea_codec::Ctx<O>) -> azalea_codec::error::WriteResult<O> {
        match self {
            InvertableDataComponentKind::Present(k) => k.codec_write(ctx),
            InvertableDataComponentKind::Removed(k) => format!("!{k}").codec_write(ctx),
        }
    }
}

#[cfg(test)]
mod tests {
    use azalea_registry_builtin::builtin::ItemKind;

    use crate::item::{ItemStack, components::MapId};

    #[test]
    fn test_get_component() {
        let item = ItemStack::from(ItemKind::Map).with_component(MapId { id: 1 });
        let map_id = item.get_component::<MapId>().unwrap();
        assert_eq!(map_id.id, 1);
    }
}
