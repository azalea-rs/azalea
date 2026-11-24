use std::{
    any::Any,
    fmt::{self, Debug},
    io::{self, Cursor, Write},
    sync::LazyLock,
};

use azalea_buf::{AzBuf, AzBufVar, BufReadError};
use azalea_codec::{Codec, MapLike};
use azalea_core::identifier::Identifier;
use indexmap::IndexMap;

use crate::{
    builtin::DataComponentKind,
    item::components::{self, DataComponentUnion, EncodableDataComponent},
};

/// An update to an item's data components.
///
/// Note that in vanilla items come with their own set of default components,
/// and Azalea does not implement that yet.
#[derive(Default)]
pub struct DataComponentPatch {
    components: Box<IndexMap<DataComponentKind, Option<DataComponentUnion>>>,
}

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
        let component_any = component as &dyn Any;
        component_any.downcast_ref::<T>()
    }

    pub fn get_kind(
        &self,
        kind: DataComponentKind,
    ) -> Option<&dyn components::EncodableDataComponent> {
        self.components.get(&kind).and_then(|c| {
            c.as_ref().map(|c| {
                // SAFETY: we just got the component from the map, so it must be
                // the correct kind
                unsafe { c.as_kind(kind) }
            })
        })
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
    ) -> impl Iterator<Item = (DataComponentKind, Option<&'a dyn EncodableDataComponent>)> + 'a
    {
        self.components.iter().map(|(&kind, component)| {
            component
                .as_ref()
                .map_or((kind, None), |c| (kind, unsafe { Some(c.as_kind(kind)) }))
        })
    }
    /// Insert a new component into this patch, or mark a component as removed.
    ///
    /// # Safety
    /// The [`DataComponentUnion`] must be of the correct kind.
    pub unsafe fn unchecked_insert_component(
        &mut self,
        kind: DataComponentKind,
        value: Option<DataComponentUnion>,
    ) {
        let existing = self.components.insert(kind, value);
        if let Some(Some(mut existing)) = existing {
            // SAFETY: we just got it from self.components, so it must already
            // be the correct type
            unsafe { existing.drop_as(kind) };
        }
    }
}

impl Drop for DataComponentPatch {
    fn drop(&mut self) {
        // the component values are ManuallyDrop since they're in a union
        for (kind, component) in self.components.iter_mut() {
            if let Some(component) = component {
                // SAFETY: we got the kind and component from the map
                unsafe { component.drop_as(*kind) };
            }
        }
    }
}

impl AzBuf for DataComponentPatch {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let components_with_data_count = u32::azalea_read_var(buf)?;
        let components_without_data_count = u32::azalea_read_var(buf)?;

        if components_without_data_count == 0 && components_with_data_count == 0 {
            return Ok(DataComponentPatch::default());
        }

        let mut components = DataComponentPatch::new();
        for _ in 0..components_with_data_count {
            let component_kind = DataComponentKind::azalea_read(buf)?;
            let component_data = DataComponentUnion::azalea_read_as(component_kind, buf)?;
            // SAFETY: it must be of the correct type because we just read using
            // azalea_read_as
            unsafe { components.unchecked_insert_component(component_kind, Some(component_data)) };
        }

        for _ in 0..components_without_data_count {
            let component_kind = DataComponentKind::azalea_read(buf)?;
            // SAFETY: the value is None so the kind doesn't matter anyways
            unsafe { components.unchecked_insert_component(component_kind, None) };
        }

        Ok(components)
    }
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        let mut components_with_data_count: u32 = 0;
        let mut components_without_data_count: u32 = 0;
        for component in self.components.values() {
            if component.is_some() {
                components_with_data_count += 1;
            } else {
                components_without_data_count += 1;
            }
        }

        components_with_data_count.azalea_write_var(buf)?;
        components_without_data_count.azalea_write_var(buf)?;

        let mut component_buf = Vec::new();
        for (kind, component) in self.components.iter() {
            if let Some(component) = component {
                kind.azalea_write(buf)?;

                component_buf.clear();
                // SAFETY: we got the component from the map and are passing in
                // the same kind
                unsafe { component.azalea_write_as(*kind, &mut component_buf) }?;
                buf.write_all(&component_buf)?;
            }
        }

        for (kind, component) in self.components.iter() {
            if component.is_none() {
                kind.azalea_write(buf)?;
            }
        }

        Ok(())
    }
}

impl Clone for DataComponentPatch {
    fn clone(&self) -> Self {
        let mut components = IndexMap::with_capacity(self.components.len());
        for (kind, component) in self.components.iter() {
            components.insert(
                *kind,
                component.as_ref().map(|c| unsafe { c.clone_as(*kind) }),
            );
        }
        DataComponentPatch {
            components: Box::new(components),
        }
    }
}
impl Debug for DataComponentPatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.components.keys()).finish()
    }
}
impl PartialEq for DataComponentPatch {
    fn eq(&self, other: &Self) -> bool {
        if self.components.len() != other.components.len() {
            return false;
        }
        for (kind, component) in self.components.iter() {
            let Some(other_component) = other.components.get(kind) else {
                return false;
            };
            // we can't use PartialEq, but we can use our own eq method
            if let Some(component) = component {
                let Some(other_component) = other_component else {
                    return false;
                };
                // SAFETY: we already checked that the kinds are the same, and
                // we got the components from the map, so they
                // must be the correct kinds
                if !unsafe { component.eq_as(other_component, *kind) } {
                    return false;
                }
            } else if other_component.is_some() {
                return false;
            }
        }
        true
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
                    let value = DataComponentUnion::codec_read_as(&value, kind, ctx)?;
                    unsafe { patch.unchecked_insert_component(kind, Some(value)) };
                }
                InvertableDataComponentKind::Removed(kind) => unsafe {
                    patch.unchecked_insert_component(kind, None)
                },
            }
        }

        Ok(patch)
    }
    fn codec_write(&self, ctx: azalea_codec::Ctx<O>) -> azalea_codec::error::WriteResult<O> {
        let mut entries = Vec::<(O, O)>::with_capacity(self.components.len());
        for (&kind, component) in self.components.iter() {
            if let Some(component) = component {
                let k = InvertableDataComponentKind::Present(kind).codec_write(ctx)?;
                let v = unsafe { component.codec_write_as::<O>(ctx, kind) }?;
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
impl<O: azalea_codec::Ops> azalea_codec::Defaultable<O> for DataComponentPatch {
    fn is_default(&self) -> bool {
        self.components.is_empty()
    }
}

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
