use std::str::FromStr;

use azalea_registry::DataRegistry;
use simdnbt::owned::NbtCompound;

use crate::{registry_holder::RegistryHolder, resource_location::ResourceLocation};

pub trait ResolvableDataRegistry: DataRegistry {
    fn resolve_name(&self, registries: &RegistryHolder) -> Option<ResourceLocation> {
        self.resolve(registries).map(|(name, _)| name.clone())
    }
    fn resolve<'a>(
        &self,
        registries: &'a RegistryHolder,
    ) -> Option<(&'a ResourceLocation, &'a NbtCompound)> {
        let name_resourcelocation = ResourceLocation::from_str(Self::NAME).unwrap_or_else(|_| {
            panic!(
                "Name for registry should be a valid ResourceLocation: {}",
                Self::NAME
            )
        });
        let registry_values = registries.map.get(&name_resourcelocation)?;
        let resolved = registry_values.get_index(self.protocol_id() as usize)?;
        Some(resolved)
    }
}
impl<T: DataRegistry> ResolvableDataRegistry for T {}
