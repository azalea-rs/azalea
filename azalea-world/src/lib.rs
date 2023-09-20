#![doc = include_str!("../README.md")]
#![feature(int_roundings)]
#![feature(error_generic_member_access)]

mod bit_storage;
mod chunk_storage;
mod container;
pub mod heightmap;
pub mod iterators;
pub mod palette;
mod world;

use std::backtrace::Backtrace;

pub use bit_storage::BitStorage;
pub use chunk_storage::{
    calculate_chunk_storage_range, Chunk, ChunkStorage, PartialChunkStorage, Section,
};
pub use container::*;
use thiserror::Error;
pub use world::*;

#[derive(Error, Debug)]
pub enum MoveEntityError {
    #[error("Entity doesn't exist")]
    EntityDoesNotExist(Backtrace),
}
