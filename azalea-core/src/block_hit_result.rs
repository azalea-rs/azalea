use crate::{
    direction::Direction,
    position::{BlockPos, Vec3},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BlockHitResult {
    pub location: Vec3,
    pub direction: Direction,
    pub block_pos: BlockPos,
    pub miss: bool,
    pub inside: bool,
    pub world_border: bool,
}

impl BlockHitResult {
    pub fn miss(location: Vec3, direction: Direction, block_pos: BlockPos) -> Self {
        Self {
            location,
            direction,
            block_pos,
            miss: true,
            inside: false,
            world_border: false,
        }
    }

    pub fn with_direction(&self, direction: Direction) -> Self {
        Self { direction, ..*self }
    }
    pub fn with_position(&self, block_pos: BlockPos) -> Self {
        Self { block_pos, ..*self }
    }
}
