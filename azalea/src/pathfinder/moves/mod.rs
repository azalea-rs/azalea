pub mod basic;
pub mod parkour;

use std::fmt::Debug;

use crate::{JumpEvent, LookAtEvent};

use super::astar;
use azalea_client::{StartSprintEvent, StartWalkEvent};
use azalea_core::{BlockPos, Vec3};
use azalea_physics::collision::{self, BlockWithShape};
use azalea_world::Instance;
use bevy_ecs::{entity::Entity, event::EventWriter};

type Edge = astar::Edge<BlockPos, MoveData>;

pub type SuccessorsFn =
    fn(&azalea_world::Instance, BlockPos) -> Vec<astar::Edge<BlockPos, MoveData>>;

#[derive(Clone)]
pub struct MoveData {
    /// Use the context to determine what events should be sent to complete this
    /// movement.
    pub execute: &'static (dyn Fn(ExecuteCtx) + Send + Sync),
    /// Whether we've reached the target.
    pub is_reached: &'static (dyn Fn(IsReachedCtx) -> bool + Send + Sync),
}
impl Debug for MoveData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MoveData")
            // .field("move_kind", &self.move_kind)
            .finish()
    }
}

/// whether this block is passable
fn is_block_passable(pos: &BlockPos, world: &Instance) -> bool {
    if let Some(block) = world.chunks.get_block_state(pos) {
        if block.shape() != &collision::empty_shape() {
            return false;
        }
        if block == azalea_registry::Block::Water.into() {
            return false;
        }
        if block.waterlogged() {
            return false;
        }
        // block.waterlogged currently doesn't account for seagrass and some other water
        // blocks
        if block == azalea_registry::Block::Seagrass.into() {
            return false;
        }

        block.shape() == &collision::empty_shape()
    } else {
        false
    }
}

/// whether this block has a solid hitbox (i.e. we can stand on it)
fn is_block_solid(pos: &BlockPos, world: &Instance) -> bool {
    if let Some(block) = world.chunks.get_block_state(pos) {
        block.shape() == &collision::block_shape()
    } else {
        false
    }
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

/// Get the amount of air blocks until the next solid block below this one.
fn fall_distance(pos: &BlockPos, world: &Instance) -> u32 {
    let mut distance = 0;
    let mut current_pos = pos.down(1);
    while is_block_passable(&current_pos, world) {
        distance += 1;
        current_pos = current_pos.down(1);

        if current_pos.y < world.chunks.min_y {
            return u32::MAX;
        }
    }
    distance
}
pub struct ExecuteCtx<'w1, 'w2, 'w3, 'w4, 'a> {
    pub entity: Entity,
    /// The node that we're trying to reach.
    pub target: BlockPos,
    /// The last node that we reached.
    pub start: BlockPos,
    pub position: Vec3,
    pub physics: &'a azalea_entity::Physics,

    pub look_at_events: &'a mut EventWriter<'w1, LookAtEvent>,
    pub sprint_events: &'a mut EventWriter<'w2, StartSprintEvent>,
    pub walk_events: &'a mut EventWriter<'w3, StartWalkEvent>,
    pub jump_events: &'a mut EventWriter<'w4, JumpEvent>,
}
pub struct IsReachedCtx<'a> {
    /// The node that we're trying to reach.
    pub target: BlockPos,
    /// The last node that we reached.
    pub start: BlockPos,
    pub position: Vec3,
    pub physics: &'a azalea_entity::Physics,
}

pub fn default_move(world: &Instance, node: BlockPos) -> Vec<Edge> {
    let mut edges = Vec::new();
    edges.extend(basic::basic_move(world, node));
    edges.extend(parkour::parkour_move(world, node));
    edges
}

/// Returns whether the entity is at the node and should start going to the
/// next node.
#[must_use]
pub fn default_is_reached(
    IsReachedCtx {
        position, target, ..
    }: IsReachedCtx,
) -> bool {
    BlockPos::from(position) == target
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
            &chunk_storage,
        );
        partial_world.chunks.set_block_state(
            &BlockPos::new(0, 1, 0),
            BlockState::AIR,
            &chunk_storage,
        );

        let world = chunk_storage.into();
        assert!(!is_block_passable(&BlockPos::new(0, 0, 0), &world));
        assert!(is_block_passable(&BlockPos::new(0, 1, 0), &world));
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
            &chunk_storage,
        );
        partial_world.chunks.set_block_state(
            &BlockPos::new(0, 1, 0),
            BlockState::AIR,
            &chunk_storage,
        );

        let world = chunk_storage.into();
        assert!(is_block_solid(&BlockPos::new(0, 0, 0), &world));
        assert!(!is_block_solid(&BlockPos::new(0, 1, 0), &world));
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
            &chunk_storage,
        );
        partial_world.chunks.set_block_state(
            &BlockPos::new(0, 1, 0),
            BlockState::AIR,
            &chunk_storage,
        );
        partial_world.chunks.set_block_state(
            &BlockPos::new(0, 2, 0),
            BlockState::AIR,
            &chunk_storage,
        );
        partial_world.chunks.set_block_state(
            &BlockPos::new(0, 3, 0),
            BlockState::AIR,
            &chunk_storage,
        );

        let world = chunk_storage.into();
        assert!(is_standable(&BlockPos::new(0, 1, 0), &world));
        assert!(!is_standable(&BlockPos::new(0, 0, 0), &world));
        assert!(!is_standable(&BlockPos::new(0, 2, 0), &world));
    }
}
