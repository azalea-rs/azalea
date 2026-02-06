#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![doc = include_str!("../README.md")]

pub mod aabb;
pub mod attribute_modifier_operation;
pub mod bitset;
#[cfg(feature = "serde")]
pub mod checksum;
#[cfg(feature = "serde")]
pub mod codec_utils;
pub mod color;
pub mod cursor3d;
#[cfg(feature = "serde")]
pub mod data_registry;
pub mod delta;
pub mod difficulty;
pub mod direction;
pub mod entity_id;
pub mod filterable;
pub mod game_type;
pub mod heightmap_kind;
pub mod hit_result;
pub mod math;
pub mod objectives;
pub mod position;
#[cfg(feature = "serde")]
pub mod registry_holder;
#[doc(hidden)]
pub mod resource_location {
    #![deprecated(note = "moved to `azalea_registry::identifier`.")]
    #[deprecated(note = "moved to `azalea_registry::identifier::Identifier`.")]
    pub type ResourceLocation = azalea_registry::identifier::Identifier;
}
#[doc(hidden)]
pub mod identifier {
    #![deprecated(note = "moved to `azalea_registry::identifier`.")]
    #[deprecated(note = "moved to `azalea_registry::identifier::Identifier`.")]
    pub type Identifier = azalea_registry::identifier::Identifier;
}
pub mod sound;
#[cfg(feature = "bevy_ecs")]
pub mod tick;
pub mod tier;
