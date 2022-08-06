use crate::ResourceLocation;
use azalea_buf::{BufReadError, McBufReadable, McBufWritable};
use std::{
    io::{Read, Write},
    ops::Rem,
};

pub trait PositionXYZ<T> {
    fn add_x(&self, n: T) -> Self;
    fn add_y(&self, n: T) -> Self;
    fn add_z(&self, n: T) -> Self;
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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

impl PositionXYZ<i32> for BlockPos {
    fn add_x(&self, n: i32) -> Self {
        BlockPos {
            x: self.x + n,
            y: self.y,
            z: self.z,
        }
    }
    fn add_y(&self, n: i32) -> Self {
        BlockPos {
            x: self.x,
            y: self.y + n,
            z: self.z,
        }
    }
    fn add_z(&self, n: i32) -> Self {
        BlockPos {
            x: self.x,
            y: self.y,
            z: self.z + n,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ChunkPos {
    pub x: i32,
    pub z: i32,
}

impl ChunkPos {
    pub fn new(x: i32, z: i32) -> Self {
        ChunkPos { x, z }
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
/// The coordinates of a block inside a chunk.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
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

/// A block pos with an attached dimension
#[derive(Debug, Clone)]
pub struct GlobalPos {
    pub pos: BlockPos,
    // this is actually a ResourceKey in Minecraft, but i don't think it matters?
    pub dimension: ResourceLocation,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct EntityPos {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl PositionXYZ<f64> for EntityPos {
    fn add_x(&self, n: f64) -> Self {
        EntityPos {
            x: self.x + n,
            y: self.y,
            z: self.z,
        }
    }
    fn add_y(&self, n: f64) -> Self {
        EntityPos {
            x: self.x,
            y: self.y + n,
            z: self.z,
        }
    }
    fn add_z(&self, n: f64) -> Self {
        EntityPos {
            x: self.x,
            y: self.y,
            z: self.z + n,
        }
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

impl From<&BlockPos> for ChunkBlockPos {
    fn from(pos: &BlockPos) -> Self {
        ChunkBlockPos {
            x: pos.x.rem_euclid(16).unsigned_abs() as u8,
            y: pos.y,
            z: pos.z.rem_euclid(16).unsigned_abs() as u8,
        }
    }
}

impl From<&BlockPos> for ChunkSectionBlockPos {
    fn from(pos: &BlockPos) -> Self {
        ChunkSectionBlockPos {
            x: pos.x.rem(16).unsigned_abs() as u8,
            y: pos.y.rem(16).unsigned_abs() as u8,
            z: pos.z.rem(16).unsigned_abs() as u8,
        }
    }
}

impl From<&ChunkBlockPos> for ChunkSectionBlockPos {
    fn from(pos: &ChunkBlockPos) -> Self {
        ChunkSectionBlockPos {
            x: pos.x,
            y: pos.y.rem(16).unsigned_abs() as u8,
            z: pos.z,
        }
    }
}
impl From<&EntityPos> for BlockPos {
    fn from(pos: &EntityPos) -> Self {
        BlockPos {
            x: pos.x.floor() as i32,
            y: pos.y.floor() as i32,
            z: pos.z.floor() as i32,
        }
    }
}

impl From<&EntityPos> for ChunkPos {
    fn from(pos: &EntityPos) -> Self {
        ChunkPos::from(&BlockPos::from(pos))
    }
}

impl McBufReadable for BlockPos {
    fn read_from(buf: &mut impl Read) -> Result<Self, BufReadError> {
        let val = u64::read_from(buf)?;
        let x = (val >> 38) as i32;
        let y = (val & 0xFFF) as i32;
        let z = ((val >> 12) & 0x3FFFFFF) as i32;
        Ok(BlockPos { x, y, z })
    }
}

impl McBufReadable for GlobalPos {
    fn read_from(buf: &mut impl Read) -> Result<Self, BufReadError> {
        Ok(GlobalPos {
            dimension: ResourceLocation::read_from(buf)?,
            pos: BlockPos::read_from(buf)?,
        })
    }
}

impl McBufReadable for ChunkSectionPos {
    fn read_from(buf: &mut impl Read) -> Result<Self, BufReadError> {
        let long = i64::read_from(buf)?;
        Ok(ChunkSectionPos {
            x: (long >> 42) as i32,
            y: (long << 44 >> 44) as i32,
            z: (long << 22 >> 42) as i32,
        })
    }
}

impl McBufWritable for BlockPos {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let data = (((self.x & 0x3FFFFFF) as i64) << 38)
            | (((self.z & 0x3FFFFFF) as i64) << 12)
            | ((self.y & 0xFFF) as i64);
        data.write_into(buf)
    }
}

impl McBufWritable for GlobalPos {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        ResourceLocation::write_into(&self.dimension, buf)?;
        BlockPos::write_into(&self.pos, buf)?;

        Ok(())
    }
}

impl McBufWritable for ChunkSectionPos {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let long = (((self.x & 0x3FFFFF) as i64) << 42)
            | (self.y & 0xFFFFF) as i64
            | (((self.z & 0x3FFFFF) as i64) << 20);
        long.write_into(buf)?;
        Ok(())
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

    #[test]
    fn test_from_entity_pos_to_block_pos() {
        let entity_pos = EntityPos {
            x: 31.5,
            y: 80.0,
            z: -16.1,
        };
        let block_pos = BlockPos::from(&entity_pos);
        assert_eq!(block_pos, BlockPos::new(31, 80, -17));
    }

    #[test]
    fn test_from_entity_pos_to_chunk_pos() {
        let entity_pos = EntityPos {
            x: 31.5,
            y: 80.0,
            z: -16.1,
        };
        let chunk_pos = ChunkPos::from(&entity_pos);
        assert_eq!(chunk_pos, ChunkPos::new(1, -2));
    }
}
