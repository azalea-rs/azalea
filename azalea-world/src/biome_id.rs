use std::io;

use azalea_buf::BufReadError;
use azalea_registry::{Registry, data::Biome, registry_holder::RegistryHolder};

/// An unresolved biome.
///
/// This can be freely converted to and from a biome, as long as you have the
/// required registries.
#[derive(PartialEq, Eq, Default, Debug, Clone, Copy)]
pub struct BiomeId(pub u32);
impl From<u32> for BiomeId {
    fn from(id: u32) -> Self {
        Self(id)
    }
}
impl From<BiomeId> for u32 {
    fn from(b: BiomeId) -> Self {
        b.0
    }
}

impl BiomeId {
    /// Resolve this ID into a proper biome.
    pub fn into_biome(&self, registries: &RegistryHolder) -> Result<Biome, BufReadError> {
        Biome::from_id(registries.into(), self.0)
    }
    /// Convert a biome into its ID.
    pub fn from_biome(registries: &RegistryHolder, biome: Biome) -> io::Result<BiomeId> {
        biome.to_id(registries.into()).map(BiomeId)
    }
}
