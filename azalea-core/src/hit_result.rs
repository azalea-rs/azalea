use crate::{
    direction::Direction,
    position::{BlockPos, Vec3},
};

/// The block or entity that our player is looking at and can interact with.
///
/// If there's nothing, it'll be a [`BlockHitResult`] with `miss` set to true.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HitResult {
    Block(BlockHitResult),
    /// TODO
    Entity,
}
impl HitResult {
    pub fn is_miss(&self) -> bool {
        match self {
            HitResult::Block(block_hit_result) => block_hit_result.miss,
            HitResult::Entity => false,
        }
    }

    pub fn is_block_hit_and_not_miss(&self) -> bool {
        match self {
            HitResult::Block(block_hit_result) => !block_hit_result.miss,
            HitResult::Entity => false,
        }
    }

    /// Returns the [`BlockHitResult`], if we were looking at a block and it
    /// wasn't a miss.
    pub fn as_block_hit_result_if_not_miss(&self) -> Option<&BlockHitResult> {
        match self {
            HitResult::Block(block_hit_result) if !block_hit_result.miss => Some(block_hit_result),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BlockHitResult {
    pub location: Vec3,
    pub direction: Direction,
    pub block_pos: BlockPos,
    pub inside: bool,
    pub world_border: bool,
    pub miss: bool,
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
