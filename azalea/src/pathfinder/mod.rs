mod astar;
pub mod goals;
mod moves;
pub mod simulation;

use crate::bot::{JumpEvent, LookAtEvent};
use crate::pathfinder::astar::a_star;
use crate::pathfinder::moves::DefaultMoves;
use crate::WalkDirection;

use crate::app::{App, Plugin};
use crate::ecs::{
    component::Component,
    entity::Entity,
    event::{EventReader, EventWriter},
    query::{With, Without},
    system::{Commands, Query, Res},
};
use azalea_client::movement::walk_listener;
use azalea_client::{StartSprintEvent, StartWalkEvent};
use azalea_core::{BlockPos, CardinalDirection};
use azalea_entity::metadata::Player;
use azalea_entity::Local;
use azalea_entity::{Physics, Position};
use azalea_physics::PhysicsSet;
use azalea_world::{InstanceContainer, InstanceName};
use bevy_app::{FixedUpdate, PreUpdate, Update};
use bevy_ecs::prelude::Event;
use bevy_ecs::query::Changed;
use bevy_ecs::schedule::IntoSystemConfigs;
use bevy_tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future;
use log::{debug, error, trace};
use std::collections::VecDeque;
use std::sync::Arc;
use std::time::Duration;

use self::moves::ExecuteCtx;

#[derive(Clone, Default)]
pub struct PathfinderPlugin;
impl Plugin for PathfinderPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GotoEvent>()
            .add_event::<PathFoundEvent>()
            .add_systems(
                FixedUpdate,
                // putting systems in the FixedUpdate schedule makes them run every Minecraft tick
                // (every 50 milliseconds).
                tick_execute_path
                    .after(PhysicsSet)
                    .after(azalea_client::movement::send_position),
            )
            .add_systems(PreUpdate, add_default_pathfinder)
            .add_systems(
                Update,
                (
                    goto_listener,
                    handle_tasks,
                    path_found_listener,
                    stop_pathfinding_on_instance_change.before(walk_listener),
                )
                    .chain(),
            );
    }
}

/// A component that makes this entity able to pathfind.
#[derive(Component, Default)]
pub struct Pathfinder {
    pub path: VecDeque<astar::Movement<BlockPos, moves::MoveData>>,
}
#[allow(clippy::type_complexity)]
fn add_default_pathfinder(
    mut commands: Commands,
    mut query: Query<Entity, (Without<Pathfinder>, With<Local>, With<Player>)>,
) {
    for entity in &mut query {
        commands.entity(entity).insert(Pathfinder::default());
    }
}

