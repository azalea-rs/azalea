pub mod generated;

use azalea_registry::builtin::ItemKind;

use crate::components::DataComponentTrait;

/// A trait for data components that some [`ItemKind`]s may have a default value
/// for.
pub trait DefaultableComponent: DataComponentTrait {
    fn default_for_item(item: ItemKind) -> Option<Self>
    where
        Self: Sized;
}
impl<T: DataComponentTrait> DefaultableComponent for T {
    default fn default_for_item(_item: ItemKind) -> Option<Self> {
        None
    }
}
pub fn get_default_component<T: DefaultableComponent>(item: ItemKind) -> Option<T> {
    T::default_for_item(item)
}
