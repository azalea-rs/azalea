use crate::palette::PalettedContainer;
use crate::palette::PalettedContainerType;
use crate::Dimension;
use azalea_block::BlockState;
use azalea_buf::{McBufReadable, McBufWritable};
use azalea_core::floor_mod;
use azalea_core::{BlockPos, ChunkBlockPos, ChunkPos, ChunkSectionBlockPos};
use std::fmt::Debug;
use std::{
    io::{Read, Write},
    ops::{Index, IndexMut},
    sync::{Arc, Mutex},
};

const SECTION_HEIGHT: u32 = 16;

pub struct ChunkStorage {
    pub view_center: ChunkPos,
    chunk_radius: u32,
    view_range: u32,
    pub height: u32,
    pub min_y: i32,
    // chunks is a list of size chunk_radius * chunk_radius
    chunks: Vec<Option<Arc<Mutex<Chunk>>>>,
}

#[derive(Debug)]
pub struct Chunk {
    pub sections: Vec<Section>,
}

#[derive(Clone, Debug)]
pub struct Section {
    pub block_count: u16,
    pub states: PalettedContainer,
    pub biomes: PalettedContainer,
}

impl ChunkStorage {
    pub fn new(chunk_radius: u32, height: u32, min_y: i32) -> Self {
        let view_range = chunk_radius * 2 + 1;
        ChunkStorage {
            view_center: ChunkPos::new(0, 0),
            chunk_radius,
            view_range,
            height,
            min_y,
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

    pub fn get_block_state(&self, pos: &BlockPos, min_y: i32) -> Option<BlockState> {
        let chunk_pos = ChunkPos::from(pos);
        println!("chunk_pos {:?} block_pos {:?}", chunk_pos, pos);
        let chunk = &self[&chunk_pos];
        chunk
            .as_ref()
            .map(|chunk| chunk.lock().unwrap().get(&ChunkBlockPos::from(pos), min_y))
    }

    pub fn replace_with_packet_data(
        &mut self,
        pos: &ChunkPos,
        data: &mut impl Read,
    ) -> Result<(), String> {
        if !self.in_range(pos) {
            println!(
                "Ignoring chunk since it's not in the view range: {}, {}",
                pos.x, pos.z
            );
            return Ok(());
        }

        let chunk = Arc::new(Mutex::new(Chunk::read_with_dimension_height(
            data,
            self.height,
        )?));
        println!("Loaded chunk {:?}", pos);
        self[pos] = Some(chunk);

        Ok(())
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

impl Chunk {
    pub fn read_with_dimension(buf: &mut impl Read, data: &Dimension) -> Result<Self, String> {
        Self::read_with_dimension_height(buf, data.height())
    }

    pub fn read_with_dimension_height(
        buf: &mut impl Read,
        dimension_height: u32,
    ) -> Result<Self, String> {
        let section_count = dimension_height / SECTION_HEIGHT;
        let mut sections = Vec::with_capacity(section_count as usize);
        for _ in 0..section_count {
            let section = Section::read_from(buf)?;
            sections.push(section);
        }
        Ok(Chunk { sections })
    }

    pub fn section_index(&self, y: i32, min_y: i32) -> u32 {
        // TODO: check the build height and stuff, this code will be broken if the min build height is 0
        // (LevelHeightAccessor.getMinSection in vanilla code)
        assert!(y >= 0);
        let min_section_index = min_y.div_floor(16);
        (y.div_floor(16) - min_section_index) as u32
    }

    pub fn get(&self, pos: &ChunkBlockPos, min_y: i32) -> BlockState {
        let section_index = self.section_index(pos.y, min_y);
        // TODO: make sure the section exists
        let section = &self.sections[section_index as usize];
        let chunk_section_pos = ChunkSectionBlockPos::from(pos);
        section.get(chunk_section_pos)
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

impl Debug for ChunkStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChunkStorage")
            .field("view_center", &self.view_center)
            .field("chunk_radius", &self.chunk_radius)
            .field("view_range", &self.view_range)
            .field("height", &self.height)
            .field("min_y", &self.min_y)
            // .field("chunks", &self.chunks)
            .field("chunks", &format_args!("{} items", self.chunks.len()))
            .finish()
    }
}

impl McBufReadable for Section {
    fn read_from(buf: &mut impl Read) -> Result<Self, String> {
        let block_count = u16::read_from(buf)?;

        // this is commented out because the vanilla server is wrong
        // assert!(
        //     block_count <= 16 * 16 * 16,
        //     "A section has more blocks than what should be possible. This is a bug!"
        // );

        let states = PalettedContainer::read_with_type(buf, &PalettedContainerType::BlockStates)?;

        for i in 0..states.storage.size() {
            if !BlockState::is_valid_state(states.storage.get(i) as u32) {
                return Err(format!(
                    "Invalid block state {} (index {}) found in section.",
                    states.storage.get(i),
                    i
                ));
            }
        }

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

impl Section {
    fn get(&self, pos: ChunkSectionBlockPos) -> BlockState {
        // TODO: use the unsafe method and do the check earlier
        self.states
            .get(pos.x as usize, pos.y as usize, pos.z as usize)
            .try_into()
            .expect("Invalid block state.")
    }
}

impl Default for ChunkStorage {
    fn default() -> Self {
        Self::new(8, 384, -64)
    }
}
