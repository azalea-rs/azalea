//! Representations of positions of various things in Minecraft.
//!
//! The most common ones are [`Vec3`] and [`BlockPos`], which are usually used
//! for entity positions and block positions, respectively.

use std::{
    fmt,
    hash::{Hash, Hasher},
    io,
    io::{Cursor, Write},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, Sub},
    str::FromStr,
};

use azalea_buf::{AzBuf, AzaleaRead, AzaleaWrite, BufReadError};
use azalea_registry::identifier::Identifier;
use serde::{Serialize, Serializer};
use simdnbt::borrow::NbtTag;

use crate::{codec_utils::IntArray, direction::Direction, math};

macro_rules! vec3_impl {
    ($name:ident, $type:ty) => {
        impl $name {
            /// The position where x, y, and z are all 0.
            pub const ZERO: Self = Self {
                x: 0 as $type,
                y: 0 as $type,
                z: 0 as $type,
            };

            #[inline]
            pub const fn new(x: $type, y: $type, z: $type) -> Self {
                Self { x, y, z }
            }

            /// Get the distance of this vector to the origin by doing `x^2 + y^2 +
            /// z^2`.
            #[inline]
            pub fn length_squared(&self) -> $type {
                self.x * self.x + self.y * self.y + self.z * self.z
            }

            /// Get the squared distance from this position to another position.
            /// Equivalent to `(self - other).length_squared()`.
            #[inline]
            pub fn distance_squared_to(self, other: Self) -> $type {
                (self - other).length_squared()
            }

            #[inline]
            pub fn horizontal_distance_squared(&self) -> $type {
                self.x * self.x + self.z * self.z
            }

            #[inline]
            pub fn horizontal_distance_squared_to(self, other: Self) -> $type {
                (self - other).horizontal_distance_squared()
            }

            /// Return a new instance of this position with the y coordinate
            /// decreased by the given number.
            #[inline]
            pub fn down(&self, y: $type) -> Self {
                Self {
                    x: self.x,
                    y: self.y - y,
                    z: self.z,
                }
            }
            /// Return a new instance of this position with the y coordinate
            /// increased by the given number.
            #[inline]
            pub fn up(&self, y: $type) -> Self {
                Self {
                    x: self.x,
                    y: self.y + y,
                    z: self.z,
                }
            }

            /// Return a new instance of this position with the z coordinate subtracted
            /// by the given number.
            #[inline]
            pub fn north(&self, z: $type) -> Self {
                Self {
                    x: self.x,
                    y: self.y,
                    z: self.z - z,
                }
            }
            /// Return a new instance of this position with the x coordinate increased
            /// by the given number.
            #[inline]
            pub fn east(&self, x: $type) -> Self {
                Self {
                    x: self.x + x,
                    y: self.y,
                    z: self.z,
                }
            }
            /// Return a new instance of this position with the z coordinate increased
            /// by the given number.
            #[inline]
            pub fn south(&self, z: $type) -> Self {
                Self {
                    x: self.x,
                    y: self.y,
                    z: self.z + z,
                }
            }
            /// Return a new instance of this position with the x coordinate subtracted
            /// by the given number.
            #[inline]
            pub fn west(&self, x: $type) -> Self {
                Self {
                    x: self.x - x,
                    y: self.y,
                    z: self.z,
                }
            }

            #[inline]
            pub fn dot(&self, other: Self) -> $type {
                self.x * other.x + self.y * other.y + self.z * other.z
            }

            /// Make a new position with the lower coordinates for each axis.
            pub fn min(&self, other: Self) -> Self {
                Self {
                    x: self.x.min(other.x),
                    y: self.x.min(other.y),
                    z: self.x.min(other.z),
                }
            }
            /// Make a new position with the higher coordinates for each axis.
            pub fn max(&self, other: Self) -> Self {
                Self {
                    x: self.x.max(other.x),
                    y: self.x.max(other.y),
                    z: self.x.max(other.z),
                }
            }

            /// Replace the Y with 0.
            #[inline]
            pub fn xz(&self) -> Self {
                Self {
                    x: self.x,
                    y: <$type>::default(),
                    z: self.z,
                }
            }

            pub fn with_x(&self, x: $type) -> Self {
                Self { x, ..*self }
            }
            pub fn with_y(&self, y: $type) -> Self {
                Self { y, ..*self }
            }
            pub fn with_z(&self, z: $type) -> Self {
                Self { z, ..*self }
            }
        }

        impl Add for &$name {
            type Output = $name;

            #[inline]
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

            #[inline]
            fn add(self, rhs: Self) -> Self::Output {
                (&self).add(&rhs)
            }
        }
        impl Add<$type> for $name {
            type Output = Self;

            #[inline]
            fn add(self, rhs: $type) -> Self::Output {
                Self {
                    x: self.x + rhs,
                    y: self.y + rhs,
                    z: self.z + rhs,
                }
            }
        }

        impl AddAssign for $name {
            #[inline]
            fn add_assign(&mut self, rhs: Self) {
                self.x += rhs.x;
                self.y += rhs.y;
                self.z += rhs.z;
            }
        }
        impl Rem<$type> for $name {
            type Output = Self;

            #[inline]
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
            #[inline]
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

            #[inline]
            fn sub(self, other: Self) -> Self::Output {
                (&self).sub(&other)
            }
        }

        impl Mul<$type> for $name {
            type Output = Self;

            #[inline]
            fn mul(self, multiplier: $type) -> Self::Output {
                Self {
                    x: self.x * multiplier,
                    y: self.y * multiplier,
                    z: self.z * multiplier,
                }
            }
        }
        impl MulAssign<$type> for $name {
            #[inline]
            fn mul_assign(&mut self, multiplier: $type) {
                self.x *= multiplier;
                self.y *= multiplier;
                self.z *= multiplier;
            }
        }

        impl Div<$type> for $name {
            type Output = Self;

            #[inline]
            fn div(self, divisor: $type) -> Self::Output {
                Self {
                    x: self.x / divisor,
                    y: self.y / divisor,
                    z: self.z / divisor,
                }
            }
        }
        impl DivAssign<$type> for $name {
            #[inline]
            fn div_assign(&mut self, divisor: $type) {
                self.x /= divisor;
                self.y /= divisor;
                self.z /= divisor;
            }
        }

        impl From<($type, $type, $type)> for $name {
            #[inline]
            fn from(pos: ($type, $type, $type)) -> Self {
                Self::new(pos.0, pos.1, pos.2)
            }
        }
        impl From<&($type, $type, $type)> for $name {
            #[inline]
            fn from(pos: &($type, $type, $type)) -> Self {
                Self::new(pos.0, pos.1, pos.2)
            }
        }
        impl From<$name> for ($type, $type, $type) {
            #[inline]
            fn from(pos: $name) -> Self {
                (pos.x, pos.y, pos.z)
            }
        }
    };
}

