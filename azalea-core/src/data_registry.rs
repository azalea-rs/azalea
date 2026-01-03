use azalea_registry::{
    DataRegistry, DataRegistryKey, DataRegistryKeyRef,
    data::{self},
    identifier::Identifier,
};
use simdnbt::owned::NbtCompound;

use crate::registry_holder::{self, RegistryDeserializesTo, RegistryHolder};

pub trait DataRegistryWithKey: DataRegistry {
    fn key<'s, 'a: 's>(
        &'s self,
        registries: &'a RegistryHolder,
    ) -> Option<<Self::Key as DataRegistryKey>::Borrow<'s>> {
        registries
            .protocol_id_to_identifier(Identifier::from(Self::NAME), self.protocol_id())
            .map(DataRegistryKeyRef::from_ident)
    }
}
impl<R: DataRegistry> DataRegistryWithKey for R {}

pub trait ResolvableDataRegistry: DataRegistry {
    type DeserializesTo: RegistryDeserializesTo;

    #[doc(hidden)]
    #[deprecated = "use `DataRegistryWithKey::key` instead."]
    fn resolve_name<'a>(&self, registries: &'a RegistryHolder) -> Option<&'a Identifier> {
        registries.protocol_id_to_identifier(Identifier::from(Self::NAME), self.protocol_id())
    }

    fn resolve<'a>(
        &self,
        registries: &'a RegistryHolder,
    ) -> Option<(&'a Identifier, &'a Self::DeserializesTo)> {
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
    data::DimensionKind => registry_holder::dimension_type::DimensionKindElement,
    data::Enchantment => registry_holder::enchantment::EnchantmentData,
}

define_default_deserializes_to! {
    data::DamageKind,
    data::Dialog,
    data::WolfSoundVariant,
    data::CowVariant,
    data::ChickenVariant,
    data::FrogVariant,
    data::CatVariant,
    data::PigVariant,
    data::PaintingVariant,
    data::WolfVariant,
    data::Biome,
}
