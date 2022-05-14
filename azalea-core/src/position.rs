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

#[derive(Clone, Copy, Debug, Default)]
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
            x: pos.x / 16,
            z: pos.z / 16,
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
            x: pos.x / 16,
            y: pos.y / 16,
            z: pos.z / 16,
        }
    }
}

impl From<ChunkSectionPos> for ChunkPos {
    fn from(pos: ChunkSectionPos) -> Self {
        ChunkPos { x: pos.x, z: pos.z }
    }
}

/// The coordinates of a block inside a chunk section.
#[derive(Clone, Copy, Debug, Default)]
pub struct ChunkSectionBlockPos {
    pub x: u8,
    pub y: u8,
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
