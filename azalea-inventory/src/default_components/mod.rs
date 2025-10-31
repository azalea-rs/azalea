pub mod generated;

use azalea_registry::Item;

use crate::components::DataComponentTrait;

/// A trait for data components that some [`Item`]s may have a default value
/// for.
pub trait DefaultableComponent: DataComponentTrait {
    fn default_for_item(item: Item) -> Option<Self>
    where
        Self: Sized;
}
impl<T: DataComponentTrait> DefaultableComponent for T {
    default fn default_for_item(_item: Item) -> Option<Self> {
        None
    }
}
#[must_use]
pub fn get_default_component<T: DefaultableComponent>(item: Item) -> Option<T> {
    T::default_for_item(item)
}
