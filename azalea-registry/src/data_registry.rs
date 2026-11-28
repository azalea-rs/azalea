use std::io;

use azalea_buf::BufReadError;
use simdnbt::owned::NbtCompound;

use crate::{
    Registry,
    data::{self},
    registry_holder::{RegistryDeserializesTo, RegistryHolder},
};

pub(crate) fn protocol_id_to_registry<R: Registry>(
    ctx: azalea_buf::Ctx,
    id: u32,
) -> Result<R, BufReadError> {
    let registry_holder = <&RegistryHolder>::from(ctx);
    let ident = registry_holder
        .protocol_id_to_identifier(R::NAME.into(), id)
        .ok_or_else(|| {
            BufReadError::custom(format!(
                "got a resolvable {} with id {id}, which isn't in our registry",
                R::NAME
            ))
        })?;
    let ident = ident.clone();
    R::try_from(ident).map_err(|_| panic!("identifier returned by `protocol_id_to_identifier` should always be valid for this registry"))
}
pub(crate) fn registry_to_protocol_id<R: Registry>(
    registry: &R,
    ctx: azalea_buf::Ctx,
) -> io::Result<u32> {
    let registry_holder = <&RegistryHolder>::from(ctx);
    let ident = registry.clone().into();
    let id = registry_holder
        .identifier_to_protocol_id(R::NAME, ident.clone())
        .ok_or_else(|| {
            io::Error::other(format!(
                "got a resolvable {} with value {ident}, which isn't in our registry",
                R::NAME
            ))
        })?;
    Ok(id)
}

pub trait DeserializableDataRegistry: Registry {
    type DeserializesTo: RegistryDeserializesTo;

    fn resolve<'a>(&self, registries: &'a RegistryHolder) -> Option<&'a Self::DeserializesTo> {
        let protocol_id = registry_to_protocol_id(self, azalea_buf::Ctx::new(registries)).ok()?;
        Self::DeserializesTo::get_for_registry(registries, Self::NAME, protocol_id)
    }
}

macro_rules! define_deserializes_to {
    ($($t:ty => $deserializes_to:ty),* $(,)?) => {
        $(
            impl DeserializableDataRegistry for $t {
                type DeserializesTo = $deserializes_to;
            }
        )*
    };
}
macro_rules! define_default_deserializes_to {
    ($($t:ty),* $(,)?) => {
        $(
            impl DeserializableDataRegistry for $t {
                type DeserializesTo = NbtCompound;
            }
        )*
    };
}

define_deserializes_to! {
    data::DimensionKind => crate::registry_holder::dimension_type::DimensionKindElement,
    data::Enchantment => crate::registry_holder::enchantment::EnchantmentData,
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