/// Used to represent an exact position in the world where an entity could be.
///
/// For blocks, [`BlockPos`] is used instead.
#[derive(AzBuf, Clone, Copy, Debug, Default, serde::Deserialize, PartialEq, serde::Serialize)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
vec3_impl!(Vec3, f64);
impl simdnbt::FromNbtTag for Vec3 {
    fn from_nbt_tag(tag: NbtTag) -> Option<Self> {
        let pos = tag.list()?;
        if let Some(pos) = pos.doubles() {
            let [x, y, z] = <[f64; 3]>::try_from(pos).ok()?;
            Some(Self { x, y, z })
        } else if let Some(pos) = pos.floats() {
            // used on hypixel
            let [x, y, z] = <[f32; 3]>::try_from(pos).ok()?.map(|f| f as f64);
            Some(Self { x, y, z })
        } else {
            None
        }
    }
}

impl Vec3 {
    /// Get the distance of this vector to the origin by doing
    /// `sqrt(x^2 + y^2 + z^2)`.
    pub fn length(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    /// Get the distance from this position to another position.
    /// Equivalent to `(self - other).length()`.
    pub fn distance_to(self, other: Self) -> f64 {
        (self - other).length()
    }

    pub fn x_rot(self, radians: f32) -> Vec3 {
        let x_delta = math::cos(radians);
        let y_delta = math::sin(radians);
        let x = self.x;
        let y = self.y * (x_delta as f64) + self.z * (y_delta as f64);
        let z = self.z * (x_delta as f64) - self.y * (y_delta as f64);
        Vec3 { x, y, z }
    }
    pub fn y_rot(self, radians: f32) -> Vec3 {
        let x_delta = math::cos(radians);
        let y_delta = math::sin(radians);
        let x = self.x * (x_delta as f64) + self.z * (y_delta as f64);
        let y = self.y;
        let z = self.z * (x_delta as f64) - self.x * (y_delta as f64);
        Vec3 { x, y, z }
    }

    pub fn to_block_pos_floor(&self) -> BlockPos {
        BlockPos {
            x: self.x.floor() as i32,
            y: self.y.floor() as i32,
            z: self.z.floor() as i32,
        }
    }
    pub fn to_block_pos_ceil(&self) -> BlockPos {
        BlockPos {
            x: self.x.ceil() as i32,
            y: self.y.ceil() as i32,
            z: self.z.ceil() as i32,
        }
    }

    /// Whether the distance between this point and `other` is less than
    /// `range`.
    pub fn closer_than(&self, other: Vec3, range: f64) -> bool {
        self.distance_squared_to(other) < range.powi(2)
    }
}

/// A lower precision [`Vec3`], used for some fields in entity metadata.
#[derive(AzBuf, Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3f32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl From<Vec3f32> for Vec3 {
    fn from(v: Vec3f32) -> Self {
        Vec3 {
            x: v.x as f64,
            y: v.y as f64,
            z: v.z as f64,
        }
    }
}
impl From<Vec3> for Vec3f32 {
    fn from(v: Vec3) -> Self {
        Vec3f32 {
            x: v.x as f32,
            y: v.y as f32,
            z: v.z as f32,
        }
    }
}

