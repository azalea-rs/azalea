use azalea_core::{BlockPos, CardinalDirection};
use azalea_physics::collision::{self, BlockWithShape};
use azalea_world::Dimension;

use crate::{Node, VerticalVel};

/// whether this block is passable
fn is_block_passable(pos: &BlockPos, dim: &Dimension) -> bool {
    if let Some(block) = dim.get_block_state(pos) {
        block.shape() == &collision::empty_shape()
    } else {
        false
    }
}

/// whether this block has a solid hitbox (i.e. we can stand on it)
fn is_block_solid(pos: &BlockPos, dim: &Dimension) -> bool {
    if let Some(block) = dim.get_block_state(pos) {
        block.shape() == &collision::block_shape()
    } else {
        false
    }
}

/// Whether this block and the block above are passable
fn is_passable(pos: &BlockPos, dim: &Dimension) -> bool {
    is_block_passable(pos, dim) && is_block_passable(&pos.up(1), dim)
}

/// Whether we can stand in this position. Checks if the block below is solid,
/// and that the two blocks above that are passable.
fn is_standable(pos: &BlockPos, dim: &Dimension) -> bool {
    is_block_solid(&pos.down(1), dim) && is_passable(&pos, dim)
}

const JUMP_COST: f32 = 0.5;
const WALK_ONE_BLOCK_COST: f32 = 1.0;

pub trait Move {
    fn cost(&self, dim: &Dimension, node: &Node) -> f32;
    /// Returns by how much the entity's position should be changed when this move is executed.
    fn offset(&self) -> BlockPos;
    fn next_node(&self, node: &Node) -> Node {
        Node {
            pos: node.pos + self.offset(),
            vertical_vel: VerticalVel::None,
        }
    }
}

pub struct ForwardMove(pub CardinalDirection);
impl Move for ForwardMove {
    fn cost(&self, dim: &Dimension, node: &Node) -> f32 {
        if is_standable(&(node.pos + self.offset()), dim) && node.vertical_vel == VerticalVel::None
        {
            WALK_ONE_BLOCK_COST
        } else {
            f32::INFINITY
        }
    }
    fn offset(&self) -> BlockPos {
        BlockPos::new(self.0.x(), 0, self.0.z())
    }
}

pub struct AscendMove(pub CardinalDirection);
impl Move for AscendMove {
    fn cost(&self, dim: &Dimension, node: &Node) -> f32 {
        if node.vertical_vel == VerticalVel::None
            && is_block_passable(&node.pos.up(2), dim)
            && is_standable(&(node.pos + self.offset()), dim)
        {
            WALK_ONE_BLOCK_COST + JUMP_COST
        } else {
            f32::INFINITY
        }
    }
    fn offset(&self) -> BlockPos {
        BlockPos::new(self.0.x(), 1, self.0.z())
    }
    fn next_node(&self, node: &Node) -> Node {
        Node {
            pos: node.pos + self.offset(),
            vertical_vel: VerticalVel::None,
        }
    }
}
pub struct DescendMove(pub CardinalDirection);
impl Move for DescendMove {
    fn cost(&self, dim: &Dimension, node: &Node) -> f32 {
        // check whether 3 blocks vertically forward are passable
        if node.vertical_vel == VerticalVel::None
            && is_standable(&(node.pos + self.offset()), dim)
            && is_block_passable(&(node.pos + self.offset().up(2)), dim)
        {
            WALK_ONE_BLOCK_COST
        } else {
            f32::INFINITY
        }
    }
    fn offset(&self) -> BlockPos {
        BlockPos::new(self.0.x(), -1, self.0.z())
    }
    fn next_node(&self, node: &Node) -> Node {
        Node {
            pos: node.pos + self.offset(),
            vertical_vel: VerticalVel::None,
        }
    }
}
pub struct DiagonalMove(pub CardinalDirection);
impl Move for DiagonalMove {
    fn cost(&self, dim: &Dimension, node: &Node) -> f32 {
        if node.vertical_vel != VerticalVel::None {
            return f32::INFINITY;
        }
        if !is_passable(
            &BlockPos::new(node.pos.x + self.0.x(), 0, node.pos.z + self.0.z()),
            dim,
        ) && !is_passable(
            &BlockPos::new(
                node.pos.x + self.0.right().x(),
                0,
                node.pos.z + self.0.right().z(),
            ),
            dim,
        ) {
            return f32::INFINITY;
        }
        if !is_standable(&(node.pos + self.offset()), dim) {
            return f32::INFINITY;
        }
        WALK_ONE_BLOCK_COST * 1.4
    }
    fn offset(&self) -> BlockPos {
        let right = self.0.right();
        BlockPos::new(self.0.x() + right.x(), 0, self.0.z() + right.z())
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

        assert_eq!(is_block_passable(&BlockPos::new(0, 0, 0), &dim), false);
        assert_eq!(is_block_passable(&BlockPos::new(0, 1, 0), &dim), true);
    }

    #[test]
    fn test_is_solid() {
        let mut dim = Dimension::default();
        dim.set_chunk(&ChunkPos { x: 0, z: 0 }, Some(Chunk::default()))
            .unwrap();
        dim.set_block_state(&BlockPos::new(0, 0, 0), BlockState::Stone);
        dim.set_block_state(&BlockPos::new(0, 1, 0), BlockState::Air);

        assert_eq!(is_block_solid(&BlockPos::new(0, 0, 0), &dim), true);
        assert_eq!(is_block_solid(&BlockPos::new(0, 1, 0), &dim), false);
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
