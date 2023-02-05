use crate::ResourceLocation;
use azalea_buf::{BufReadError, McBuf, McBufReadable, McBufWritable};
use std::{
    io::{Cursor, Write},
    ops::{Add, AddAssign, Mul, Rem, Sub},
};

macro_rules! vec3_impl {
    ($name:ident, $type:ty) => {
        impl $name {
            pub fn new(x: $type, y: $type, z: $type) -> Self {
                Self { x, y, z }
            }

            pub fn length_sqr(&self) -> $type {
                self.x * self.x + self.y * self.y + self.z * self.z
            }

            /// Return a new instance of this position with the y coordinate
            /// decreased by the given number.
            pub fn down(&self, y: $type) -> Self {
                Self {
                    x: self.x,
                    y: self.y - y,
                    z: self.z,
                }
            }
            /// Return a new instance of this position with the y coordinate
            /// increased by the given number.
            pub fn up(&self, y: $type) -> Self {
                Self {
                    x: self.x,
                    y: self.y + y,
                    z: self.z,
                }
            }
        }

        impl Add for &$name {
            type Output = $name;

            fn add(self, rhs: Self) -> Self::Output {
                $name {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                    z: self.z + rhs.z,
                }
            }
        }

        impl Add for $name {
            type Output = $name;

            fn add(self, rhs: Self) -> Self::Output {
                (&self).add(&rhs)
            }
        }

        impl AddAssign for $name {
            fn add_assign(&mut self, rhs: Self) {
                self.x += rhs.x;
                self.y += rhs.y;
                self.z += rhs.z;
            }
        }
        impl Rem<$type> for $name {
            type Output = Self;

            fn rem(self, rhs: $type) -> Self::Output {
                Self {
                    x: self.x % rhs,
                    y: self.y % rhs,
                    z: self.z % rhs,
                }
            }
        }

        impl Sub for &$name {
            type Output = $name;

            /// Find the difference between two positions.
            fn sub(self, other: Self) -> Self::Output {
                Self::Output {
                    x: self.x - other.x,
                    y: self.y - other.y,
                    z: self.z - other.z,
                }
            }
        }
        impl Sub for $name {
            type Output = Self;

            fn sub(self, other: Self) -> Self::Output {
                (&self).sub(&other)
            }
        }

        impl Mul<$type> for $name {
            type Output = Self;

            fn mul(self, multiplier: $type) -> Self::Output {
                Self {
                    x: self.x * multiplier,
                    y: self.y * multiplier,
                    z: self.z * multiplier,
                }
            }
        }
    };
}

/// Used to represent an exact position in the world where an entity could be.
/// For blocks, [`BlockPos`] is used instead.
#[derive(Clone, Copy, Debug, Default, PartialEq, McBuf)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
vec3_impl!(Vec3, f64);

/// The coordinates of a block in the world. For entities (if the coordinate
/// with decimals), use [`Vec3`] instead.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
vec3_impl!(BlockPos, i32);

impl BlockPos {
    /// Get the absolute center of a block position by adding 0.5 to each
    /// coordinate.
    pub fn center(&self) -> Vec3 {
        Vec3 {
            x: self.x as f64 + 0.5,
            y: self.y as f64 + 0.5,
            z: self.z as f64 + 0.5,
        }
    }
}

/// Chunk coordinates are used to represent where a chunk is in the world. You
/// can convert the x and z to block coordinates by multiplying them by 16.
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
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ChunkSectionPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
vec3_impl!(ChunkSectionPos, i32);

impl ChunkSectionPos {
    pub fn block_to_section_coord(block: i32) -> i32 {
        block >> 4
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

/// The coordinates of a block inside a chunk section. Each coordinate must be
/// in the range [0, 15].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ChunkSectionBlockPos {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}
vec3_impl!(ChunkSectionBlockPos, u8);

impl Add<ChunkSectionBlockPos> for ChunkSectionPos {
    type Output = BlockPos;

    fn add(self, rhs: ChunkSectionBlockPos) -> Self::Output {
        BlockPos::new(
            self.x * 16 + rhs.x as i32,
            self.y * 16 + rhs.y as i32,
            self.z * 16 + rhs.z as i32,
        )
    }
}

/// A block pos with an attached world
#[derive(Debug, Clone)]
pub struct GlobalPos {
    pub pos: BlockPos,
    // this is actually a ResourceKey in Minecraft, but i don't think it matters?
    pub world: ResourceLocation,
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
            x: pos.x.rem_euclid(16) as u8,
            y: pos.y,
            z: pos.z.rem_euclid(16) as u8,
        }
    }
}

