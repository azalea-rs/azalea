//! A pathfinding plugin to make bots navigate the world. A lot of this code is
//! based on [Baritone](https://github.com/cabaletta/baritone).

pub mod astar;
pub mod costs;
pub mod goals;
pub mod moves;
pub mod simulation;

use crate::bot::{JumpEvent, LookAtEvent};
use crate::pathfinder::astar::a_star;
use crate::WalkDirection;

use crate::app::{App, Plugin};
use crate::ecs::{
    component::Component,
    entity::Entity,
    event::{EventReader, EventWriter},
    query::{With, Without},
    system::{Commands, Query, Res},
};
use crate::pathfinder::moves::PathfinderCtx;
use azalea_client::movement::walk_listener;
use azalea_client::{StartSprintEvent, StartWalkEvent};
use azalea_core::position::BlockPos;
use azalea_entity::metadata::Player;
use azalea_entity::LocalEntity;
use azalea_entity::{Physics, Position};
use azalea_physics::PhysicsSet;
use azalea_world::{InstanceContainer, InstanceName};
use bevy_app::{FixedUpdate, PreUpdate, Update};
use bevy_ecs::prelude::Event;
use bevy_ecs::query::Changed;
use bevy_ecs::schedule::IntoSystemConfigs;
use bevy_tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future;
use log::{debug, error, info, trace, warn};
use std::collections::VecDeque;
use std::sync::Arc;
use std::time::{Duration, Instant};

use self::moves::{ExecuteCtx, IsReachedCtx, SuccessorsFn};

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
    pub queued_path: Option<VecDeque<astar::Movement<BlockPos, moves::MoveData>>>,
    pub is_path_partial: bool,

    pub last_reached_node: Option<BlockPos>,
    pub last_node_reached_at: Option<Instant>,
    pub goal: Option<Arc<dyn Goal + Send + Sync>>,
    pub successors_fn: Option<SuccessorsFn>,
    pub is_calculating: bool,
}
#[derive(Event)]
pub struct GotoEvent {
    pub entity: Entity,
    pub goal: Arc<dyn Goal + Send + Sync>,
    /// The function that's used for checking what moves are possible. Usually
    /// `pathfinder::moves::default_move`
    pub successors_fn: SuccessorsFn,
}
#[derive(Event)]
pub struct PathFoundEvent {
    pub entity: Entity,
    pub start: BlockPos,
    pub path: Option<VecDeque<astar::Movement<BlockPos, moves::MoveData>>>,
    pub is_partial: bool,
    pub successors_fn: SuccessorsFn,
}

#[allow(clippy::type_complexity)]
fn add_default_pathfinder(
    mut commands: Commands,
    mut query: Query<Entity, (Without<Pathfinder>, With<LocalEntity>, With<Player>)>,
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
    /// bot.goto(BlockPosGoal(BlockPos::new(0, 70, 0)));
    /// # }
    /// ```
    fn goto(&self, goal: impl Goal + Send + Sync + 'static) {
        self.ecs.lock().send_event(GotoEvent {
            entity: self.entity,
            goal: Arc::new(goal),
            successors_fn: moves::default_move,
        });
    }
}

#[derive(Component)]
pub struct ComputePath(Task<Option<PathFoundEvent>>);

