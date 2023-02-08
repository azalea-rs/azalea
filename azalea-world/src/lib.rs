#![doc = include_str!("../README.md")]
#![feature(int_roundings)]
#![feature(error_generic_member_access)]
#![feature(provide_any)]

mod bit_storage;
mod chunk_storage;
mod container;
pub mod entity;
mod palette;
mod world;

use std::backtrace::Backtrace;

pub use bit_storage::BitStorage;
pub use chunk_storage::{Chunk, ChunkStorage, PartialChunkStorage};
pub use container::*;
use thiserror::Error;
pub use world::*;

#[derive(Error, Debug)]
pub enum MoveEntityError {
    #[error("Entity doesn't exist")]
    EntityDoesNotExist(Backtrace),
}
