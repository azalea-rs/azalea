use std::{collections::BTreeMap, sync::LazyLock};

use azalea_codec::Codec;

use crate::{
    builtin::ItemKind,
    item::components::{DataComponentTrait, patch::DataComponentPatch},
};

#[derive(Codec)]
#[codec(transparent)]
pub struct AllDefaultComponents {
    pub data: BTreeMap<ItemKind, DataComponentPatch>,
}

fn uncached_read_default_components() -> AllDefaultComponents {
    // we read components at runtime to make the code simpler since we can just
    // reuse our Codec impls

    let data = include_str!("item.json");
    let mut data = data.to_owned().into_bytes();
    let json = simd_json::from_slice::<simd_json::OwnedValue>(&mut data).unwrap();
    //
    AllDefaultComponents::codec_read(&json).unwrap()
}
pub static ALL_DEFAULT_COMPONENTS: LazyLock<AllDefaultComponents> =
    LazyLock::new(uncached_read_default_components);

/// A trait for data components that some [`ItemKind`]s may have a default value
/// for.
pub trait DefaultableComponent: DataComponentTrait {
    fn default_for_item(item: ItemKind) -> Option<&'static Self>
    where
        Self: Sized;
}
impl<T: DataComponentTrait> DefaultableComponent for T {
    default fn default_for_item(item: ItemKind) -> Option<&'static Self> {
        let a = ALL_DEFAULT_COMPONENTS.data.get(&item)?;
        a.get::<Self>()
    }
}
pub fn get_default_component<T: DefaultableComponent>(item: ItemKind) -> Option<&'static T> {
    T::default_for_item(item)
}

#[cfg(test)]
mod tests {
    use azalea_registry_builtin::builtin::ItemKind;

    use crate::{Registry, item::default_components::ALL_DEFAULT_COMPONENTS};

    #[test]
    fn default_components() {
        let defaults = &*ALL_DEFAULT_COMPONENTS;

        // and now make sure all items are represented
        for id in 0..=ItemKind::MAX_ID {
            let item = ItemKind::from_id(Default::default(), id).unwrap();
            let item_defaults = defaults.data.get(&item);
            assert!(item_defaults.is_some());
        }
    }
}
