#![doc = include_str!("../README.md")]
#![feature(trait_upcasting)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

pub mod aabb;
pub mod bitset;
pub mod block_hit_result;
pub mod cursor3d;
pub mod delta;
pub mod difficulty;
pub mod direction;
pub mod game_type;
pub mod math;
pub mod objectives;
pub mod position;
pub mod registry_holder;
pub mod resource_location;
#[cfg(feature = "bevy_ecs")]
pub mod tick;
pub mod tier;
