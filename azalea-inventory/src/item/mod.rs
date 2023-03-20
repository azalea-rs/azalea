
pub trait MaxStackSizeExt {
    fn max_stack_size(&self) -> u32;
}

impl MaxStackSizeExt for azalea_registry::Item {
    fn max_stack_size(&self) -> u32 {
        // TODO: have the properties for every item defined somewhere
        64
    }
}