/// The coordinates of a block in the world.
///
/// For entities (if the coordinates are floating-point), use [`Vec3`] instead.
/// To convert a `BlockPos` to a `Vec3`, you'll usually want [`Self::center`].
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
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

    /// Get the center of the bottom of a block position by adding 0.5 to the x
    /// and z coordinates.
    pub fn center_bottom(&self) -> Vec3 {
        Vec3 {
            x: self.x as f64 + 0.5,
            y: self.y as f64,
            z: self.z as f64 + 0.5,
        }
    }

    /// Convert the block position into a Vec3 without centering it.
    pub fn to_vec3_floored(&self) -> Vec3 {
        Vec3 {
            x: self.x as f64,
            y: self.y as f64,
            z: self.z as f64,
        }
    }

    /// Get the distance of this vector from the origin by doing `x + y + z`.
    pub fn length_manhattan(&self) -> u32 {
        (self.x.abs() + self.y.abs() + self.z.abs()) as u32
    }

    /// Add or subtract `1` to one of this position's coordinates, depending on
    /// the direction.
    ///
    /// ```
    /// # use azalea_core::{position::BlockPos, direction::Direction};
    /// let pos = BlockPos::new(10, 10, 10);
    /// assert_eq!(
    ///     pos.offset_with_direction(Direction::North),
    ///     BlockPos::new(10, 10, 9)
    /// );
    /// ```
    pub fn offset_with_direction(self, direction: Direction) -> Self {
        self + direction.normal()
    }

    /// Get the distance (as an f64) of this BlockPos to the origin by
    /// doing `sqrt(x^2 + y^2 + z^2)`.
    pub fn length(&self) -> f64 {
        f64::sqrt((self.x * self.x + self.y * self.y + self.z * self.z) as f64)
    }

    /// Get the distance (as an f64) from this position to another position.
    /// Equivalent to `(self - other).length()`.
    ///
    /// Note that if you're using this in a hot path, it may be more performant
    /// to use [`BlockPos::distance_squared_to`] instead (by squaring the other
    /// side in the comparison).
    pub fn distance_to(self, other: Self) -> f64 {
        (self - other).length()
    }
}
impl serde::Serialize for BlockPos {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // makes sure it gets serialized correctly for the checksum
        IntArray([self.x, self.y, self.z]).serialize(serializer)
    }
}
impl<'de> serde::Deserialize<'de> for BlockPos {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let [x, y, z] = <[i32; 3]>::deserialize(deserializer)?;
        Ok(BlockPos { x, y, z })
    }
}

