pub mod generated;

use azalea_registry::Item;

use crate::components::DataComponent;

/// A [`DataComponent`] that some [`Item`]s may have a default value for.
pub trait DefaultableComponent: DataComponent {
    fn default_for_item(item: Item) -> Option<Self>
    where
        Self: Sized;
}
pub fn get_default_component<T: DefaultableComponent>(item: Item) -> Option<T> {
    T::default_for_item(item)
}
