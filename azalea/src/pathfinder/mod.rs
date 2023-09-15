mod astar;
pub mod costs;
pub mod goals;
mod moves;
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
use azalea_client::movement::walk_listener;
use azalea_client::{StartSprintEvent, StartWalkEvent};
use azalea_core::BlockPos;
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
    pub queued_path: Option<VecDeque<astar::Movement<BlockPos, moves::MoveData>>>,
    pub is_path_partial: bool,

    pub last_reached_node: Option<BlockPos>,
    pub last_node_reached_at: Option<Instant>,
    pub goal: Option<Arc<dyn Goal + Send + Sync>>,
    pub is_calculating: bool,
}
#[derive(Event)]
pub struct GotoEvent {
    pub entity: Entity,
    pub goal: Arc<dyn Goal + Send + Sync>,
}
#[derive(Event)]
pub struct PathFoundEvent {
    pub entity: Entity,
    pub start: BlockPos,
    pub path: Option<VecDeque<astar::Movement<BlockPos, moves::MoveData>>>,
    pub is_partial: bool,
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

#[derive(Component)]
pub struct ComputePath(Task<Option<PathFoundEvent>>);

fn goto_listener(
    mut commands: Commands,
    mut events: EventReader<GotoEvent>,
    mut query: Query<(&mut Pathfinder, &Position, &InstanceName)>,
    instance_container: Res<InstanceContainer>,
) {
    let successors_fn = moves::basic::basic_move;

    let thread_pool = AsyncComputeTaskPool::get();

    for event in events.iter() {
        let (mut pathfinder, position, instance_name) = query
            .get_mut(event.entity)
            .expect("Called goto on an entity that's not in the world");

        // we store the goal so it can be recalculated later if necessary
        pathfinder.goal = Some(event.goal.clone());
        pathfinder.is_calculating = true;

        let start = BlockPos::from(position);

        let world_lock = instance_container
            .get(instance_name)
            .expect("Entity tried to pathfind but the entity isn't in a valid world");
        let end = event.goal.goal_node();

        let goal = event.goal.clone();
        let entity = event.entity;

        let task = thread_pool.spawn(async move {
            debug!("start: {start:?}, end: {end:?}");

            let successors = |pos: BlockPos| {
                let world = world_lock.read();
                successors_fn(&world, pos)
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
                debug!("time: {:?}", end_time - start_time);

                info!("Path:");
                for movement in &movements {
                    info!("  {:?}", movement.target);
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
            if pathfinder.path.is_empty() {
                pathfinder.path = path.to_owned();
            } else {
                pathfinder.queued_path = Some(path.to_owned());
            }
            pathfinder.last_reached_node = Some(event.start);
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
    let successors_fn = moves::basic::basic_move;

    for (entity, mut pathfinder, position, physics, instance_name) in &mut query {
        if pathfinder.goal.is_none() {
            // no goal, no pathfinding
            continue;
        }

        let world_lock = instance_container
            .get(instance_name)
            .expect("Entity tried to pathfind but the entity isn't in a valid world");

        if !pathfinder.is_calculating {
            // timeout check
            if let Some(last_node_reached_at) = pathfinder.last_node_reached_at {
                if last_node_reached_at.elapsed() > Duration::from_secs(2) {
                    warn!("pathfinder timeout");
                    pathfinder.path.clear();
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
                if is_goal_reached(movement.target, position, physics) {
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
                            }
                        }
                    }
                }
            }
            break;
        }

        if let Some(movement) = pathfinder.path.front() {
            let ctx = ExecuteCtx {
                entity,
                target: movement.target,
                position: **position,
                look_at_events: &mut look_at_events,
                sprint_events: &mut sprint_events,
                walk_events: &mut walk_events,
                jump_events: &mut jump_events,
            };
            trace!("executing move");
            (movement.data.execute)(ctx);
        }

        {
            // obstruction check
            let successors = |pos: BlockPos| {
                let world = world_lock.read();
                successors_fn(&world, pos)
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
                    goto_events.send(GotoEvent { entity, goal });

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
    // TODO: this should be removed and mtdstarlite should stop depending on
    // being given a goal node
    fn goal_node(&self) -> BlockPos;
}

/// Returns whether the entity is at the node and should start going to the
/// next node.
#[must_use]
pub fn is_goal_reached(goal_pos: BlockPos, current_pos: &Position, physics: &Physics) -> bool {
    BlockPos::from(current_pos) == goal_pos && physics.on_ground
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

    use azalea_core::{BlockPos, ChunkPos, Vec3};
    use azalea_world::{Chunk, ChunkStorage, PartialChunkStorage};
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
        // simulation.app.add_plugins(bevy_log::LogPlugin {
        //     level: bevy_log::Level::TRACE,
        //     filter: "".to_string(),
        // });

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
        for i in 0..120 {
            simulation.tick();
            info!("-- tick #{i} --")
        }
        assert_eq!(
            BlockPos::from(simulation.position()),
            BlockPos::new(5, 76, 0)
        );
    }
}
