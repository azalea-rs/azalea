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
