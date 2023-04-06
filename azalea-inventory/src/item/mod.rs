pub trait MaxStackSizeExt {
    /// Get the maximum stack size for this item.
    ///
    /// This is a signed integer to be consistent with the `count` field of
    /// [`ItemSlotData`].
    fn max_stack_size(&self) -> i8;
}

impl MaxStackSizeExt for azalea_registry::Item {
    fn max_stack_size(&self) -> i8 {
        // TODO: have the properties for every item defined somewhere
        64
    }
}
