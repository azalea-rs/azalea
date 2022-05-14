use std::ops::Rem;

#[derive(Clone, Copy, Debug, Default)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl BlockPos {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        BlockPos { x, y, z }
    }
}

impl Rem<i32> for BlockPos {
    type Output = Self;

    fn rem(self, rhs: i32) -> Self {
        BlockPos {
            x: self.x % rhs,
            y: self.y % rhs,
            z: self.z % rhs,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ChunkPos {
    pub x: i32,
    pub z: i32,
}

impl ChunkPos {
    pub fn new(x: i32, z: i32) -> Self {
        ChunkPos { x, z }
    }
}

impl From<&BlockPos> for ChunkPos {
    fn from(pos: &BlockPos) -> Self {
        ChunkPos {
            x: pos.x.div_floor(16),
            z: pos.z.div_floor(16),
        }
    }
}

/// The coordinates of a chunk section in the world.
#[derive(Clone, Copy, Debug, Default)]
pub struct ChunkSectionPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl ChunkSectionPos {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        ChunkSectionPos { x, y, z }
    }
}

impl From<BlockPos> for ChunkSectionPos {
    fn from(pos: BlockPos) -> Self {
        ChunkSectionPos {
            x: pos.x.div_floor(16),
            y: pos.y.div_floor(16),
            z: pos.z.div_floor(16),
        }
    }
}

impl From<ChunkSectionPos> for ChunkPos {
    fn from(pos: ChunkSectionPos) -> Self {
        ChunkPos { x: pos.x, z: pos.z }
    }
}

/// The coordinates of a block inside a chunk.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ChunkBlockPos {
    pub x: u8,
    pub y: i32,
    pub z: u8,
}

impl ChunkBlockPos {
    pub fn new(x: u8, y: i32, z: u8) -> Self {
        ChunkBlockPos { x, y, z }
    }
}

impl From<&BlockPos> for ChunkBlockPos {
    fn from(pos: &BlockPos) -> Self {
        ChunkBlockPos {
            x: pos.x.rem_euclid(16).abs() as u8,
            y: pos.y,
            z: pos.z.rem_euclid(16).abs() as u8,
        }
    }
}

/// The coordinates of a block inside a chunk section.
#[derive(Clone, Copy, Debug, Default)]
pub struct ChunkSectionBlockPos {
    /// A number between 0 and 16.
    pub x: u8,
    /// A number between 0 and 16.
    pub y: u8,
    /// A number between 0 and 16.
    pub z: u8,
}

impl ChunkSectionBlockPos {
    pub fn new(x: u8, y: u8, z: u8) -> Self {
        ChunkSectionBlockPos { x, y, z }
    }
}

impl From<&BlockPos> for ChunkSectionBlockPos {
    fn from(pos: &BlockPos) -> Self {
        ChunkSectionBlockPos {
            x: pos.x.rem(16).abs() as u8,
            y: pos.y.rem(16).abs() as u8,
            z: pos.z.rem(16).abs() as u8,
        }
    }
}

impl From<&ChunkBlockPos> for ChunkSectionBlockPos {
    fn from(pos: &ChunkBlockPos) -> Self {
        ChunkSectionBlockPos {
            x: pos.x,
            y: pos.y.rem(16).abs() as u8,
            z: pos.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_block_pos_to_chunk_pos() {
        let block_pos = BlockPos::new(5, 78, -2);
        let chunk_pos = ChunkPos::from(&block_pos);
        assert_eq!(chunk_pos, ChunkPos::new(0, -1));
    }

    #[test]
    fn test_from_block_pos_to_chunk_block_pos() {
        let block_pos = BlockPos::new(5, 78, -2);
        let chunk_block_pos = ChunkBlockPos::from(&block_pos);
        assert_eq!(chunk_block_pos, ChunkBlockPos::new(5, 78, 14));
    }
}
