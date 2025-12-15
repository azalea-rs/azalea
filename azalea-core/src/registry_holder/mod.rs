//! The data sent to the client in the `ClientboundRegistryDataPacket`.
//!
//! This module contains the structures used to represent the registry
//! sent to the client upon login. This contains a lot of information about
//! the game, including the types of chat messages, dimensions, and
//! biomes.

pub mod block_predicate;
pub mod block_state_provider;
pub mod components;
pub mod dimension_type;
pub mod enchantment;
pub mod entity_effect;
pub mod float_provider;
pub mod value;

use std::{collections::HashMap, io::Cursor};

use azalea_registry::identifier::Identifier;
use indexmap::IndexMap;
use simdnbt::{DeserializeError, FromNbtTag, borrow, owned::NbtCompound};
use thiserror::Error;
use tracing::error;

/// The base of the registry.
///
/// This is the registry that is sent to the client upon login.
///
/// Note that `azalea-client` stores registries in `Instance` rather than
/// per-client like you might expect. This is an optimization for swarms to
/// reduce memory usage, since registries are expected to be the same for every
/// client in a world.
#[derive(Clone, Debug, Default)]
pub struct RegistryHolder {
    // if you add new fields here, don't forget to also update `RegistryHolder::append`,
    // `protocol_id_to_identifier`, and `define_default_deserializes_to!` in
    // `data_registry.rs`.
    #[rustfmt::skip] // allow empty line

    /// Attributes about the dimension.
    pub dimension_type: RegistryType<dimension_type::DimensionKindElement>,

    pub enchantment: RegistryType<enchantment::EnchantmentData>,

    /// Registries that we haven't implemented deserializable types for.
    ///
    /// You can still access these just fine, but they'll be NBT instead of
    /// nicer structs.
    pub extra: HashMap<Identifier, RegistryType<NbtCompound>>,
}

macro_rules! registry_holder {
    ($($registry:ident),* $(,)?) => {
        impl RegistryHolder {
            pub fn append(
                &mut self,
                id: Identifier,
                entries: Vec<(Identifier, Option<NbtCompound>)>,
            ) {

                if id.namespace() == "minecraft" {
                    match id.path() {
                        $(
                            stringify!($registry) => {
                                return self.$registry.append_nbt(id, entries);
                            }
                        )*
                        _ => {}
                    }
                }

                self.extra
                    .entry(id.clone())
                    .or_default()
                    .append_nbt(id, entries);
            }

            pub fn extend(&mut self, other: RegistryHolder) {
                $(
                    self.$registry = other.$registry;
                )*
                self.extra.extend(other.extra);
            }

            /// Convert a protocol ID for a registry key (like the protocol_id for
            /// something that implements `DataRegistry`) and convert it to its string
            /// name.
            pub fn protocol_id_to_identifier(
                &self,
                registry: Identifier,
                protocol_id: u32,
            ) -> Option<&Identifier> {
                let index = protocol_id as usize;

                if registry.namespace() == "minecraft" {
                    match registry.path() {
                        $(
                            stringify!($registry) => {
                                return self.$registry.map.get_index(index).map(|(k, _)| k);
                            }
                        )*
                        _ => {}
                    }
                }

                self.extra
                    .get(&registry)
                    .and_then(|r| r.map.get_index(index))
                    .map(|(k, _)| k)
            }
        }
    };
}

registry_holder!(dimension_type, enchantment);

fn nbt_to_serializable_type<T: simdnbt::Deserialize>(
    value: &NbtCompound,
) -> Result<T, NbtToSerializableTypeError> {
    // convert the value to T
    let mut nbt_bytes = Vec::new();
    value.write(&mut nbt_bytes);
    let nbt_borrow_compound = simdnbt::borrow::read_compound(&mut Cursor::new(&nbt_bytes))?;
    T::from_compound((&nbt_borrow_compound).into()).map_err(Into::into)
}

#[derive(Debug, Error)]
enum NbtToSerializableTypeError {
    #[error(transparent)]
    NbtError(#[from] simdnbt::Error),
    #[error(transparent)]
    DeserializeError(#[from] simdnbt::DeserializeError),
}

/// A collection of values for a certain type of registry data.
#[derive(Clone, Debug)]
pub struct RegistryType<T: simdnbt::Deserialize> {
    pub map: IndexMap<Identifier, T>,
}

impl<T: simdnbt::Deserialize> Default for RegistryType<T> {
    fn default() -> Self {
        Self {
            map: IndexMap::new(),
        }
    }
}

impl<T: simdnbt::Deserialize> RegistryType<T> {
    fn append_nbt(&mut self, id: Identifier, entries: Vec<(Identifier, Option<NbtCompound>)>) {
        let map = &mut self.map;
        for (key, value) in entries {
            if let Some(value) = value {
                match nbt_to_serializable_type(&value) {
                    Ok(value) => {
                        map.insert(key, value);
                    }
                    Err(err) => {
                        error!("Error deserializing {id} entry {key}: {err:?}\n{value:?}");
                    }
                }
            } else {
                map.shift_remove(&key);
            }
        }
    }
}

pub trait RegistryDeserializesTo: simdnbt::Deserialize {
    fn get_for_registry<'a>(
        registries: &'a RegistryHolder,
        registry_name: &'static str,
        protocol_id: u32,
    ) -> Option<(&'a Identifier, &'a Self)>;
}

impl RegistryDeserializesTo for NbtCompound {
    fn get_for_registry<'a>(
        registries: &'a RegistryHolder,
        registry_name: &'static str,
        protocol_id: u32,
    ) -> Option<(&'a Identifier, &'a Self)> {
        registries
            .extra
            .get(&Identifier::new(registry_name))?
            .map
            .get_index(protocol_id as usize)
    }
}
impl RegistryDeserializesTo for dimension_type::DimensionKindElement {
    fn get_for_registry<'a>(
        registries: &'a RegistryHolder,
        registry_name: &'static str,
        protocol_id: u32,
    ) -> Option<(&'a Identifier, &'a Self)> {
        if registry_name != "dimension_type" {
            error!(
                "called RegistryDeserializesTo::get_for_registry with the wrong registry: {registry_name}"
            );
        }
        registries
            .dimension_type
            .map
            .get_index(protocol_id as usize)
    }
}
impl RegistryDeserializesTo for enchantment::EnchantmentData {
    fn get_for_registry<'a>(
        registries: &'a RegistryHolder,
        registry_name: &'static str,
        protocol_id: u32,
    ) -> Option<(&'a Identifier, &'a Self)> {
        if registry_name != "enchantment" {
            error!(
                "called RegistryDeserializesTo::get_for_registry with the wrong registry: {registry_name}"
            );
        }
        registries.enchantment.map.get_index(protocol_id as usize)
    }
}

pub fn get_in_compound<T: FromNbtTag>(
    compound: &borrow::NbtCompound,
    key: &str,
) -> Result<T, DeserializeError> {
    let value = compound.get(key).ok_or(DeserializeError::MissingField)?;
    T::from_nbt_tag(value).ok_or(DeserializeError::MissingField)
}