fn goto_listener(
    mut commands: Commands,
    mut events: EventReader<GotoEvent>,
    mut query: Query<(&mut Pathfinder, &Position, &InstanceName)>,
    instance_container: Res<InstanceContainer>,
) {
    let thread_pool = AsyncComputeTaskPool::get();

    for event in events.iter() {
        let (mut pathfinder, position, instance_name) = query
            .get_mut(event.entity)
            .expect("Called goto on an entity that's not in the world");

        // we store the goal so it can be recalculated later if necessary
        pathfinder.goal = Some(event.goal.clone());
        pathfinder.successors_fn = Some(event.successors_fn);
        pathfinder.is_calculating = true;

        let start = if pathfinder.path.is_empty() {
            BlockPos::from(position)
        } else {
            // if we're currently pathfinding and got a goto event, start a little ahead
            pathfinder
                .path
                .get(5)
                .unwrap_or_else(|| pathfinder.path.back().unwrap())
                .target
        };
        info!(
            "got goto, starting from {start:?} (currently at {:?})",
            BlockPos::from(position)
        );

        let successors_fn: moves::SuccessorsFn = event.successors_fn;

        let world_lock = instance_container
            .get(instance_name)
            .expect("Entity tried to pathfind but the entity isn't in a valid world");

        let goal = event.goal.clone();
        let entity = event.entity;

        let task = thread_pool.spawn(async move {
            debug!("start: {start:?}");

            let ctx = PathfinderCtx::new(world_lock);
            let successors = |pos: BlockPos| {
                let mut edges = Vec::with_capacity(16);
                successors_fn(&mut edges, &ctx, pos);
                edges
            };

            let mut attempt_number = 0;

            let mut path;
            let mut is_partial: bool;

            'calculate: loop {
                let start_time = std::time::Instant::now();
                let astar::Path { movements, partial } = a_star(
                    start,
                    |n| goal.heuristic(n),
                    successors,
                    |n| goal.success(n),
                    Duration::from_secs(if attempt_number == 0 { 1 } else { 5 }),
                );
                let end_time = std::time::Instant::now();
                debug!("partial: {partial:?}");
                let duration = end_time - start_time;
                if partial {
                    info!("Pathfinder took {duration:?} (timed out)");
                } else {
                    info!("Pathfinder took {duration:?}");
                }

                debug!("Path:");
                for movement in &movements {
                    debug!("  {:?}", movement.target);
                }

                path = movements.into_iter().collect::<VecDeque<_>>();
                is_partial = partial;

                if path.is_empty() && partial {
                    if attempt_number == 0 {
                        debug!("this path is empty, retrying with a higher timeout");
                        attempt_number += 1;
                        continue 'calculate;
                    } else {
                        debug!("this path is empty, giving up");
                        break 'calculate;
                    }
                }
                break;
            }

            Some(PathFoundEvent {
                entity,
                start,
                path: Some(path),
                is_partial,
                successors_fn,
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
fn path_found_listener(
    mut events: EventReader<PathFoundEvent>,
    mut query: Query<(&mut Pathfinder, &InstanceName)>,
    instance_container: Res<InstanceContainer>,
) {
    for event in events.iter() {
        let (mut pathfinder, instance_name) = query
            .get_mut(event.entity)
            .expect("Path found for an entity that doesn't have a pathfinder");
        if let Some(path) = &event.path {
            if pathfinder.path.is_empty() {
                pathfinder.path = path.to_owned();
                debug!("set path to {:?}", path.iter().take(10).collect::<Vec<_>>());
                pathfinder.last_reached_node = Some(event.start);
            } else {
                let mut new_path = VecDeque::new();

                // combine the old and new paths if the first node of the new path is a
                // successor of the last node of the old path
                if let Some(first_node) = path.front() {
                    if let Some(last_node) = pathfinder.path.back() {
                        let world_lock = instance_container.get(instance_name).expect(
                            "Entity tried to pathfind but the entity isn't in a valid world",
                        );
                        let successors_fn: moves::SuccessorsFn = event.successors_fn;
                        let ctx = PathfinderCtx::new(world_lock);
                        let successors = |pos: BlockPos| {
                            let mut edges = Vec::with_capacity(16);
                            successors_fn(&mut edges, &ctx, pos);
                            edges
                        };

                        if successors(last_node.target)
                            .iter()
                            .any(|edge| edge.movement.target == first_node.target)
                        {
                            debug!("combining old and new paths");
                            debug!("old path: {:?}", pathfinder.path.iter().collect::<Vec<_>>());
                            debug!("new path: {:?}", path.iter().take(10).collect::<Vec<_>>());
                            new_path.extend(pathfinder.path.iter().cloned());
                        }
                    }
                }

                new_path.extend(path.to_owned());

                debug!(
                    "set queued path to {:?}",
                    new_path.iter().take(10).collect::<Vec<_>>()
                );
                pathfinder.queued_path = Some(new_path);
            }
            pathfinder.last_node_reached_at = Some(Instant::now());
        } else {
            error!("No path found");
            pathfinder.path.clear();
            pathfinder.queued_path = None;
        }
        pathfinder.is_calculating = false;
        pathfinder.is_path_partial = event.is_partial;
    }
}

fn tick_execute_path(
    mut query: Query<(Entity, &mut Pathfinder, &Position, &Physics, &InstanceName)>,
    mut look_at_events: EventWriter<LookAtEvent>,
    mut sprint_events: EventWriter<StartSprintEvent>,
    mut walk_events: EventWriter<StartWalkEvent>,
    mut jump_events: EventWriter<JumpEvent>,
    mut goto_events: EventWriter<GotoEvent>,
    instance_container: Res<InstanceContainer>,
) {
    for (entity, mut pathfinder, position, physics, instance_name) in &mut query {
        if pathfinder.goal.is_none() {
            // no goal, no pathfinding
            continue;
        }

        let successors_fn: moves::SuccessorsFn = pathfinder
            .successors_fn
            .expect("pathfinder.successors_fn should be Some if the goal is Some");

        let world_lock = instance_container
            .get(instance_name)
            .expect("Entity tried to pathfind but the entity isn't in a valid world");

        if !pathfinder.is_calculating {
            // timeout check
            if let Some(last_node_reached_at) = pathfinder.last_node_reached_at {
                if last_node_reached_at.elapsed() > Duration::from_secs(2) {
                    warn!("pathfinder timeout");
                    pathfinder.path.clear();
                    // set partial to true to make sure that the recalculation happens
                    pathfinder.is_path_partial = true;
                }
            }
        }

        'skip: loop {
            // we check if the goal was reached *before* actually executing the movement so
            // we don't unnecessarily execute a movement when it wasn't necessary

            // see if we already reached any future nodes and can skip ahead
            for (i, movement) in pathfinder
                .path
                .clone()
                .into_iter()
                .enumerate()
                .take(10)
                .rev()
            {
                let is_reached_ctx = IsReachedCtx {
                    target: movement.target,
                    start: pathfinder.last_reached_node.expect(
                        "pathfinder.last_node_reached_at should always be present if there's a path",
                    ),
                    position: **position,
                    physics,
                };
                let extra_strict_if_last = if i == pathfinder.path.len() - 1 {
                    physics.on_ground && BlockPos::from(position) == movement.target
                } else {
                    true
                };
                if (movement.data.is_reached)(is_reached_ctx) && extra_strict_if_last {
                    pathfinder.path = pathfinder.path.split_off(i + 1);
                    pathfinder.last_reached_node = Some(movement.target);
                    pathfinder.last_node_reached_at = Some(Instant::now());

                    if let Some(new_path) = pathfinder.queued_path.take() {
                        debug!(
                            "swapped path to {:?}",
                            new_path.iter().take(10).collect::<Vec<_>>()
                        );
                        pathfinder.path = new_path;

                        if pathfinder.path.is_empty() {
                            info!("the path we just swapped to was empty, so reached end of path");
                            walk_events.send(StartWalkEvent {
                                entity,
                                direction: WalkDirection::None,
                            });
                            break;
                        }

                        // run the function again since we just swapped
                        continue 'skip;
                    }

                    if pathfinder.path.is_empty() {
                        debug!("pathfinder path is now empty");
                        walk_events.send(StartWalkEvent {
                            entity,
                            direction: WalkDirection::None,
                        });
                        if let Some(goal) = pathfinder.goal.clone() {
                            if goal.success(movement.target) {
                                info!("goal was reached!");
                                pathfinder.goal = None;
                                pathfinder.successors_fn = None;
                            }
                        }
                    }

                    break;
                }
            }
            break;
        }

        if let Some(movement) = pathfinder.path.front() {
            let ctx = ExecuteCtx {
                entity,
                target: movement.target,
                position: **position,
                start: pathfinder.last_reached_node.expect(
                    "pathfinder.last_reached_node should always be present if there's a path",
                ),
                physics,
                look_at_events: &mut look_at_events,
                sprint_events: &mut sprint_events,
                walk_events: &mut walk_events,
                jump_events: &mut jump_events,
            };
            trace!("executing move");
            (movement.data.execute)(ctx);
        }

        {
            // obstruction check (the path we're executing isn't possible anymore)
            let ctx = PathfinderCtx::new(world_lock);
            let successors = |pos: BlockPos| {
                let mut edges = Vec::with_capacity(16);
                successors_fn(&mut edges, &ctx, pos);
                edges
            };

            if let Some(last_reached_node) = pathfinder.last_reached_node {
                if let Some(obstructed_index) =
                    check_path_obstructed(last_reached_node, &pathfinder.path, successors)
                {
                    warn!("path obstructed at index {obstructed_index} (starting at {last_reached_node:?}, path: {:?})", pathfinder.path);
                    pathfinder.path.truncate(obstructed_index);
                }
            }
        }

        {
            // start recalculating if the path ends soon
            if pathfinder.path.len() < 5 && !pathfinder.is_calculating && pathfinder.is_path_partial
            {
                if let Some(goal) = pathfinder.goal.as_ref().cloned() {
                    debug!("Recalculating path because it ends soon");
                    goto_events.send(GotoEvent {
                        entity,
                        goal,
                        successors_fn,
                    });
                    pathfinder.is_calculating = true;

                    if pathfinder.path.is_empty() {
                        if let Some(new_path) = pathfinder.queued_path.take() {
                            pathfinder.path = new_path;
                            if pathfinder.path.is_empty() {
                                info!(
                                    "the path we just swapped to was empty, so reached end of path"
                                );
                                walk_events.send(StartWalkEvent {
                                    entity,
                                    direction: WalkDirection::None,
                                });
                                break;
                            }
                        } else {
                            walk_events.send(StartWalkEvent {
                                entity,
                                direction: WalkDirection::None,
                            });
                        }
                    }
                }
            }
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
}

/// Checks whether the path has been obstructed, and returns Some(index) if it
/// has been. The index is of the first obstructed node.
fn check_path_obstructed<SuccessorsFn>(
    mut current_position: BlockPos,
    path: &VecDeque<astar::Movement<BlockPos, moves::MoveData>>,
    successors_fn: SuccessorsFn,
) -> Option<usize>
where
    SuccessorsFn: Fn(BlockPos) -> Vec<astar::Edge<BlockPos, moves::MoveData>>,
{
    for (i, movement) in path.iter().enumerate() {
        let mut found_obstruction = false;
        for edge in successors_fn(current_position) {
            if edge.movement.target == movement.target {
                current_position = movement.target;
                found_obstruction = false;
                break;
            } else {
                found_obstruction = true;
            }
        }
        if found_obstruction {
            return Some(i);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, sync::Arc};

    use azalea_core::position::{BlockPos, ChunkPos, Vec3};
    use azalea_world::{Chunk, ChunkStorage, PartialChunkStorage};

    use super::{
        goals::BlockPosGoal,
        moves,
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

        // you can uncomment this while debugging tests to get trace logs
        // simulation.app.add_plugins(bevy_log::LogPlugin {
        //     level: bevy_log::Level::TRACE,
        //     filter: "".to_string(),
        // });

        simulation.app.world.send_event(GotoEvent {
            entity: simulation.entity,
            goal: Arc::new(BlockPosGoal(end_pos)),
            successors_fn: moves::default_move,
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
        for _ in 0..30 {
            simulation.tick();
        }
        assert_eq!(
            BlockPos::from(simulation.position()),
            BlockPos::new(2, 71, 2)
        );
    }

    #[test]
    fn test_jump_with_sideways_momentum() {
        let mut partial_chunks = PartialChunkStorage::default();
        let mut simulation = setup_simulation(
            &mut partial_chunks,
            BlockPos::new(0, 71, 3),
            BlockPos::new(5, 76, 0),
            vec![
                BlockPos::new(0, 70, 3),
                BlockPos::new(0, 70, 2),
                BlockPos::new(0, 70, 1),
                BlockPos::new(0, 70, 0),
                BlockPos::new(1, 71, 0),
                BlockPos::new(2, 72, 0),
                BlockPos::new(3, 73, 0),
                BlockPos::new(4, 74, 0),
                BlockPos::new(5, 75, 0),
            ],
        );
        for _ in 0..120 {
            simulation.tick();
        }
        assert_eq!(
            BlockPos::from(simulation.position()),
            BlockPos::new(5, 76, 0)
        );
    }

    #[test]
    fn test_parkour_2_block_gap() {
        let mut partial_chunks = PartialChunkStorage::default();
        let mut simulation = setup_simulation(
            &mut partial_chunks,
            BlockPos::new(0, 71, 0),
            BlockPos::new(0, 71, 3),
            vec![BlockPos::new(0, 70, 0), BlockPos::new(0, 70, 3)],
        );
        for _ in 0..40 {
            simulation.tick();
        }
        assert_eq!(
            BlockPos::from(simulation.position()),
            BlockPos::new(0, 71, 3)
        );
    }

    #[test]
    fn test_descend_and_parkour_2_block_gap() {
        let mut partial_chunks = PartialChunkStorage::default();
        let mut simulation = setup_simulation(
            &mut partial_chunks,
            BlockPos::new(0, 71, 0),
            BlockPos::new(3, 67, 4),
            vec![
                BlockPos::new(0, 70, 0),
                BlockPos::new(0, 69, 1),
                BlockPos::new(0, 68, 2),
                BlockPos::new(0, 67, 3),
                BlockPos::new(0, 66, 4),
                BlockPos::new(3, 66, 4),
            ],
        );
        for _ in 0..100 {
            simulation.tick();
        }
        assert_eq!(
            BlockPos::from(simulation.position()),
            BlockPos::new(3, 67, 4)
        );
    }

    #[test]
    fn test_quickly_descend() {
        let mut partial_chunks = PartialChunkStorage::default();
        let mut simulation = setup_simulation(
            &mut partial_chunks,
            BlockPos::new(0, 71, 0),
            BlockPos::new(0, 68, 3),
            vec![
                BlockPos::new(0, 70, 0),
                BlockPos::new(0, 69, 1),
                BlockPos::new(0, 68, 2),
                BlockPos::new(0, 67, 3),
            ],
        );
        for _ in 0..60 {
            simulation.tick();
        }
        assert_eq!(
            BlockPos::from(simulation.position()),
            BlockPos::new(0, 68, 3)
        );
    }
}