/// An arbitrary position that's represented as 32-bit integers.
///
/// This is similar to [`BlockPos`], but isn't limited to representing block
/// positions and can represent a larger range of numbers.
#[derive(AzBuf, Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Vec3i {
    #[var]
    pub x: i32,
    #[var]
    pub y: i32,
    #[var]
    pub z: i32,
}
vec3_impl!(Vec3i, i32);
impl simdnbt::FromNbtTag for Vec3i {
    fn from_nbt_tag(tag: NbtTag) -> Option<Self> {
        let pos = tag.list()?.ints()?;
        let [x, y, z] = <[i32; 3]>::try_from(pos).ok()?;
        Some(Self { x, y, z })
    }
}

/// Chunk coordinates are used to represent where a chunk is in the world.
///
/// You can convert the x and z to block coordinates by multiplying them by 16.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ChunkPos {
    pub x: i32,
    pub z: i32,
}
impl ChunkPos {
    pub fn new(x: i32, z: i32) -> Self {
        ChunkPos { x, z }
    }
}
impl Add<ChunkPos> for ChunkPos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            z: self.z + rhs.z,
        }
    }
}
impl Add<ChunkBlockPos> for ChunkPos {
    type Output = BlockPos;

    fn add(self, rhs: ChunkBlockPos) -> Self::Output {
        BlockPos {
            x: self.x * 16 + rhs.x as i32,
            y: rhs.y,
            z: self.z * 16 + rhs.z as i32,
        }
    }
}

// reading ChunkPos is done in reverse, so z first and then x
// ........
// mojang why
impl From<ChunkPos> for u64 {
    #[inline]
    fn from(pos: ChunkPos) -> Self {
        (pos.x as u64) | ((pos.z as u64) << 32)
    }
}
impl From<u64> for ChunkPos {
    #[inline]
    fn from(pos: u64) -> Self {
        ChunkPos {
            x: (pos) as i32,
            z: (pos >> 32) as i32,
        }
    }
}
impl AzaleaRead for ChunkPos {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let long = u64::azalea_read(buf)?;
        Ok(ChunkPos::from(long))
    }
}
impl AzaleaWrite for ChunkPos {
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        u64::from(*self).azalea_write(buf)?;
        Ok(())
    }
}

impl Hash for ChunkPos {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        // optimized hash that only calls hash once
        u64::from(*self).hash(state);
    }
}
/// `nohash_hasher` lets us have IntMap<ChunkPos, _> which is significantly
/// faster than a normal HashMap
impl nohash_hasher::IsEnabled for ChunkPos {}

/// The coordinates of a chunk section in the world.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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

impl Hash for ChunkBlockPos {
    // optimized hash that only calls hash once
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        u64::from(*self).hash(state);
    }
}
impl From<ChunkBlockPos> for u64 {
    #[inline]
    fn from(pos: ChunkBlockPos) -> Self {
        // convert to u64
        let mut val: u64 = 0;
        // first 32 bits are y
        val |= pos.y as u64;
        // next 8 bits are z
        val |= (pos.z as u64) << 32;
        // last 8 bits are x
        val |= (pos.x as u64) << 40;
        val
    }
}
impl nohash_hasher::IsEnabled for ChunkBlockPos {}

