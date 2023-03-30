use super::{Node, VerticalVel};
use azalea_core::{BlockPos, CardinalDirection};
use azalea_physics::collision::{self, BlockWithShape};
use azalea_world::Instance;

/// whether this block is passable
fn is_block_passable(pos: &BlockPos, world: &Instance) -> bool {
    world
        .chunks
        .get_block_state(pos)
        .map_or(false, |block| block.shape() == &collision::empty_shape())
}

/// whether this block has a solid hitbox (i.e. we can stand on it)
fn is_block_solid(pos: &BlockPos, world: &Instance) -> bool {
    world
        .chunks
        .get_block_state(pos)
        .map_or(false, |block| block.shape() == &collision::block_shape())
}

/// Whether this block and the block above are passable
fn is_passable(pos: &BlockPos, world: &Instance) -> bool {
    is_block_passable(pos, world) && is_block_passable(&pos.up(1), world)
}

/// Whether we can stand in this position. Checks if the block below is solid,
/// and that the two blocks above that are passable.
fn is_standable(pos: &BlockPos, world: &Instance) -> bool {
    is_block_solid(&pos.down(1), world) && is_passable(pos, world)
}

const JUMP_COST: f32 = 0.5;
const WALK_ONE_BLOCK_COST: f32 = 1.0;

pub trait Move: Send + Sync {
    fn cost(&self, world: &Instance, node: &Node) -> f32;
    fn offset(&self) -> BlockPos;
    fn next_node(&self, node: &Node) -> Node {
        Node {
            pos: node.pos + self.offset(),
            vertical_vel: VerticalVel::None,
        }
    }
}

macro_rules! impl_move {
    ($name:ident, $y_offset:expr, $cost:expr) => {
        pub struct $name(pub CardinalDirection);
        impl Move for $name {
            fn cost(&self, world: &Instance, node: &Node) -> f32 {
                if is_standable(&(node.pos + self.offset()), world)
                    && node.vertical_vel == VerticalVel::None
                {
                    $cost
                } else {
                    f32::INFINITY
                }
            }
            fn offset(&self) -> BlockPos {
                BlockPos::new(self.0.x(), $y_offset, self.0.z())
            }
        }
    };
}

impl_move!(ForwardMove, 0, WALK_ONE_BLOCK_COST);
impl_move!(AscendMove, 1, WALK_ONE_BLOCK_COST + JUMP_COST);
impl_move!(DescendMove, -1, WALK_ONE_BLOCK_COST);

pub struct DiagonalMove(pub CardinalDirection);
impl Move for DiagonalMove {
    fn cost(&self, world: &Instance, node: &Node) -> f32 {
        if node.vertical_vel != VerticalVel::None {
            return f32::INFINITY;
        }
        let diagonal_pos = node.pos + self.offset();
        if !is_passable(&diagonal_pos, world) && !is_passable(&diagonal_pos.up(1), world) {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use azalea_block::BlockState;
    use azalea_core::ChunkPos;
    use azalea_world::{Chunk, ChunkStorage, PartialInstance};

    #[test]
    fn test_is_passable() {
        let mut partial_world = PartialInstance::default();
        let mut chunk_storage = ChunkStorage::default();

        partial_world.chunks.set(
            &ChunkPos { x: 0, z: 0 },
            Some(Chunk::default()),
            &mut chunk_storage,
        );
        partial_world.chunks.set_block_state(
            &BlockPos::new(0, 0, 0),
            azalea_registry::Block::Stone.into(),
            &mut chunk_storage,
        );
        partial_world.chunks.set_block_state(
            &BlockPos::new(0, 1, 0),
            BlockState::AIR,
            &mut chunk_storage,
        );

        let world = chunk_storage.into();
        assert_eq!(is_block_passable(&BlockPos::new(0, 0, 0), &world), false);
        assert_eq!(is_block_passable(&BlockPos::new(0, 1, 0), &world), true);
    }

    #[test]
    fn test_is_solid() {
        let mut partial_world = PartialInstance::default();
        let mut chunk_storage = ChunkStorage::default();
        partial_world.chunks.set(
            &ChunkPos { x: 0, z: 0 },
            Some(Chunk::default()),
            &mut chunk_storage,
        );
        partial_world.chunks.set_block_state(
            &BlockPos::new(0, 0, 0),
            azalea_registry::Block::Stone.into(),
            &mut chunk_storage,
        );
        partial_world.chunks.set_block_state(
            &BlockPos::new(0, 1, 0),
            BlockState::AIR,
            &mut chunk_storage,
        );

        let world = chunk_storage.into();
        assert_eq!(is_block_solid(&BlockPos::new(0, 0, 0), &world), true);
        assert_eq!(is_block_solid(&BlockPos::new(0, 1, 0), &world), false);
    }

    #[test]
    fn test_is_standable() {
        let mut partial_world = PartialInstance::default();
        let mut chunk_storage = ChunkStorage::default();
        partial_world.chunks.set(
            &ChunkPos { x: 0, z: 0 },
            Some(Chunk::default()),
            &mut chunk_storage,
        );
        partial_world.chunks.set_block_state(
            &BlockPos::new(0, 0, 0),
            azalea_registry::Block::Stone.into(),
            &mut chunk_storage,
        );
        partial_world.chunks.set_block_state(
            &BlockPos::new(0, 1, 0),
            BlockState::AIR,
            &mut chunk_storage,
        );
        partial_world.chunks.set_block_state(
            &BlockPos::new(0, 2, 0),
            BlockState::AIR,
            &mut chunk_storage,
        );
        partial_world.chunks.set_block_state(
            &BlockPos::new(0, 3, 0),
            BlockState::AIR,
            &mut chunk_storage,
        );

        let world = chunk_storage.into();
        assert!(is_standable(&BlockPos::new(0, 1, 0), &world));
        assert!(!is_standable(&BlockPos::new(0, 0, 0), &world));
        assert!(!is_standable(&BlockPos::new(0, 2, 0), &world));
    }
}
