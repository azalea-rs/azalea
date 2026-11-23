#![allow(incomplete_features)]
#![feature(gca_min_const_items, gca_const_items)]
#![feature(generic_const_items)]
#![doc = include_str!("../README.md")]

pub mod aabb;
pub mod attribute_modifier_operation;
pub mod bitset;
pub mod color;
pub mod cursor3d;
pub mod delta;
pub mod difficulty;
pub mod direction;
pub mod entity_id;
pub mod filterable;
pub mod game_type;
pub mod heightmap_kind;
pub mod hit_result;
pub mod identifier;
pub mod math;
pub mod objectives;
pub mod position;
pub mod sound;
#[cfg(feature = "bevy_ecs")]
pub mod tick;