/// The coordinates of a block inside of a chunk section.
///
/// Each coordinate should be in the range 0..=15.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ChunkSectionBlockPos {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}
vec3_impl!(ChunkSectionBlockPos, u8);

/// The coordinates of a biome inside of a chunk section.
///
/// Each coordinate should be in the range 0..=3.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ChunkSectionBiomePos {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}
impl From<&ChunkBiomePos> for ChunkSectionBiomePos {
    #[inline]
    fn from(pos: &ChunkBiomePos) -> Self {
        ChunkSectionBiomePos {
            x: pos.x,
            y: (pos.y & 0b11) as u8,
            z: pos.z,
        }
    }
}
impl From<ChunkBiomePos> for ChunkSectionBiomePos {
    #[inline]
    fn from(pos: ChunkBiomePos) -> Self {
        Self::from(&pos)
    }
}
vec3_impl!(ChunkSectionBiomePos, u8);

/// The coordinates of a biome inside a chunk.
///
/// Biomes are 4x4 blocks.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ChunkBiomePos {
    pub x: u8,
    pub y: i32,
    pub z: u8,
}
impl From<&BlockPos> for ChunkBiomePos {
    #[inline]
    fn from(pos: &BlockPos) -> Self {
        ChunkBiomePos::from(&ChunkBlockPos::from(pos))
    }
}
impl From<BlockPos> for ChunkBiomePos {
    #[inline]
    fn from(pos: BlockPos) -> Self {
        ChunkBiomePos::from(&ChunkBlockPos::from(pos))
    }
}
impl From<&ChunkBlockPos> for ChunkBiomePos {
    #[inline]
    fn from(pos: &ChunkBlockPos) -> Self {
        ChunkBiomePos {
            x: pos.x >> 2,
            y: pos.y >> 2,
            z: pos.z >> 2,
        }
    }
}

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
impl Hash for ChunkSectionBlockPos {
    // optimized hash that only calls hash once
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        u16::from(*self).hash(state);
    }
}

impl From<ChunkSectionBlockPos> for u16 {
    #[inline]
    fn from(pos: ChunkSectionBlockPos) -> Self {
        // (pos.z as u16) | ((pos.y as u16) << 4) | ((pos.x as u16) << 8)
        ((((pos.y as u16) << 4) | pos.z as u16) << 4) | pos.x as u16
    }
}
impl nohash_hasher::IsEnabled for ChunkSectionBlockPos {}

/// A block pos with an attached world
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GlobalPos {
    // this is actually a ResourceKey in Minecraft, but i don't think it matters?
    pub dimension: Identifier,
    pub pos: BlockPos,
}

impl From<&BlockPos> for ChunkPos {
    #[inline]
    fn from(pos: &BlockPos) -> Self {
        ChunkPos {
            x: pos.x >> 4,
            z: pos.z >> 4,
        }
    }
}
impl From<BlockPos> for ChunkPos {
    #[inline]
    fn from(pos: BlockPos) -> Self {
        ChunkPos {
            x: pos.x >> 4,
            z: pos.z >> 4,
        }
    }
}

impl From<BlockPos> for ChunkSectionPos {
    #[inline]
    fn from(pos: BlockPos) -> Self {
        ChunkSectionPos {
            x: pos.x >> 4,
            y: pos.y >> 4,
            z: pos.z >> 4,
        }
    }
}
impl From<&BlockPos> for ChunkSectionPos {
    #[inline]
    fn from(pos: &BlockPos) -> Self {
        ChunkSectionPos {
            x: pos.x >> 4,
            y: pos.y >> 4,
            z: pos.z >> 4,
        }
    }
}

