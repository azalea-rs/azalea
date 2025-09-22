use azalea_registry::DataRegistry;
use simdnbt::owned::NbtCompound;

use crate::{
    registry_holder::{self, RegistryDeserializesTo, RegistryHolder},
    resource_location::ResourceLocation,
};

pub trait ResolvableDataRegistry: DataRegistry {
    type DeserializesTo: RegistryDeserializesTo;

    fn resolve_name<'a>(&self, registries: &'a RegistryHolder) -> Option<&'a ResourceLocation> {
        // self.resolve(registries).map(|(name, _)| name.clone())
        registries.protocol_id_to_resource_location(
            ResourceLocation::from(Self::NAME),
            self.protocol_id(),
        )
    }

    fn resolve<'a>(
        &self,
        registries: &'a RegistryHolder,
    ) -> Option<(&'a ResourceLocation, &'a Self::DeserializesTo)> {
        Self::DeserializesTo::get_for_registry(registries, Self::NAME, self.protocol_id())
    }
}

macro_rules! define_deserializes_to {
    ($($t:ty => $deserializes_to:ty),* $(,)?) => {
        $(
            impl ResolvableDataRegistry for $t {
                type DeserializesTo = $deserializes_to;
            }
        )*
    };
}
macro_rules! define_default_deserializes_to {
    ($($t:ty),* $(,)?) => {
        $(
            impl ResolvableDataRegistry for $t {
                type DeserializesTo = NbtCompound;
            }
        )*
    };
}

define_deserializes_to! {
    azalea_registry::DimensionType => registry_holder::dimension_type::DimensionTypeElement,
}

define_default_deserializes_to! {
    azalea_registry::Enchantment,
    azalea_registry::DamageKind,
    azalea_registry::Dialog,
    azalea_registry::WolfSoundVariant,
    azalea_registry::CowVariant,
    azalea_registry::ChickenVariant,
    azalea_registry::FrogVariant,
    azalea_registry::CatVariant,
    azalea_registry::PigVariant,
    azalea_registry::PaintingVariant,
    azalea_registry::WolfVariant,
    azalea_registry::Biome,
}
