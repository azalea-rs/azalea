#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![doc = include_str!("../README.md")]

pub mod aabb;
pub mod bitset;
pub mod checksum;
pub mod codec_utils;
pub mod color;
pub mod cursor3d;
pub mod data_registry;
pub mod delta;
pub mod difficulty;
pub mod direction;
pub mod filterable;
pub mod game_type;
pub mod hit_result;
pub mod math;
pub mod objectives;
pub mod position;
pub mod registry_holder;
pub mod resource_location;
pub mod sound;
#[cfg(feature = "bevy_ecs")]
pub mod tick;
pub mod tier;