impl From<ChunkSectionPos> for ChunkPos {
    fn from(pos: ChunkSectionPos) -> Self {
        ChunkPos { x: pos.x, z: pos.z }
    }
}
impl From<&Vec3> for ChunkSectionPos {
    fn from(pos: &Vec3) -> Self {
        ChunkSectionPos::from(&BlockPos::from(pos))
    }
}
impl From<Vec3> for ChunkSectionPos {
    fn from(pos: Vec3) -> Self {
        ChunkSectionPos::from(&pos)
    }
}

impl From<&BlockPos> for ChunkBlockPos {
    #[inline]
    fn from(pos: &BlockPos) -> Self {
        ChunkBlockPos {
            x: (pos.x & 0xF) as u8,
            y: pos.y,
            z: (pos.z & 0xF) as u8,
        }
    }
}
impl From<BlockPos> for ChunkBlockPos {
    #[inline]
    fn from(pos: BlockPos) -> Self {
        ChunkBlockPos {
            x: (pos.x & 0xF) as u8,
            y: pos.y,
            z: (pos.z & 0xF) as u8,
        }
    }
}

impl From<BlockPos> for ChunkSectionBlockPos {
    #[inline]
    fn from(pos: BlockPos) -> Self {
        ChunkSectionBlockPos {
            x: (pos.x & 0xF) as u8,
            y: (pos.y & 0xF) as u8,
            z: (pos.z & 0xF) as u8,
        }
    }
}

impl From<&ChunkBlockPos> for ChunkSectionBlockPos {
    #[inline]
    fn from(pos: &ChunkBlockPos) -> Self {
        ChunkSectionBlockPos {
            x: pos.x,
            y: (pos.y & 0xF) as u8,
            z: pos.z,
        }
    }
}
impl From<&Vec3> for BlockPos {
    #[inline]
    fn from(pos: &Vec3) -> Self {
        BlockPos {
            x: pos.x.floor() as i32,
            y: pos.y.floor() as i32,
            z: pos.z.floor() as i32,
        }
    }
}
impl From<Vec3> for BlockPos {
    #[inline]
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

impl From<&Vec3> for ChunkBlockPos {
    fn from(pos: &Vec3) -> Self {
        ChunkBlockPos::from(&BlockPos::from(pos))
    }
}
impl From<Vec3> for ChunkBlockPos {
    fn from(pos: Vec3) -> Self {
        ChunkBlockPos::from(&pos)
    }
}

impl fmt::Display for BlockPos {
    /// Display a block position as `x y z`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}
impl fmt::Display for Vec3 {
    /// Display a position as `x y z`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

/// A 2D vector.
#[derive(AzBuf, Clone, Copy, Debug, Default, simdnbt::Deserialize, PartialEq, Serialize)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
impl Vec2 {
    const ZERO: Vec2 = Vec2 { x: 0.0, y: 0.0 };

    #[inline]
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 { x, y }
    }
    #[inline]
    pub fn scale(&self, amount: f32) -> Self {
        Vec2 {
            x: self.x * amount,
            y: self.y * amount,
        }
    }
    #[inline]
    pub fn dot(&self, other: Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }
    #[inline]
    pub fn normalized(&self) -> Self {
        let length = (self.x * self.x + self.y * self.y).sqrt();
        if length < 1e-4 {
            return Vec2::ZERO;
        }
        Vec2 {
            x: self.x / length,
            y: self.y / length,
        }
    }
    #[inline]
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }
    #[inline]
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
}
impl Mul<f32> for Vec2 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        self.scale(rhs)
    }
}
impl MulAssign<f32> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        *self = self.scale(rhs);
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

impl AzaleaRead for BlockPos {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let val = i64::azalea_read(buf)?;
        let x = (val << (64 - X_OFFSET - PACKED_X_LENGTH) >> (64 - PACKED_X_LENGTH)) as i32;
        let y = (val << (64 - PACKED_Y_LENGTH) >> (64 - PACKED_Y_LENGTH)) as i32;
        let z = (val << (64 - Z_OFFSET - PACKED_Z_LENGTH) >> (64 - PACKED_Z_LENGTH)) as i32;
        Ok(BlockPos { x, y, z })
    }
}

