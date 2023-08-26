use crate::{JumpEvent, LookAtEvent};

use super::astar;
use azalea_client::{SprintDirection, StartSprintEvent, StartWalkEvent};
use azalea_core::{BlockPos, CardinalDirection, Vec3};
use azalea_physics::collision::{self, BlockWithShape};
use azalea_world::Instance;
use bevy_ecs::{entity::Entity, event::EventWriter};

type Edge = astar::Edge<BlockPos, MoveData>;

#[derive(Debug, Clone)]
pub struct MoveData {
    pub move_kind: DefaultMoves,
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

const JUMP_COST: f32 = 0.5;
const WALK_ONE_BLOCK_COST: f32 = 1.0;
const FALL_ONE_BLOCK_COST: f32 = 0.5;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum DefaultMoves {
    Forward(CardinalDirection),
    Ascend(CardinalDirection),
    Descend(CardinalDirection),
    Diagonal(CardinalDirection),
}

impl DefaultMoves {
    pub fn get(self, world: &Instance, node: BlockPos) -> Option<Edge> {
        match self {
            DefaultMoves::Forward(dir) => ForwardMove(dir).get(world, node),
            DefaultMoves::Ascend(dir) => AscendMove(dir).get(world, node),
            DefaultMoves::Descend(dir) => DescendMove(dir).get(world, node),
            DefaultMoves::Diagonal(dir) => DiagonalMove(dir).get(world, node),
        }
    }

    pub fn execute(self, ctx: ExecuteCtx) {
        match self {
            DefaultMoves::Forward(_) => ForwardMove::execute(ctx),
            DefaultMoves::Ascend(_) => AscendMove::execute(ctx),
            DefaultMoves::Descend(_) => DescendMove::execute(ctx),
            DefaultMoves::Diagonal(_) => DiagonalMove::execute(ctx),
        }
    }
}

pub trait MoveImpl: Send + Sync {
    fn get(&self, world: &Instance, node: BlockPos) -> Option<Edge>;
    fn execute(ctx: ExecuteCtx);
}

pub struct ExecuteCtx<'w1, 'w2, 'w3, 'w4, 'a> {
    pub entity: Entity,
    pub target: BlockPos,
    pub position: Vec3,

    pub look_at_events: &'a mut EventWriter<'w1, LookAtEvent>,
    pub sprint_events: &'a mut EventWriter<'w2, StartSprintEvent>,
    pub walk_events: &'a mut EventWriter<'w3, StartWalkEvent>,
    pub jump_events: &'a mut EventWriter<'w4, JumpEvent>,
}

pub struct ForwardMove(pub CardinalDirection);
impl MoveImpl for ForwardMove {
    fn get(&self, world: &Instance, pos: BlockPos) -> Option<Edge> {
        let offset = BlockPos::new(self.0.x(), 0, self.0.z());

        if !is_standable(&(pos + offset), world) {
            return None;
        }

        let cost = WALK_ONE_BLOCK_COST;

        Some(Edge {
            movement: astar::Movement {
                target: pos + offset,
                data: MoveData {
                    move_kind: DefaultMoves::Forward(self.0),
                },
            },
            cost,
        })
    }

    fn execute(
        ExecuteCtx {
            entity,
            target,
            look_at_events,
            sprint_events,
            ..
        }: ExecuteCtx,
    ) {
        let center = target.center();
        look_at_events.send(LookAtEvent {
            entity,
            position: center,
        });
        sprint_events.send(StartSprintEvent {
            entity,
            direction: SprintDirection::Forward,
        });
    }
}

