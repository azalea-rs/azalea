use std::{io::Cursor, str::FromStr};

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

    fn resolve_and_deserialize<'a, T: simdnbt::Deserialize>(
        &self,
        registries: &'a RegistryHolder,
    ) -> Option<Result<(ResourceLocation, T), simdnbt::DeserializeError>> {
        let (name, value) = self.resolve(registries)?;

        let mut nbt_bytes = Vec::new();
        value.write(&mut nbt_bytes);
        let nbt_borrow_compound =
            simdnbt::borrow::read_compound(&mut Cursor::new(&nbt_bytes)).ok()?;
        let value = match T::from_compound((&nbt_borrow_compound).into()) {
            Ok(value) => value,
            Err(err) => {
                return Some(Err(err));
            }
        };

        Some(Ok((name.clone(), value)))
    }
}
impl<T: DataRegistry> ResolvableDataRegistry for T {}
