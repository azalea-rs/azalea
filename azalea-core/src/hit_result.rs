use bevy_ecs::entity::Entity;

use crate::{
    direction::Direction,
    position::{BlockPos, Vec3},
};

/// The block or entity that our player is looking at and can interact with.
///
/// If there's nothing, it'll be a [`BlockHitResult`] with `miss` set to true.
#[derive(Debug, Clone, PartialEq)]
pub enum HitResult {
    Block(BlockHitResult),
    Entity(EntityHitResult),
}

impl HitResult {
    pub fn miss(&self) -> bool {
        match self {
            HitResult::Block(r) => r.miss,
            _ => false,
        }
    }
    pub fn location(&self) -> Vec3 {
        match self {
            HitResult::Block(r) => r.location,
            HitResult::Entity(r) => r.location,
        }
    }

    pub fn new_miss(location: Vec3, direction: Direction, block_pos: BlockPos) -> Self {
        HitResult::Block(BlockHitResult {
            location,
            miss: true,
            direction,
            block_pos,
            inside: false,
            world_border: false,
        })
    }

    pub fn is_block_hit_and_not_miss(&self) -> bool {
        matches!(self, HitResult::Block(r) if !r.miss)
    }

    /// Returns the [`BlockHitResult`], if we were looking at a block and it
    /// wasn't a miss.
    pub fn as_block_hit_result_if_not_miss(&self) -> Option<&BlockHitResult> {
        if let HitResult::Block(r) = self
            && !r.miss
        {
            Some(r)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockHitResult {
    pub location: Vec3,
    pub miss: bool,

    pub direction: Direction,
    pub block_pos: BlockPos,
    pub inside: bool,
    pub world_border: bool,
}
impl BlockHitResult {
    pub fn miss(location: Vec3, direction: Direction, block_pos: BlockPos) -> Self {
        Self {
            location,
            miss: true,

            direction,
            block_pos,
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

#[derive(Debug, Clone, PartialEq)]
pub struct EntityHitResult {
    pub location: Vec3,
    pub entity: Entity,
}

impl From<BlockHitResult> for HitResult {
    fn from(value: BlockHitResult) -> Self {
        HitResult::Block(value)
    }
}
impl From<EntityHitResult> for HitResult {
    fn from(value: EntityHitResult) -> Self {
        HitResult::Entity(value)
    }
}
