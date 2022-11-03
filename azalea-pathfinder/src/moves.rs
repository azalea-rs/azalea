use azalea_block::Block;
use azalea_core::BlockPos;
use azalea_physics::collision::{self, BlockWithShape};
use azalea_world::Dimension;

use crate::{Node, VerticalVel};

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
    fn can_execute(&self, dim: &Dimension, node: &Node) -> bool;
    /// Returns by how much the entity's position should be changed when this move is executed.
    fn offset(&self) -> BlockPos;
    fn next_node(&self, node: &Node) -> Node {
        Node {
            pos: node.pos + self.offset(),
            vertical_vel: VerticalVel::None,
        }
    }
    fn cost(&self) -> f32 {
        1.0
    }
}

pub struct NorthMove;
impl Move for NorthMove {
    fn can_execute(&self, dim: &Dimension, node: &Node) -> bool {
        is_standable(&(node.pos + self.offset()), dim)
            && (node.vertical_vel == VerticalVel::None
                || node.vertical_vel == VerticalVel::NoneMidair)
    }
    fn offset(&self) -> BlockPos {
        BlockPos::new(0, 0, -1)
    }
}

pub struct SouthMove;
impl Move for SouthMove {
    fn can_execute(&self, dim: &Dimension, node: &Node) -> bool {
        is_standable(&(node.pos + self.offset()), dim)
            && (node.vertical_vel == VerticalVel::None
                || node.vertical_vel == VerticalVel::NoneMidair)
    }
    fn offset(&self) -> BlockPos {
        BlockPos::new(0, 0, 1)
    }
}

pub struct EastMove;
impl Move for EastMove {
    fn can_execute(&self, dim: &Dimension, node: &Node) -> bool {
        is_standable(&(node.pos + self.offset()), dim)
            && (node.vertical_vel == VerticalVel::None
                || node.vertical_vel == VerticalVel::NoneMidair)
    }
    fn offset(&self) -> BlockPos {
        BlockPos::new(1, 0, 0)
    }
}

pub struct WestMove;
impl Move for WestMove {
    fn can_execute(&self, dim: &Dimension, node: &Node) -> bool {
        is_standable(&(node.pos + self.offset()), dim)
            && (node.vertical_vel == VerticalVel::None
                || node.vertical_vel == VerticalVel::NoneMidair)
    }
    fn offset(&self) -> BlockPos {
        BlockPos::new(-1, 0, 0)
    }
}

pub struct JumpUpMove;
impl Move for JumpUpMove {
    fn can_execute(&self, dim: &Dimension, node: &Node) -> bool {
        is_passable(&node.pos.up(2), dim) && node.vertical_vel == VerticalVel::None
    }
    fn offset(&self) -> BlockPos {
        BlockPos::new(0, 1, 0)
    }
    fn next_node(&self, node: &Node) -> Node {
        Node {
            pos: node.pos + self.offset(),
            vertical_vel: VerticalVel::NoneMidair,
        }
    }
}
pub struct FallNorthMove;
impl Move for FallNorthMove {
    fn can_execute(&self, dim: &Dimension, node: &Node) -> bool {
        // check whether 3 blocks vertically forward are passable
        is_passable(&(node.pos + self.offset().down(1)), dim)
            && is_passable(&(node.pos + self.offset()), dim)
            && is_passable(&(node.pos + self.offset().up(1)), dim)
            && node.vertical_vel == VerticalVel::None
    }
    fn offset(&self) -> BlockPos {
        BlockPos::new(0, 0, -1)
    }
    fn next_node(&self, node: &Node) -> Node {
        Node {
            pos: node.pos + self.offset(),
            vertical_vel: VerticalVel::NoneMidair,
        }
    }
}
pub struct FallSouthMove;
impl Move for FallSouthMove {
    fn can_execute(&self, dim: &Dimension, node: &Node) -> bool {
        // check whether 3 blocks vertically forward are passable
        is_passable(&(node.pos + self.offset().down(1)), dim)
            && is_passable(&(node.pos + self.offset()), dim)
            && is_passable(&(node.pos + self.offset().up(1)), dim)
            && node.vertical_vel == VerticalVel::None
    }
    fn offset(&self) -> BlockPos {
        BlockPos::new(0, 0, 1)
    }
    fn next_node(&self, node: &Node) -> Node {
        Node {
            pos: node.pos + self.offset(),
            vertical_vel: VerticalVel::NoneMidair,
        }
    }
}
pub struct FallEastMove;
impl Move for FallEastMove {
    fn can_execute(&self, dim: &Dimension, node: &Node) -> bool {
        // check whether 3 blocks vertically forward are passable
        is_passable(&(node.pos + self.offset().down(1)), dim)
            && is_passable(&(node.pos + self.offset()), dim)
            && is_passable(&(node.pos + self.offset().up(1)), dim)
            && node.vertical_vel == VerticalVel::None
    }
    fn offset(&self) -> BlockPos {
        BlockPos::new(1, 0, 0)
    }
    fn next_node(&self, node: &Node) -> Node {
        Node {
            pos: node.pos + self.offset(),
            vertical_vel: VerticalVel::NoneMidair,
        }
    }
}
pub struct FallWestMove;
impl Move for FallWestMove {
    fn can_execute(&self, dim: &Dimension, node: &Node) -> bool {
        // check whether 3 blocks vertically forward are passable
        is_passable(&(node.pos + self.offset().down(1)), dim)
            && is_passable(&(node.pos + self.offset()), dim)
            && is_passable(&(node.pos + self.offset().up(1)), dim)
            && node.vertical_vel == VerticalVel::None
    }
    fn offset(&self) -> BlockPos {
        BlockPos::new(-1, 0, 0)
    }
    fn next_node(&self, node: &Node) -> Node {
        Node {
            pos: node.pos + self.offset(),
            vertical_vel: VerticalVel::NoneMidair,
        }
    }
}

