#![doc = include_str!("../README.md")]

mod bit_storage;
pub mod chunk;
mod container;
pub mod find_blocks;
pub mod heightmap;
pub mod iterators;
pub mod palette;
mod world;

pub use bit_storage::BitStorage;
pub use chunk::{Chunk, Section, partial::PartialChunkStorage, storage::ChunkStorage};
pub use container::{WorldName, Worlds};
pub use world::*;

#[deprecated = "renamed to `WorldName`."]
pub type InstanceName = WorldName;
#[deprecated = "renamed to `WorldContainer`."]
pub type InstanceContainer = Worlds;
#[deprecated = "renamed to `PartialWorld`."]
pub type PartialInstance = PartialWorld;
