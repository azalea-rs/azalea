use azalea_buf::AzBuf;

/// A registry which has its values decided by the server in the
/// `ClientboundRegistryData` packet.
///
/// These can be resolved into their actual values with
/// `ResolvableDataRegistry` from azalea-core.
pub trait DataRegistry {
    const NAME: &'static str;

    fn protocol_id(&self) -> u32;
}

#[derive(Debug, Clone, Copy, AzBuf, PartialEq, Eq, Hash)]
pub struct Enchantment {
    #[var]
    id: u32,
}
impl DataRegistry for Enchantment {
    const NAME: &'static str = "enchantment";
    fn protocol_id(&self) -> u32 {
        self.id
    }
}
