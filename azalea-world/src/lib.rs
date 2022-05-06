mod bit_storage;
mod palette;

use azalea_core::ChunkPos;
use azalea_protocol::mc_buf::{McBufReadable, McBufWritable};
pub use bit_storage::BitStorage;
use palette::PalettedContainer;
use std::{
    io::{Read, Write},
    ops::{Index, IndexMut},
    sync::{Arc, Mutex},
};

use crate::palette::PalettedContainerType;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

const SECTION_HEIGHT: u32 = 16;

pub struct World {
    pub storage: ChunkStorage,
    pub height: u32,
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

pub struct ChunkStorage {
    view_center: ChunkPos,
    chunk_radius: u32,
    view_range: u32,
    // chunks is a list of size chunk_radius * chunk_radius
    chunks: Vec<Option<Arc<Mutex<Chunk>>>>,
}

// java moment
// it might be possible to replace this with just a modulo, but i copied java's floorMod just in case
fn floor_mod(x: i32, y: u32) -> u32 {
    if x < 0 {
        y - ((-x) as u32 % y)
    } else {
        x as u32 % y
    }
}

impl ChunkStorage {
    pub fn new(chunk_radius: u32) -> Self {
        let view_range = chunk_radius * 2 + 1;
        ChunkStorage {
            view_center: ChunkPos::new(0, 0),
            chunk_radius,
            view_range,
            chunks: vec![None; (view_range * view_range) as usize],
        }
    }

    fn get_index(&self, chunk_pos: &ChunkPos) -> usize {
        (floor_mod(chunk_pos.x, self.view_range) * self.view_range
            + floor_mod(chunk_pos.z, self.view_range)) as usize
    }

    pub fn in_range(&self, chunk_pos: &ChunkPos) -> bool {
        (chunk_pos.x - self.view_center.x).unsigned_abs() <= self.chunk_radius
            && (chunk_pos.z - self.view_center.z).unsigned_abs() <= self.chunk_radius
    }
}

impl Index<&ChunkPos> for ChunkStorage {
    type Output = Option<Arc<Mutex<Chunk>>>;

    fn index(&self, pos: &ChunkPos) -> &Self::Output {
        &self.chunks[self.get_index(pos)]
    }
}
impl IndexMut<&ChunkPos> for ChunkStorage {
    fn index_mut<'a>(&'a mut self, pos: &ChunkPos) -> &'a mut Self::Output {
        let index = self.get_index(pos);
        &mut self.chunks[index]
    }
}

#[derive(Debug)]
pub struct Chunk {
    pub sections: Vec<Section>,
}

impl Chunk {
    pub fn read_with_world(buf: &mut impl Read, data: &World) -> Result<Self, String> {
        Self::read_with_world_height(buf, data.height)
    }

    pub fn read_with_world_height(buf: &mut impl Read, world_height: u32) -> Result<Self, String> {
        let section_count = world_height / SECTION_HEIGHT;
        let mut sections = Vec::with_capacity(section_count as usize);
        for i in 0..section_count {
            let section = Section::read_into(buf)?;
            sections.push(section);
        }
        Ok(Chunk { sections })
    }
}

impl McBufWritable for Chunk {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        for section in &self.sections {
            section.write_into(buf)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Section {
    pub block_count: u16,
    pub states: PalettedContainer,
    pub biomes: PalettedContainer,
}

impl McBufReadable for Section {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        let block_count = u16::read_into(buf)?;
        // this is commented out because the vanilla server is wrong
        // assert!(
        //     block_count <= 16 * 16 * 16,
        //     "A section has more blocks than what should be possible. This is a bug!"
        // );
        let states = PalettedContainer::read_with_type(buf, &PalettedContainerType::BlockStates)?;
        let biomes = PalettedContainer::read_with_type(buf, &PalettedContainerType::Biomes)?;
        Ok(Section {
            block_count,
            states,
            biomes,
        })
    }
}

impl McBufWritable for Section {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        self.block_count.write_into(buf)?;
        self.states.write_into(buf)?;
        self.biomes.write_into(buf)?;
        Ok(())
    }
}
