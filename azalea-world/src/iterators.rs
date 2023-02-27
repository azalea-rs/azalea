//! Iterators, based on [prismarine-world's iterators](https://github.com/PrismarineJS/prismarine-world/blob/master/src/iterators.js).

use azalea_core::{BlockPos, ChunkPos};

/// An octahedron iterator, useful for iterating over blocks in a world.
///
/// ```
/// let mut iter = OctahedronIterator::new(BlockPos::default(), 4);
/// for block_pos in iter {
///    println!("{:?}", block_pos);
/// }
/// ```
pub struct BlockIterator {
    start: BlockPos,
    max_distance: u32,

    pos: BlockPos,
    apothem: u32,
    left: i32,
    right: i32,
}
impl BlockIterator {
    pub fn new(start: BlockPos, max_distance: u32) -> Self {
        Self {
            start,
            max_distance,

            pos: BlockPos {
                x: -1,
                y: -1,
                z: -1,
            },
            apothem: 1,
            left: 1,
            right: 2,
        }
    }
}

impl Iterator for BlockIterator {
    type Item = BlockPos;

    fn next(&mut self) -> Option<Self::Item> {
        if self.apothem > self.max_distance {
            return None;
        }

        self.right -= 1;
        if self.right < 0 {
            self.left -= 1;
            if self.left < 0 {
                self.pos.z += 2;
                if self.pos.z > 1 {
                    self.pos.y += 2;
                    if self.pos.y > 1 {
                        self.pos.x += 2;
                        if self.pos.x > 1 {
                            self.apothem += 1;
                            self.pos.x = -1;
                        }
                        self.pos.y = -1;
                    }
                    self.pos.z = -1;
                }
                self.left = self.apothem as i32;
            }
            self.right = self.left;
        }
        let x = self.pos.x * self.right;
        let y = self.pos.y * ((self.apothem as i32) - self.left);
        let z = self.pos.z * ((self.apothem as i32) - (i32::abs(x) + i32::abs(y)));
        Some(BlockPos { x: x, y, z } + self.start)
    }
}

/// A spiral iterator, useful for iterating over chunks in a world.
///
/// ```
/// let mut iter = ChunkIterator::new(ChunkPos::default(), 4);
/// for chunk_pos in iter {
///   println!("{:?}", chunk_pos);
/// }
/// ```
pub struct ChunkIterator {
    start: ChunkPos,
    number_of_points: u32,

    dir: ChunkPos,

    segment_len: u32,
    pos: ChunkPos,
    segment_passed: u32,
    current_iter: u32,
}
impl ChunkIterator {
    pub fn new(start: ChunkPos, max_distance: u32) -> Self {
        Self {
            start,
            number_of_points: u32::pow(max_distance * 2 - 1, 2),

            dir: ChunkPos { x: 1, z: 0 },

            segment_len: 1,
            pos: ChunkPos::default(),
            segment_passed: 0,
            current_iter: 0,
        }
    }
}
impl Iterator for ChunkIterator {
    type Item = ChunkPos;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_iter > self.number_of_points {
            return None;
        }

        let output = self.start + self.dir;

        // make a step, add the direction to the current position
        self.pos.x += self.dir.x;
        self.pos.z += self.dir.z;
        self.segment_passed += 1;

        if self.segment_passed == self.segment_len {
            // done with current segment
            self.segment_passed = 0;

            // rotate directions
            (self.dir.x, self.dir.z) = (-self.dir.z, self.dir.x);

            // increase segment length if necessary
            if self.dir.z == 0 {
                self.segment_len += 1;
            }
        }
        self.current_iter += 1;
        Some(output)
    }
}