pub trait PathfinderClientExt {
    fn goto(&self, goal: impl Goal + Send + Sync + 'static);
}

impl PathfinderClientExt for azalea_client::Client {
    /// ```
    /// # use azalea::prelude::*;
    /// # use azalea::{BlockPos, pathfinder::goals::BlockPosGoal};
    /// # fn example(bot: &Client) {
    /// bot.goto(BlockPosGoal::from(BlockPos::new(0, 70, 0)));
    /// # }
    /// ```
    fn goto(&self, goal: impl Goal + Send + Sync + 'static) {
        self.ecs.lock().send_event(GotoEvent {
            entity: self.entity,
            goal: Arc::new(goal),
        });
    }
}
#[derive(Event)]
pub struct GotoEvent {
    pub entity: Entity,
    pub goal: Arc<dyn Goal + Send + Sync>,
}
#[derive(Event)]
pub struct PathFoundEvent {
    pub entity: Entity,
    pub path: Option<VecDeque<astar::Movement<BlockPos, moves::MoveData>>>,
}

#[derive(Component)]
pub struct ComputePath(Task<Option<PathFoundEvent>>);

fn goto_listener(
    mut commands: Commands,
    mut events: EventReader<GotoEvent>,
    mut query: Query<(&Position, &InstanceName)>,
    instance_container: Res<InstanceContainer>,
) {
    let thread_pool = AsyncComputeTaskPool::get();

    for event in events.iter() {
        let (position, world_name) = query
            .get_mut(event.entity)
            .expect("Called goto on an entity that's not in the world");
        let start = BlockPos::from(position);

        let world_lock = instance_container
            .get(world_name)
            .expect("Entity tried to pathfind but the entity isn't in a valid world");
        let end = event.goal.goal_node();

        let goal = event.goal.clone();
        let entity = event.entity;

        let task = thread_pool.spawn(async move {
            debug!("start: {start:?}, end: {end:?}");

            let possible_moves: Vec<DefaultMoves> = vec![
                DefaultMoves::Forward(CardinalDirection::North),
                DefaultMoves::Forward(CardinalDirection::East),
                DefaultMoves::Forward(CardinalDirection::South),
                DefaultMoves::Forward(CardinalDirection::West),
                //
                DefaultMoves::Ascend(CardinalDirection::North),
                DefaultMoves::Ascend(CardinalDirection::East),
                DefaultMoves::Ascend(CardinalDirection::South),
                DefaultMoves::Ascend(CardinalDirection::West),
                //
                DefaultMoves::Descend(CardinalDirection::North),
                DefaultMoves::Descend(CardinalDirection::East),
                DefaultMoves::Descend(CardinalDirection::South),
                DefaultMoves::Descend(CardinalDirection::West),
                //
                DefaultMoves::Diagonal(CardinalDirection::North),
                DefaultMoves::Diagonal(CardinalDirection::East),
                DefaultMoves::Diagonal(CardinalDirection::South),
                DefaultMoves::Diagonal(CardinalDirection::West),
            ];

            let successors = |pos: BlockPos| {
                let mut edges = Vec::new();

                let world = world_lock.read();
                for possible_move in &possible_moves {
                    let move_result = possible_move.get(&world, pos);
                    if let Some(edge) = move_result {
                        edges.push(edge);
                    }
                }

                edges
            };

            let start_time = std::time::Instant::now();
            let astar::Path { movements, partial } = a_star(
                start,
                |n| goal.heuristic(n),
                successors,
                |n| goal.success(n),
                Duration::from_secs(1),
            );
            let end_time = std::time::Instant::now();
            debug!("movements: {movements:?}");
            debug!("partial: {partial:?}");
            debug!("time: {:?}", end_time - start_time);

            let path = movements.into_iter().collect::<VecDeque<_>>();
            Some(PathFoundEvent {
                entity,
                path: Some(path),
            })
        });

        commands.spawn(ComputePath(task));
    }
}

// poll the tasks and send the PathFoundEvent if they're done
fn handle_tasks(
    mut commands: Commands,
    mut transform_tasks: Query<(Entity, &mut ComputePath)>,
    mut path_found_events: EventWriter<PathFoundEvent>,
) {
    for (entity, mut task) in &mut transform_tasks {
        if let Some(optional_path_found_event) = future::block_on(future::poll_once(&mut task.0)) {
            if let Some(path_found_event) = optional_path_found_event {
                path_found_events.send(path_found_event);
            }

            // Task is complete, so remove task component from entity
            commands.entity(entity).remove::<ComputePath>();
        }
    }
}

// set the path for the target entity when we get the PathFoundEvent
fn path_found_listener(mut events: EventReader<PathFoundEvent>, mut query: Query<&mut Pathfinder>) {
    for event in events.iter() {
        let mut pathfinder = query
            .get_mut(event.entity)
            .expect("Path found for an entity that doesn't have a pathfinder");
        if let Some(path) = &event.path {
            pathfinder.path = path.to_owned();
        } else {
            error!("No path found");
            pathfinder.path.clear();
        }
    }
}

fn tick_execute_path(
    mut query: Query<(Entity, &mut Pathfinder, &Position, &Physics)>,
    mut look_at_events: EventWriter<LookAtEvent>,
    mut sprint_events: EventWriter<StartSprintEvent>,
    mut walk_events: EventWriter<StartWalkEvent>,
    mut jump_events: EventWriter<JumpEvent>,
) {
    for (entity, mut pathfinder, position, physics) in &mut query {
        loop {
            let Some(movement) = pathfinder.path.front() else {
                break;
            };
            // we check if the goal was reached *before* actually executing the movement so
            // we don't unnecessarily execute a movement when it wasn't necessary
            if is_goal_reached(movement.target, position, physics) {
                // println!("reached target");
                pathfinder.path.pop_front();
                if pathfinder.path.is_empty() {
                    // println!("reached goal");
                    walk_events.send(StartWalkEvent {
                        entity,
                        direction: WalkDirection::None,
                    });
                }
                // tick again, maybe we already reached the next node!
                continue;
            }

            let ctx = ExecuteCtx {
                entity,
                target: movement.target,
                position: **position,
                look_at_events: &mut look_at_events,
                sprint_events: &mut sprint_events,
                walk_events: &mut walk_events,
                jump_events: &mut jump_events,
            };
            trace!("executing move {:?}", movement.data.move_kind);
            movement.data.move_kind.execute(ctx);
            break;
        }
    }
}

fn stop_pathfinding_on_instance_change(
    mut query: Query<(Entity, &mut Pathfinder), Changed<InstanceName>>,
    mut walk_events: EventWriter<StartWalkEvent>,
) {
    for (entity, mut pathfinder) in &mut query {
        if !pathfinder.path.is_empty() {
            debug!("instance changed, clearing path");
            pathfinder.path.clear();
            walk_events.send(StartWalkEvent {
                entity,
                direction: WalkDirection::None,
            });
        }
    }
}

pub trait Goal {
    fn heuristic(&self, n: BlockPos) -> f32;
    fn success(&self, n: BlockPos) -> bool;
    // TODO: this should be removed and mtdstarlite should stop depending on
    // being given a goal node
    fn goal_node(&self) -> BlockPos;
}

/// Returns whether the entity is at the node and should start going to the
/// next node.
#[must_use]
pub fn is_goal_reached(goal_pos: BlockPos, current_pos: &Position, physics: &Physics) -> bool {
    // println!(
    //     "entity.delta.y: {} {:?}=={:?}, self.vertical_vel={:?}",
    //     entity.delta.y,
    //     BlockPos::from(entity.pos()),
    //     self.pos,
    //     self.vertical_vel
    // );
    BlockPos::from(current_pos) == goal_pos && physics.on_ground
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, sync::Arc};

