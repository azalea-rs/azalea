#![feature(int_roundings)]

mod bit_storage;
mod chunk;
mod entity;
mod palette;

use azalea_block::BlockState;
use azalea_core::{BlockPos, ChunkBlockPos, ChunkPos, ChunkSectionBlockPos};
use azalea_protocol::mc_buf::{McBufReadable, McBufWritable};
pub use bit_storage::BitStorage;
pub use chunk::{Chunk, ChunkStorage};
pub use entity::EntityStorage;
use std::{
    io::{Read, Write},
    ops::{Index, IndexMut},
    sync::{Arc, Mutex},
};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub struct World {
    pub storage: ChunkStorage,
    pub entities: EntityStorage,
    pub height: u32,
    pub min_y: i32,
}

impl World {
    pub fn replace_with_packet_data(
        &mut self,
        pos: &ChunkPos,
        data: &mut impl Read,
    ) -> Result<(), String> {
        if !self.storage.in_range(pos) {
            println!(
                "Ignoring chunk since it's not in the view range: {}, {}",
                pos.x, pos.z
            );
            return Ok(());
        }
        // let existing_chunk = &self.storage[pos];

        let chunk = Arc::new(Mutex::new(Chunk::read_with_world(data, self)?));
        println!("Loaded chunk {:?}", pos);
        self.storage[pos] = Some(chunk);

        Ok(())
    }

    pub fn update_view_center(&mut self, pos: &ChunkPos) {
        self.storage.view_center = *pos;
    }

    pub fn get_block_state(&self, pos: &BlockPos) -> Option<BlockState> {
        self.storage.get_block_state(pos, self.min_y)
    }
}
impl Index<&ChunkPos> for World {
    type Output = Option<Arc<Mutex<Chunk>>>;

    fn index(&self, pos: &ChunkPos) -> &Self::Output {
        &self.storage[pos]
    }
}
impl IndexMut<&ChunkPos> for World {
    fn index_mut<'a>(&'a mut self, pos: &ChunkPos) -> &'a mut Self::Output {
        &mut self.storage[pos]
    }
}
// impl Index<&BlockPos> for World {
//     type Output = Option<Arc<Mutex<Chunk>>>;

//     fn index(&self, pos: &BlockPos) -> &Self::Output {
//         let chunk = &self[ChunkPos::from(pos)];
//         // chunk.

//     }
// }
