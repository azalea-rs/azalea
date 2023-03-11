use crate::{BlockPos, Direction, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct BlockHitResult {
    pub location: Vec3,
    pub direction: Direction,
    pub block_pos: BlockPos,
    pub miss: bool,
    pub inside: bool,
}

impl BlockHitResult {
    pub fn miss(location: Vec3, direction: Direction, block_pos: BlockPos) -> Self {
        Self {
            location,
            direction,
            block_pos,
            miss: true,
            inside: false,
        }
    }
}
