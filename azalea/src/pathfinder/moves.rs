use super::{Node, VerticalVel};
use azalea_core::{BlockPos, CardinalDirection};
use azalea_physics::collision::{self, BlockWithShape};
use azalea_world::World;

/// whether this block is passable
fn is_block_passable(pos: &BlockPos, world: &World) -> bool {
    if let Some(block) = world.get_block_state(pos) {
        block.shape() == &collision::empty_shape()
    } else {
        false
    }
}

/// whether this block has a solid hitbox (i.e. we can stand on it)
fn is_block_solid(pos: &BlockPos, world: &World) -> bool {
    if let Some(block) = world.get_block_state(pos) {
        block.shape() == &collision::block_shape()
    } else {
        false
    }
}

/// Whether this block and the block above are passable
fn is_passable(pos: &BlockPos, world: &World) -> bool {
    is_block_passable(pos, world) && is_block_passable(&pos.up(1), world)
}

/// Whether we can stand in this position. Checks if the block below is solid,
/// and that the two blocks above that are passable.

fn is_standable(pos: &BlockPos, world: &World) -> bool {
    is_block_solid(&pos.down(1), world) && is_passable(pos, world)
}

const JUMP_COST: f32 = 0.5;
const WALK_ONE_BLOCK_COST: f32 = 1.0;

pub trait Move {
    fn cost(&self, world: &World, node: &Node) -> f32;
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
    fn cost(&self, world: &World, node: &Node) -> f32 {
        if is_standable(&(node.pos + self.offset()), world)
            && node.vertical_vel == VerticalVel::None
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
    fn cost(&self, world: &World, node: &Node) -> f32 {
        if node.vertical_vel == VerticalVel::None
            && is_block_passable(&node.pos.up(2), world)
            && is_standable(&(node.pos + self.offset()), world)
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
    fn cost(&self, world: &World, node: &Node) -> f32 {
        // check whether 3 blocks vertically forward are passable
        if node.vertical_vel == VerticalVel::None
            && is_standable(&(node.pos + self.offset()), world)
            && is_block_passable(&(node.pos + self.offset().up(2)), world)
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
    fn cost(&self, world: &World, node: &Node) -> f32 {
        if node.vertical_vel != VerticalVel::None {
            return f32::INFINITY;
        }
        if !is_passable(
            &BlockPos::new(node.pos.x + self.0.x(), node.pos.y, node.pos.z + self.0.z()),
            world,
        ) && !is_passable(
            &BlockPos::new(
                node.pos.x + self.0.right().x(),
                node.pos.y,
                node.pos.z + self.0.right().z(),
            ),
            world,
        ) {
            return f32::INFINITY;
        }
        if !is_standable(&(node.pos + self.offset()), world) {
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
        let mut world = World::default();
        world
            .set_chunk(&ChunkPos { x: 0, z: 0 }, Some(Chunk::default()))
            .unwrap();
        world.set_block_state(&BlockPos::new(0, 0, 0), BlockState::Stone);
        world.set_block_state(&BlockPos::new(0, 1, 0), BlockState::Air);

        assert_eq!(is_block_passable(&BlockPos::new(0, 0, 0), &world), false);
        assert_eq!(is_block_passable(&BlockPos::new(0, 1, 0), &world), true);
    }

    #[test]
    fn test_is_solid() {
        let mut world = World::default();
        world
            .set_chunk(&ChunkPos { x: 0, z: 0 }, Some(Chunk::default()))
            .unwrap();
        world.set_block_state(&BlockPos::new(0, 0, 0), BlockState::Stone);
        world.set_block_state(&BlockPos::new(0, 1, 0), BlockState::Air);

        assert_eq!(is_block_solid(&BlockPos::new(0, 0, 0), &world), true);
        assert_eq!(is_block_solid(&BlockPos::new(0, 1, 0), &world), false);
    }

    #[test]
    fn test_is_standable() {
        let mut world = World::default();
        world
            .set_chunk(&ChunkPos { x: 0, z: 0 }, Some(Chunk::default()))
            .unwrap();
        world.set_block_state(&BlockPos::new(0, 0, 0), BlockState::Stone);
        world.set_block_state(&BlockPos::new(0, 1, 0), BlockState::Air);
        world.set_block_state(&BlockPos::new(0, 2, 0), BlockState::Air);
        world.set_block_state(&BlockPos::new(0, 3, 0), BlockState::Air);

        assert!(is_standable(&BlockPos::new(0, 1, 0), &world));
        assert!(!is_standable(&BlockPos::new(0, 0, 0), &world));
        assert!(!is_standable(&BlockPos::new(0, 2, 0), &world));
    }
}
