use std::{
    fmt::{self, Display},
    str::FromStr,
};

use azalea_block::{BlockState, BlockTrait};
use azalea_buf::AzBuf;
use azalea_core::{math, position::ChunkBlockPos};
use azalea_registry::tags::blocks::LEAVES;
use tracing::warn;

use crate::{BitStorage, Section, chunk_storage::get_block_state_from_sections};

// (wg stands for worldgen)

#[derive(AzBuf, Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum HeightmapKind {
    WorldSurfaceWg,
    WorldSurface,
    OceanFloorWg,
    OceanFloor,
    MotionBlocking,
    MotionBlockingNoLeaves,
}

#[derive(Clone, Debug)]
pub struct Heightmap {
    pub data: BitStorage,
    pub min_y: i32,
    pub kind: HeightmapKind,
}

fn blocks_motion(block_state: BlockState) -> bool {
    // TODO
    !block_state.is_air()
}

fn motion_blocking(block_state: BlockState) -> bool {
    // TODO
    !block_state.is_air()
        || block_state
            .property::<azalea_block::properties::Waterlogged>()
            .unwrap_or_default()
}

impl HeightmapKind {
    pub fn is_opaque(self, block_state: BlockState) -> bool {
        let block = Box::<dyn BlockTrait>::from(block_state);
        let registry_block = block.as_registry_block();
        match self {
            HeightmapKind::WorldSurfaceWg => !block_state.is_air(),
            HeightmapKind::WorldSurface => !block_state.is_air(),
            HeightmapKind::OceanFloorWg => blocks_motion(block_state),
            HeightmapKind::OceanFloor => blocks_motion(block_state),
            HeightmapKind::MotionBlocking => motion_blocking(block_state),
            HeightmapKind::MotionBlockingNoLeaves => {
                motion_blocking(block_state) && !LEAVES.contains(&registry_block)
            }
        }
    }
}

impl Heightmap {
    pub fn new(kind: HeightmapKind, dimension_height: u32, min_y: i32, data: Box<[u64]>) -> Self {
        let bits = math::ceil_log2(dimension_height + 1);
        let mut bit_storage = BitStorage::new(bits as usize, 16 * 16, None)
            .expect("data is empty, so this can't fail");
        if bit_storage.data.len() != data.len() {
            warn!(
                "Ignoring heightmap data, size does not match; expected: {}, got: {}",
                bit_storage.data.len(),
                data.len()
            );
        } else {
            bit_storage.data.copy_from_slice(&data);
        }
        Self {
            kind,
            data: bit_storage,
            min_y,
        }
    }

    pub fn get_index(x: u8, z: u8) -> usize {
        (x as usize) + (z as usize) * 16
    }

    pub fn get_first_available_at_index(&self, index: usize) -> i32 {
        self.data.get(index) as i32 + self.min_y
    }

    pub fn get_first_available(&self, x: u8, z: u8) -> i32 {
        self.get_first_available_at_index(Self::get_index(x, z))
    }

    pub fn get_highest_taken(&self, x: u8, z: u8) -> i32 {
        self.get_first_available(x, z) - 1
    }

    pub fn set_height(&mut self, x: u8, z: u8, height: i32) {
        self.data
            .set(Self::get_index(x, z), (height - self.min_y) as u64);
    }

    /// Updates the heightmap with the given block state at the given position.
    pub fn update(
        &mut self,
        pos: &ChunkBlockPos,
        block_state: BlockState,
        sections: &[Section],
    ) -> bool {
        let first_available_y = self.get_first_available(pos.x, pos.z);
        if pos.y <= first_available_y - 2 {
            return false;
        }
        if self.kind.is_opaque(block_state) {
            // increase y
            if pos.y >= first_available_y {
                self.set_height(pos.x, pos.z, pos.y + 1);
                return true;
            }
        } else if first_available_y - 1 == pos.y {
            // decrease y
            for y in (self.min_y..pos.y).rev() {
                if self.kind.is_opaque(
                    get_block_state_from_sections(
                        sections,
                        &ChunkBlockPos::new(pos.x, y, pos.z),
                        self.min_y,
                    )
                    .unwrap_or_default(),
                ) {
                    self.set_height(pos.x, pos.z, y + 1);
                    return true;
                }
            }

            self.set_height(pos.x, pos.z, self.min_y);
            return true;
        }

        false
    }

    /// Get an iterator over the top available block positions in this
    /// heightmap.
    pub fn iter_first_available(&self) -> impl Iterator<Item = ChunkBlockPos> + '_ {
        self.data.iter().enumerate().map(move |(index, height)| {
            let x = (index % 16) as u8;
            let z = (index / 16) as u8;
            ChunkBlockPos::new(x, height as i32 + self.min_y, z)
        })
    }

    /// Get an iterator over the top block positions in this heightmap.
    pub fn iter_highest_taken(&self) -> impl Iterator<Item = ChunkBlockPos> + '_ {
        self.data.iter().enumerate().map(move |(index, height)| {
            let x = (index % 16) as u8;
            let z = (index / 16) as u8;
            ChunkBlockPos::new(x, height as i32 + self.min_y - 1, z)
        })
    }
}

impl FromStr for HeightmapKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "WORLD_SURFACE_WG" => Ok(HeightmapKind::WorldSurfaceWg),
            "WORLD_SURFACE" => Ok(HeightmapKind::WorldSurface),
            "OCEAN_FLOOR_WG" => Ok(HeightmapKind::OceanFloorWg),
            "OCEAN_FLOOR" => Ok(HeightmapKind::OceanFloor),
            "MOTION_BLOCKING" => Ok(HeightmapKind::MotionBlocking),
            "MOTION_BLOCKING_NO_LEAVES" => Ok(HeightmapKind::MotionBlockingNoLeaves),
            _ => Err(()),
        }
    }
}

impl Display for HeightmapKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HeightmapKind::WorldSurfaceWg => write!(f, "WORLD_SURFACE_WG"),
            HeightmapKind::WorldSurface => write!(f, "WORLD_SURFACE"),
            HeightmapKind::OceanFloorWg => write!(f, "OCEAN_FLOOR_WG"),
            HeightmapKind::OceanFloor => write!(f, "OCEAN_FLOOR"),
            HeightmapKind::MotionBlocking => write!(f, "MOTION_BLOCKING"),
            HeightmapKind::MotionBlockingNoLeaves => write!(f, "MOTION_BLOCKING_NO_LEAVES"),
        }
    }
}