pub struct AscendMove(pub CardinalDirection);
impl MoveImpl for AscendMove {
    fn get(&self, world: &Instance, pos: BlockPos) -> Option<Edge> {
        let offset = BlockPos::new(self.0.x(), 1, self.0.z());

        if !is_block_passable(&pos.up(2), world) || !is_standable(&(pos + offset), world) {
            return None;
        }

        let cost = WALK_ONE_BLOCK_COST + JUMP_COST;

        // Some(MoveResult {
        //     node: Node {
        //         pos: node.pos + offset,
        //         vertical_vel: VerticalVel::None,
        //     },
        //     cost,
        // })
        Some(Edge {
            movement: astar::Movement {
                target: pos + offset,
                data: MoveData {
                    move_kind: DefaultMoves::Ascend(self.0),
                },
            },
            cost,
        })
    }

    fn execute(
        ExecuteCtx {
            entity,
            target,
            look_at_events,
            sprint_events,
            jump_events,
            ..
        }: ExecuteCtx,
    ) {
        let center = target.center();
        look_at_events.send(LookAtEvent {
            entity,
            position: center,
        });
        jump_events.send(JumpEvent { entity });
        sprint_events.send(StartSprintEvent {
            entity,
            direction: SprintDirection::Forward,
        });
    }
}
pub struct DescendMove(pub CardinalDirection);
impl MoveImpl for DescendMove {
    fn get(&self, world: &Instance, pos: BlockPos) -> Option<Edge> {
        let new_horizontal_position = pos + BlockPos::new(self.0.x(), 0, self.0.z());
        let fall_distance = fall_distance(&new_horizontal_position, world);
        if fall_distance == 0 {
            return None;
        }
        if fall_distance > 3 {
            return None;
        }
        let new_position = new_horizontal_position.down(fall_distance as i32);

        // check whether 3 blocks vertically forward are passable
        if !is_passable(&new_horizontal_position, world) {
            return None;
        }

        let cost = WALK_ONE_BLOCK_COST + FALL_ONE_BLOCK_COST * fall_distance as f32;

        Some(Edge {
            movement: astar::Movement {
                target: new_position,
                data: MoveData {
                    move_kind: DefaultMoves::Descend(self.0),
                },
            },
            cost,
        })
    }

    fn execute(
        ExecuteCtx {
            entity,
            target,
            look_at_events,
            sprint_events,
            ..
        }: ExecuteCtx,
    ) {
        let center = target.center();
        look_at_events.send(LookAtEvent {
            entity,
            position: center,
        });
        sprint_events.send(StartSprintEvent {
            entity,
            direction: SprintDirection::Forward,
        });
    }
}
pub struct DiagonalMove(pub CardinalDirection);
impl MoveImpl for DiagonalMove {
    fn get(&self, world: &Instance, pos: BlockPos) -> Option<Edge> {
        let right = self.0.right();
        let offset = BlockPos::new(self.0.x() + right.x(), 0, self.0.z() + right.z());

        if !is_passable(
            &BlockPos::new(pos.x + self.0.x(), pos.y, pos.z + self.0.z()),
            world,
        ) && !is_passable(
            &BlockPos::new(
                pos.x + self.0.right().x(),
                pos.y,
                pos.z + self.0.right().z(),
            ),
            world,
        ) {
            return None;
        }
        if !is_standable(&(pos + offset), world) {
            return None;
        }
        let cost = WALK_ONE_BLOCK_COST * 1.4;

        // Some(MoveResult {
        //     node: Node {
        //         pos: node.pos + offset,
        //         vertical_vel: VerticalVel::None,
        //     },
        //     cost,
        // })
        Some(Edge {
            movement: astar::Movement {
                target: pos + offset,
                data: MoveData {
                    move_kind: DefaultMoves::Diagonal(self.0),
                },
            },
            cost,
        })
    }

    fn execute(
        ExecuteCtx {
            entity,
            target,
            look_at_events,
            sprint_events,
            ..
        }: ExecuteCtx,
    ) {
        let center = target.center();
        look_at_events.send(LookAtEvent {
            entity,
            position: center,
        });
        sprint_events.send(StartSprintEvent {
            entity,
            direction: SprintDirection::Forward,
        });
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