pub struct LandMove;
impl Move for LandMove {
    fn can_execute(&self, dim: &Dimension, node: &Node) -> bool {
        is_standable(&(node.pos + self.offset()), dim)
            && (node.vertical_vel == VerticalVel::NoneMidair
                || node.vertical_vel == VerticalVel::FallingLittle)
    }
    fn offset(&self) -> BlockPos {
        BlockPos::new(0, -1, 0)
    }
    fn next_node(&self, node: &Node) -> Node {
        Node {
            pos: node.pos + self.offset(),
            vertical_vel: VerticalVel::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azalea_block::BlockState;
    use azalea_core::ChunkPos;
    use azalea_world::Chunk;

    #[test]
    fn test_is_passable() {
        let mut dim = Dimension::default();
        dim.set_chunk(&ChunkPos { x: 0, z: 0 }, Some(Chunk::default()))
            .unwrap();
        dim.set_block_state(&BlockPos::new(0, 0, 0), BlockState::Stone);
        dim.set_block_state(&BlockPos::new(0, 1, 0), BlockState::Air);

        assert_eq!(is_passable(&BlockPos::new(0, 0, 0), &dim), false);
        assert_eq!(is_passable(&BlockPos::new(0, 1, 0), &dim), true);
    }

    #[test]
    fn test_is_solid() {
        let mut dim = Dimension::default();
        dim.set_chunk(&ChunkPos { x: 0, z: 0 }, Some(Chunk::default()))
            .unwrap();
        dim.set_block_state(&BlockPos::new(0, 0, 0), BlockState::Stone);
        dim.set_block_state(&BlockPos::new(0, 1, 0), BlockState::Air);

        assert_eq!(is_solid(&BlockPos::new(0, 0, 0), &dim), true);
        assert_eq!(is_solid(&BlockPos::new(0, 1, 0), &dim), false);
    }

    #[test]
    fn test_is_standable() {
        let mut dim = Dimension::default();
        dim.set_chunk(&ChunkPos { x: 0, z: 0 }, Some(Chunk::default()))
            .unwrap();
        dim.set_block_state(&BlockPos::new(0, 0, 0), BlockState::Stone);
        dim.set_block_state(&BlockPos::new(0, 1, 0), BlockState::Air);
        dim.set_block_state(&BlockPos::new(0, 2, 0), BlockState::Air);
        dim.set_block_state(&BlockPos::new(0, 3, 0), BlockState::Air);

        assert!(is_standable(&BlockPos::new(0, 1, 0), &dim));
        assert!(!is_standable(&BlockPos::new(0, 0, 0), &dim));
        assert!(!is_standable(&BlockPos::new(0, 2, 0), &dim));
    }
}