impl AzaleaRead for GlobalPos {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        Ok(GlobalPos {
            dimension: Identifier::azalea_read(buf)?,
            pos: BlockPos::azalea_read(buf)?,
        })
    }
}

impl AzaleaRead for ChunkSectionPos {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let long = i64::azalea_read(buf)?;
        Ok(ChunkSectionPos {
            x: (long >> 42) as i32,
            y: (long << 44 >> 44) as i32,
            z: (long << 22 >> 42) as i32,
        })
    }
}

impl AzaleaWrite for BlockPos {
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        let mut val: u64 = 0;
        val |= ((self.x as u64) & PACKED_X_MASK) << X_OFFSET;
        val |= (self.y as u64) & PACKED_Y_MASK;
        val |= ((self.z as u64) & PACKED_Z_MASK) << Z_OFFSET;
        val.azalea_write(buf)
    }
}

impl AzaleaWrite for GlobalPos {
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        Identifier::azalea_write(&self.dimension, buf)?;
        BlockPos::azalea_write(&self.pos, buf)?;

        Ok(())
    }
}

impl AzaleaWrite for ChunkSectionPos {
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        let long = (((self.x & 0x3FFFFF) as i64) << 42)
            | (self.y & 0xFFFFF) as i64
            | (((self.z & 0x3FFFFF) as i64) << 20);
        long.azalea_write(buf)?;
        Ok(())
    }
}

fn parse_three_values<T>(s: &str) -> Result<[T; 3], &'static str>
where
    T: FromStr,
    <T as FromStr>::Err: fmt::Debug,
{
    let parts = s.split_whitespace().collect::<Vec<_>>();
    if parts.len() != 3 {
        return Err("Expected three values");
    }

    let x = parts[0].parse().map_err(|_| "Invalid X value")?;
    let y = parts[1].parse().map_err(|_| "Invalid Y value")?;
    let z = parts[2].parse().map_err(|_| "Invalid Z value")?;

    Ok([x, y, z])
}

/// Parses a string in the format "X Y Z" into a BlockPos.
///
/// The input string should contain three integer values separated by spaces,
/// representing the x, y, and z components of the vector respectively.
/// This can be used to parse user input or from `BlockPos::to_string`.
impl FromStr for BlockPos {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [x, y, z] = parse_three_values::<i32>(s)?;
        Ok(BlockPos { x, y, z })
    }
}

/// Parses a string in the format "X Y Z" into a Vec3.
///
/// The input string should contain three floating-point values separated by
/// spaces, representing the x, y, and z components of the vector respectively.
/// This can be used to parse user input or from `Vec3::to_string`.
impl FromStr for Vec3 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [x, y, z] = parse_three_values::<f64>(s)?;
        Ok(Vec3 { x, y, z })
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
        13743895338965u64.azalea_write(&mut buf).unwrap();
        let mut buf = Cursor::new(&buf[..]);
        let block_pos = BlockPos::azalea_read(&mut buf).unwrap();
        assert_eq!(block_pos, BlockPos::new(49, -43, -3));
    }

    #[test]
    fn test_into_chunk_section_block_pos() {
        let block_pos = BlockPos::new(0, -60, 0);
        assert_eq!(
            ChunkSectionBlockPos::from(block_pos),
            ChunkSectionBlockPos::new(0, 4, 0)
        );
    }

    #[test]
    fn test_read_chunk_pos_from() {
        let mut buf = Vec::new();
        ChunkPos::new(2, -1).azalea_write(&mut buf).unwrap();
        let mut buf = Cursor::new(&buf[..]);
        let chunk_pos = ChunkPos::from(u64::azalea_read(&mut buf).unwrap());
        assert_eq!(chunk_pos, ChunkPos::new(2, -1));
    }
}
