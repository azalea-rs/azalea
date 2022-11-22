#![feature(int_roundings)]

mod bit_storage;
mod chunk_storage;
mod container;
pub mod entity;
mod entity_storage;
mod palette;
mod world;

pub use bit_storage::BitStorage;
pub use chunk_storage::{Chunk, ChunkStorage, LimitedChunkStorage, WeakChunkStorage};
pub use container::*;
pub use entity_storage::{EntityStorage, WeakEntityStorage};
use thiserror::Error;
pub use world::*;

#[derive(Error, Debug)]
pub enum MoveEntityError {
    #[error("Entity doesn't exist")]
    EntityDoesNotExist,
}