impl From<&BlockPos> for ChunkSectionBlockPos {
    fn from(pos: &BlockPos) -> Self {
        ChunkSectionBlockPos {
            x: pos.x.rem_euclid(16) as u8,
            y: pos.y.rem_euclid(16) as u8,
            z: pos.z.rem_euclid(16) as u8,
        }
    }
}

impl From<&ChunkBlockPos> for ChunkSectionBlockPos {
    fn from(pos: &ChunkBlockPos) -> Self {
        ChunkSectionBlockPos {
            x: pos.x,
            y: pos.y.rem_euclid(16) as u8,
            z: pos.z,
        }
    }
}
impl From<&Vec3> for BlockPos {
    fn from(pos: &Vec3) -> Self {
        BlockPos {
            x: pos.x.floor() as i32,
            y: pos.y.floor() as i32,
            z: pos.z.floor() as i32,
        }
    }
}
impl From<Vec3> for BlockPos {
    fn from(pos: Vec3) -> Self {
        BlockPos::from(&pos)
    }
}

impl From<&Vec3> for ChunkPos {
    fn from(pos: &Vec3) -> Self {
        ChunkPos::from(&BlockPos::from(pos))
    }
}
impl From<Vec3> for ChunkPos {
    fn from(pos: Vec3) -> Self {
        ChunkPos::from(&pos)
    }
}

const PACKED_X_LENGTH: u64 = 1 + 25; // minecraft does something a bit more complicated to get this 25
const PACKED_Z_LENGTH: u64 = PACKED_X_LENGTH;
const PACKED_Y_LENGTH: u64 = 64 - PACKED_X_LENGTH - PACKED_Z_LENGTH;
const PACKED_X_MASK: u64 = (1 << PACKED_X_LENGTH) - 1;
const PACKED_Y_MASK: u64 = (1 << PACKED_Y_LENGTH) - 1;
const PACKED_Z_MASK: u64 = (1 << PACKED_Z_LENGTH) - 1;
const Z_OFFSET: u64 = PACKED_Y_LENGTH;
const X_OFFSET: u64 = PACKED_Y_LENGTH + PACKED_Z_LENGTH;

impl McBufReadable for BlockPos {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let val = i64::read_from(buf)?;
        let x = (val << (64 - X_OFFSET - PACKED_X_LENGTH) >> (64 - PACKED_X_LENGTH)) as i32;
        let y = (val << (64 - PACKED_Y_LENGTH) >> (64 - PACKED_Y_LENGTH)) as i32;
        let z = (val << (64 - Z_OFFSET - PACKED_Z_LENGTH) >> (64 - PACKED_Z_LENGTH)) as i32;
        Ok(BlockPos { x, y, z })
    }
}

impl McBufReadable for GlobalPos {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        Ok(GlobalPos {
            world: ResourceLocation::read_from(buf)?,
            pos: BlockPos::read_from(buf)?,
        })
    }
}

impl McBufReadable for ChunkSectionPos {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
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
        let mut val: u64 = 0;
        val |= ((self.x as u64) & PACKED_X_MASK) << X_OFFSET;
        val |= (self.y as u64) & PACKED_Y_MASK;
        val |= ((self.z as u64) & PACKED_Z_MASK) << Z_OFFSET;
        val.write_into(buf)
    }
}

impl McBufWritable for GlobalPos {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        ResourceLocation::write_into(&self.world, buf)?;
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
        let entity_pos = Vec3 {
            x: 31.5,
            y: 80.0,
            z: -16.1,
        };
        let block_pos = BlockPos::from(&entity_pos);
        assert_eq!(block_pos, BlockPos::new(31, 80, -17));
    }

    #[test]
    fn test_from_entity_pos_to_chunk_pos() {
        let entity_pos = Vec3 {
            x: 31.5,
            y: 80.0,
            z: -16.1,
        };
        let chunk_pos = ChunkPos::from(&entity_pos);
        assert_eq!(chunk_pos, ChunkPos::new(1, -2));
    }

    #[test]
    fn test_read_blockpos_from() {
        let mut buf = Vec::new();
        13743895338965u64.write_into(&mut buf).unwrap();
        let mut buf = Cursor::new(&buf[..]);
        let block_pos = BlockPos::read_from(&mut buf).unwrap();
        assert_eq!(block_pos, BlockPos::new(49, -43, -3));
    }

    #[test]
    fn test_into_chunk_section_block_pos() {
        let block_pos = BlockPos::new(0, -60, 0);
        assert_eq!(
            ChunkSectionBlockPos::from(&block_pos),
            ChunkSectionBlockPos::new(0, 4, 0)
        );
    }
}
