use azalea_core::BlockPos;
use azalea_physics::collision::{self, BlockWithShape};
use azalea_world::Dimension;

/// whether this block is passable
fn is_passable(pos: &BlockPos, dim: &Dimension) -> bool {
    if let Some(block) = dim.get_block_state(pos) {
        block.shape() == &collision::empty_shape()
    } else {
        false
    }
}

/// whether this block has a solid hitbox (i.e. we can stand on it)
fn is_solid(pos: &BlockPos, dim: &Dimension) -> bool {
    if let Some(block) = dim.get_block_state(pos) {
        block.shape() == &collision::block_shape()
    } else {
        false
    }
}

/// Whether we can stand in this position. Checks if the block below is solid,
/// and that the two blocks above that are passable.
fn is_standable(pos: &BlockPos, dim: &Dimension) -> bool {
    is_solid(&pos.down(1), dim) && is_passable(&pos, dim) && is_passable(&pos.up(1), dim)
}

pub trait Move {
    fn can_execute(&self, dim: &Dimension, pos: &BlockPos) -> bool;
    /// Returns by how much the entity's position should be changed when this move is executed.
    fn offset(&self) -> BlockPos;
}

pub struct NorthMove;
impl Move for NorthMove {
    fn can_execute(&self, dim: &Dimension, pos: &BlockPos) -> bool {
        is_standable(&(pos + &self.offset()), dim)
    }
    fn offset(&self) -> BlockPos {
        BlockPos::new(0, 0, -1)
    }
}

pub struct SouthMove;
impl Move for SouthMove {
    fn can_execute(&self, dim: &Dimension, pos: &BlockPos) -> bool {
        is_standable(&(pos + &self.offset()), dim)
    }
    fn offset(&self) -> BlockPos {
        BlockPos::new(0, 0, 1)
    }
}

pub struct EastMove;
impl Move for EastMove {
    fn can_execute(&self, dim: &Dimension, pos: &BlockPos) -> bool {
        is_standable(&(pos + &self.offset()), dim)
    }
    fn offset(&self) -> BlockPos {
        BlockPos::new(1, 0, 0)
    }
}

pub struct WestMove;
impl Move for WestMove {
    fn can_execute(&self, dim: &Dimension, pos: &BlockPos) -> bool {
        is_standable(&(pos + &self.offset()), dim)
    }
    fn offset(&self) -> BlockPos {
        BlockPos::new(-1, 0, 0)
    }
}
