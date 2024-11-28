pub trait MaxStackSizeExt {
    /// Get the maximum stack size for this item.
    ///
    /// This is a signed integer to be consistent with the `count` field of
    /// [`ItemStackData`].
    ///
    /// [`ItemStackData`]: crate::ItemStackData
    fn max_stack_size(&self) -> i32;

    /// Whether this item can be stacked with other items.
    ///
    /// This is equivalent to `self.max_stack_size() > 1`.
    fn stackable(&self) -> bool {
        self.max_stack_size() > 1
    }
}

impl MaxStackSizeExt for azalea_registry::Item {
    fn max_stack_size(&self) -> i32 {
        // TODO: have the properties for every item defined somewhere
        64
    }
}