    use azalea_core::{BlockPos, ChunkPos, Vec3};
    use azalea_world::{Chunk, ChunkStorage, PartialChunkStorage};
    use bevy_log::LogPlugin;
    use log::info;

    use super::{
        goals::BlockPosGoal,
        simulation::{SimulatedPlayerBundle, Simulation},
        GotoEvent,
    };

    fn setup_simulation(
        partial_chunks: &mut PartialChunkStorage,
        start_pos: BlockPos,
        end_pos: BlockPos,
        solid_blocks: Vec<BlockPos>,
    ) -> Simulation {
        let mut chunk_positions = HashSet::new();
        for block_pos in &solid_blocks {
            chunk_positions.insert(ChunkPos::from(block_pos));
        }

        let mut chunks = ChunkStorage::default();
        for chunk_pos in chunk_positions {
            partial_chunks.set(&chunk_pos, Some(Chunk::default()), &mut chunks);
        }
        for block_pos in solid_blocks {
            chunks.set_block_state(&block_pos, azalea_registry::Block::Stone.into());
        }
        let player = SimulatedPlayerBundle::new(Vec3::new(
            start_pos.x as f64 + 0.5,
            start_pos.y as f64,
            start_pos.z as f64 + 0.5,
        ));
        let mut simulation = Simulation::new(chunks, player);
        simulation.app.add_plugins(LogPlugin {
            level: bevy_log::Level::TRACE,
            filter: "".to_string(),
        });

        simulation.app.world.send_event(GotoEvent {
            entity: simulation.entity,
            goal: Arc::new(BlockPosGoal::from(end_pos)),
        });
        simulation
    }

    #[test]
    fn test_simple_forward() {
        let mut partial_chunks = PartialChunkStorage::default();
        let mut simulation = setup_simulation(
            &mut partial_chunks,
            BlockPos::new(0, 71, 0),
            BlockPos::new(0, 71, 1),
            vec![BlockPos::new(0, 70, 0), BlockPos::new(0, 70, 1)],
        );
        for _ in 0..20 {
            simulation.tick();
        }
        assert_eq!(
            BlockPos::from(simulation.position()),
            BlockPos::new(0, 71, 1)
        );
    }

    #[test]
    fn test_double_diagonal_with_walls() {
        let mut partial_chunks = PartialChunkStorage::default();
        let mut simulation = setup_simulation(
            &mut partial_chunks,
            BlockPos::new(0, 71, 0),
            BlockPos::new(2, 71, 2),
            vec![
                BlockPos::new(0, 70, 0),
                BlockPos::new(1, 70, 1),
                BlockPos::new(2, 70, 2),
                BlockPos::new(1, 72, 0),
                BlockPos::new(2, 72, 1),
            ],
        );
        for i in 0..30 {
            simulation.tick();
            info!("-- tick #{i} --")
        }
        assert_eq!(
            BlockPos::from(simulation.position()),
            BlockPos::new(2, 71, 2)
        );
    }
}
