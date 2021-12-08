use crate::{base_component::BaseComponent, component::Component};

pub trait MutableComponent {
    /// Add a component as a sibling of this one
    fn append(&self, component: Component);
}
