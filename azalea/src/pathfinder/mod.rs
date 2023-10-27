//! A pathfinding plugin to make bots navigate the world. A lot of this code is
//! based on [Baritone](https://github.com/cabaletta/baritone).

pub mod astar;
pub mod costs;
pub mod goals;
pub mod mining;
pub mod moves;
pub mod simulation;
pub mod world;

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
use crate::pathfinder::world::CachedWorld;
use azalea_client::chat::SendChatEvent;
use azalea_client::inventory::{InventoryComponent, InventorySet};
use azalea_client::movement::MoveEventsSet;
use azalea_client::{StartSprintEvent, StartWalkEvent};
use azalea_core::position::{BlockPos, Vec3};
use azalea_entity::metadata::Player;
use azalea_entity::LocalEntity;
use azalea_entity::{Physics, Position};
use azalea_physics::PhysicsSet;
use azalea_world::{InstanceContainer, InstanceName};
use bevy_app::{FixedUpdate, PreUpdate, Update};
use bevy_ecs::event::Events;
use bevy_ecs::prelude::Event;
use bevy_ecs::query::Changed;
use bevy_ecs::schedule::IntoSystemConfigs;
use bevy_ecs::system::{Local, ResMut};
use bevy_tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future;
use log::{debug, error, info, trace, warn};
use std::collections::VecDeque;
use std::sync::atomic::{self, AtomicUsize};
use std::sync::Arc;
use std::time::{Duration, Instant};

use self::mining::MiningCache;
use self::moves::{ExecuteCtx, IsReachedCtx, SuccessorsFn};

#[derive(Clone, Default)]
pub struct PathfinderPlugin;
impl Plugin for PathfinderPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GotoEvent>()
            .add_event::<PathFoundEvent>()
            .add_event::<StopPathfindingEvent>()
            .add_systems(
                FixedUpdate,
                // putting systems in the FixedUpdate schedule makes them run every Minecraft tick
                // (every 50 milliseconds).
                (
                    timeout_movement,
                    check_node_reached,
                    tick_execute_path,
                    check_for_path_obstruction,
                    debug_render_path_with_particles,
                    recalculate_near_end_of_path,
                    recalculate_if_has_goal_but_no_path,
                )
                    .chain()
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
                    stop_pathfinding_on_instance_change,
                    handle_stop_pathfinding_event,
                )
                    .chain()
                    .before(MoveEventsSet)
                    .before(InventorySet),
            );
    }
}

/// A component that makes this client able to pathfind.
#[derive(Component, Default)]
pub struct Pathfinder {
    pub goal: Option<Arc<dyn Goal + Send + Sync>>,
    pub successors_fn: Option<SuccessorsFn>,
    pub is_calculating: bool,

    pub goto_id: Arc<AtomicUsize>,
}

/// A component that's present on clients that are actively following a
/// pathfinder path.
#[derive(Component)]
pub struct ExecutingPath {
    pub path: VecDeque<astar::Movement<BlockPos, moves::MoveData>>,
    pub queued_path: Option<VecDeque<astar::Movement<BlockPos, moves::MoveData>>>,
    pub last_reached_node: BlockPos,
    pub last_node_reached_at: Instant,
    pub is_path_partial: bool,
}

#[derive(Event)]
pub struct GotoEvent {
    pub entity: Entity,
    pub goal: Arc<dyn Goal + Send + Sync>,
    /// The function that's used for checking what moves are possible. Usually
    /// `pathfinder::moves::default_move`
    pub successors_fn: SuccessorsFn,
}
#[derive(Event, Clone)]
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
    fn stop_pathfinding(&self);
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

    fn stop_pathfinding(&self) {
        self.ecs.lock().send_event(StopPathfindingEvent {
            entity: self.entity,
            force: false,
        });
    }
}

#[derive(Component)]
pub struct ComputePath(Task<Option<PathFoundEvent>>);

fn goto_listener(
    mut commands: Commands,
    mut events: EventReader<GotoEvent>,
    mut query: Query<(
        &mut Pathfinder,
        Option<&ExecutingPath>,
        &Position,
        &InstanceName,
        &InventoryComponent,
    )>,
    instance_container: Res<InstanceContainer>,
) {
    let thread_pool = AsyncComputeTaskPool::get();

    for event in events.iter() {
        let (mut pathfinder, executing_path, position, instance_name, inventory) = query
            .get_mut(event.entity)
            .expect("Called goto on an entity that's not in the world");

        // we store the goal so it can be recalculated later if necessary
        pathfinder.goal = Some(event.goal.clone());
        pathfinder.successors_fn = Some(event.successors_fn);
        pathfinder.is_calculating = true;

        let start = if let Some(executing_path) = executing_path
            && let Some(final_node) = executing_path.path.back() {
            // if we're currently pathfinding and got a goto event, start a little ahead
            executing_path
                .path
                .get(20)
                .unwrap_or(final_node)
                .target
        } else {
            BlockPos::from(position)
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

        let goto_id_atomic = pathfinder.goto_id.clone();
        let goto_id = goto_id_atomic.fetch_add(1, atomic::Ordering::Relaxed) + 1;
        let mining_cache = MiningCache::new(inventory.inventory_menu.clone());

        let task = thread_pool.spawn(async move {
            debug!("start: {start:?}");

            let cached_world = CachedWorld::new(world_lock);
            let successors = |pos: BlockPos| {
                call_successors_fn(&cached_world, &mining_cache, successors_fn, pos)
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
                    info!("Pathfinder took {duration:?} (incomplete path)");
                    // wait a bit so it's not a busy loop
                    std::thread::sleep(Duration::from_millis(100));
                } else {
                    info!("Pathfinder took {duration:?}");
                }

                debug!("Path:");
                for movement in &movements {
                    debug!("  {:?}", movement.target);
                }

                path = movements.into_iter().collect::<VecDeque<_>>();
                is_partial = partial;

                let goto_id_now = goto_id_atomic.load(atomic::Ordering::Relaxed);
                if goto_id != goto_id_now {
                    // we must've done another goto while calculating this path, so throw it away
                    warn!("finished calculating a path, but it's outdated");
                    return None;
                }

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
    mut query: Query<(
        &mut Pathfinder,
        Option<&mut ExecutingPath>,
        &InstanceName,
        &InventoryComponent,
    )>,
    instance_container: Res<InstanceContainer>,
    mut commands: Commands,
) {
    for event in events.iter() {
        let (mut pathfinder, executing_path, instance_name, inventory) = query
            .get_mut(event.entity)
            .expect("Path found for an entity that doesn't have a pathfinder");
        if let Some(path) = &event.path {
            if let Some(mut executing_path) = executing_path {
                let mut new_path = VecDeque::new();

                // combine the old and new paths if the first node of the new path is a
                // successor of the last node of the old path
                if let Some(last_node_of_current_path) = executing_path.path.back() {
                    let world_lock = instance_container
                        .get(instance_name)
                        .expect("Entity tried to pathfind but the entity isn't in a valid world");
                    let successors_fn: moves::SuccessorsFn = event.successors_fn;
                    let cached_world = CachedWorld::new(world_lock);
                    let mining_cache = MiningCache::new(inventory.inventory_menu.clone());
                    let successors = |pos: BlockPos| {
                        call_successors_fn(&cached_world, &mining_cache, successors_fn, pos)
                    };

                    if let Some(first_node_of_new_path) = path.front() {
                        if successors(last_node_of_current_path.target)
                            .iter()
                            .any(|edge| edge.movement.target == first_node_of_new_path.target)
                        {
                            debug!("combining old and new paths");
                            debug!(
                                "old path: {:?}",
                                executing_path.path.iter().collect::<Vec<_>>()
                            );
                            debug!("new path: {:?}", path.iter().take(10).collect::<Vec<_>>());
                            new_path.extend(executing_path.path.iter().cloned());
                        }
                    } else {
                        new_path.extend(executing_path.path.iter().cloned());
                    }
                }

                new_path.extend(path.to_owned());

                debug!(
                    "set queued path to {:?}",
                    new_path.iter().take(10).collect::<Vec<_>>()
                );
                executing_path.queued_path = Some(new_path);
                executing_path.is_path_partial = event.is_partial;
            } else if path.is_empty() {
                debug!("calculated path is empty, so didn't add ExecutingPath");
            } else {
                commands.entity(event.entity).insert(ExecutingPath {
                    path: path.to_owned(),
                    queued_path: None,
                    last_reached_node: event.start,
                    last_node_reached_at: Instant::now(),
                    is_path_partial: event.is_partial,
                });
                debug!("set path to {:?}", path.iter().take(10).collect::<Vec<_>>());
                debug!("partial: {}", event.is_partial);
            }
        } else {
            error!("No path found");
            if let Some(mut executing_path) = executing_path {
                // set the queued path so we don't stop in the middle of a move
                executing_path.queued_path = Some(VecDeque::new());
            } else {
                // wasn't executing a path, don't need to do anything
            }
        }
        pathfinder.is_calculating = false;
    }
}

fn timeout_movement(mut query: Query<(&Pathfinder, &mut ExecutingPath, &Position)>) {
    for (pathfinder, mut executing_path, position) in &mut query {
        if executing_path.last_node_reached_at.elapsed() > Duration::from_secs(2)
            && !pathfinder.is_calculating
            && !executing_path.path.is_empty()
        {
            warn!("pathfinder timeout");
            // the path wasn't being followed anyways, so clearing it is fine
            executing_path.path.clear();
            executing_path.queued_path = None;
            executing_path.last_reached_node = BlockPos::from(position);
            // invalidate whatever calculation we were just doing, if any
            pathfinder.goto_id.fetch_add(1, atomic::Ordering::Relaxed);
            // set partial to true to make sure that a recalculation will happen
            executing_path.is_path_partial = true;

            // the path will get recalculated automatically because the path is
            // empty
        }
    }
}

fn check_node_reached(
    mut query: Query<(
        Entity,
        &mut Pathfinder,
        &mut ExecutingPath,
        &Position,
        &Physics,
    )>,
    mut walk_events: EventWriter<StartWalkEvent>,
    mut commands: Commands,
) {
    for (entity, mut pathfinder, mut executing_path, position, physics) in &mut query {
        'skip: loop {
            // we check if the goal was reached *before* actually executing the movement so
            // we don't unnecessarily execute a movement when it wasn't necessary

            // see if we already reached any future nodes and can skip ahead
            for (i, movement) in executing_path
                .path
                .clone()
                .into_iter()
                .enumerate()
                .take(20)
                .rev()
            {
                let is_reached_ctx = IsReachedCtx {
                    target: movement.target,
                    start: executing_path.last_reached_node,
                    position: **position,
                    physics,
                };
                let extra_strict_if_last = if i == executing_path.path.len() - 1 {
                    let x_difference_from_center = position.x - (movement.target.x as f64 + 0.5);
                    let z_difference_from_center = position.z - (movement.target.z as f64 + 0.5);
                    // this is to make sure we don't fall off immediately after finishing the path
                    physics.on_ground
                    && BlockPos::from(position) == movement.target
                    // adding the delta like this isn't a perfect solution but it helps to make
                    // sure we don't keep going if our delta is high
                    && (x_difference_from_center + physics.velocity.x).abs() < 0.2
                    && (z_difference_from_center + physics.velocity.z).abs() < 0.2
                } else {
                    true
                };
                if (movement.data.is_reached)(is_reached_ctx) && extra_strict_if_last {
                    executing_path.path = executing_path.path.split_off(i + 1);
                    executing_path.last_reached_node = movement.target;
                    executing_path.last_node_reached_at = Instant::now();

                    if let Some(new_path) = executing_path.queued_path.take() {
                        debug!(
                            "swapped path to {:?}",
                            new_path.iter().take(10).collect::<Vec<_>>()
                        );
                        executing_path.path = new_path;

                        if executing_path.path.is_empty() {
                            info!("the path we just swapped to was empty, so reached end of path");
                            walk_events.send(StartWalkEvent {
                                entity,
                                direction: WalkDirection::None,
                            });
                            commands.entity(entity).remove::<ExecutingPath>();
                            break;
                        }

                        // run the function again since we just swapped
                        continue 'skip;
                    }

                    if executing_path.path.is_empty() {
                        debug!("pathfinder path is now empty");
                        walk_events.send(StartWalkEvent {
                            entity,
                            direction: WalkDirection::None,
                        });
                        commands.entity(entity).remove::<ExecutingPath>();
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
    }
}

fn check_for_path_obstruction(
    mut query: Query<(
        &Pathfinder,
        &mut ExecutingPath,
        &InstanceName,
        &InventoryComponent,
    )>,
    instance_container: Res<InstanceContainer>,
) {
    for (pathfinder, mut executing_path, instance_name, inventory) in &mut query {
        let Some(successors_fn) = pathfinder.successors_fn else {
            continue;
        };

        let world_lock = instance_container
            .get(instance_name)
            .expect("Entity tried to pathfind but the entity isn't in a valid world");

        // obstruction check (the path we're executing isn't possible anymore)
        let cached_world = CachedWorld::new(world_lock);
        let mining_cache = MiningCache::new(inventory.inventory_menu.clone());
        let successors =
            |pos: BlockPos| call_successors_fn(&cached_world, &mining_cache, successors_fn, pos);

        if let Some(obstructed_index) = check_path_obstructed(
            executing_path.last_reached_node,
            &executing_path.path,
            successors,
        ) {
            warn!(
                "path obstructed at index {obstructed_index} (starting at {:?}, path: {:?})",
                executing_path.last_reached_node, executing_path.path
            );
            executing_path.path.truncate(obstructed_index);
            executing_path.is_path_partial = true;
        }
    }
}

fn recalculate_near_end_of_path(
    mut query: Query<(Entity, &mut Pathfinder, &mut ExecutingPath)>,
    mut walk_events: EventWriter<StartWalkEvent>,
    mut goto_events: EventWriter<GotoEvent>,
    mut commands: Commands,
) {
    for (entity, mut pathfinder, mut executing_path) in &mut query {
        let Some(successors_fn) = pathfinder.successors_fn else {
            continue;
        };

        // start recalculating if the path ends soon
        if (executing_path.path.len() == 20 || executing_path.path.len() < 5)
            && !pathfinder.is_calculating
            && executing_path.is_path_partial
        {
            if let Some(goal) = pathfinder.goal.as_ref().cloned() {
                debug!("Recalculating path because it ends soon");
                debug!(
                    "recalculate_near_end_of_path executing_path.is_path_partial: {}",
                    executing_path.is_path_partial
                );
                goto_events.send(GotoEvent {
                    entity,
                    goal,
                    successors_fn,
                });
                pathfinder.is_calculating = true;

                if executing_path.path.is_empty() {
                    if let Some(new_path) = executing_path.queued_path.take() {
                        executing_path.path = new_path;
                        if executing_path.path.is_empty() {
                            info!("the path we just swapped to was empty, so reached end of path");
                            walk_events.send(StartWalkEvent {
                                entity,
                                direction: WalkDirection::None,
                            });
                            commands.entity(entity).remove::<ExecutingPath>();
                            break;
                        }
                    } else {
                        walk_events.send(StartWalkEvent {
                            entity,
                            direction: WalkDirection::None,
                        });
                        commands.entity(entity).remove::<ExecutingPath>();
                    }
                }
            } else if executing_path.path.is_empty() {
                // idk when this can happen but stop moving just in case
                walk_events.send(StartWalkEvent {
                    entity,
                    direction: WalkDirection::None,
                });
            }
        }
    }
}

fn tick_execute_path(
    mut query: Query<(Entity, &mut ExecutingPath, &Position, &Physics)>,
    mut look_at_events: EventWriter<LookAtEvent>,
    mut sprint_events: EventWriter<StartSprintEvent>,
    mut walk_events: EventWriter<StartWalkEvent>,
    mut jump_events: EventWriter<JumpEvent>,
) {
    for (entity, executing_path, position, physics) in &mut query {
        if let Some(movement) = executing_path.path.front() {
            let ctx = ExecuteCtx {
                entity,
                target: movement.target,
                position: **position,
                start: executing_path.last_reached_node,
                physics,
                look_at_events: &mut look_at_events,
                sprint_events: &mut sprint_events,
                walk_events: &mut walk_events,
                jump_events: &mut jump_events,
            };
            trace!("executing move");
            (movement.data.execute)(ctx);
        }
    }
}

fn recalculate_if_has_goal_but_no_path(
    mut query: Query<(Entity, &mut Pathfinder), Without<ExecutingPath>>,
    mut goto_events: EventWriter<GotoEvent>,
) {
    for (entity, mut pathfinder) in &mut query {
        if pathfinder.goal.is_some() && !pathfinder.is_calculating {
            if let Some(goal) = pathfinder.goal.as_ref().cloned() {
                debug!("Recalculating path because it has a goal but no ExecutingPath");
                goto_events.send(GotoEvent {
                    entity,
                    goal,
                    successors_fn: pathfinder.successors_fn.unwrap(),
                });
                pathfinder.is_calculating = true;
            }
        }
    }
}

#[derive(Event)]
pub struct StopPathfindingEvent {
    pub entity: Entity,
    /// If false, then let the current movement finish before stopping. If true,
    /// then stop moving immediately. This might cause the bot to fall if it was
    /// in the middle of parkouring.
    pub force: bool,
}

fn handle_stop_pathfinding_event(
    mut events: EventReader<StopPathfindingEvent>,
    mut query: Query<(&mut Pathfinder, &mut ExecutingPath)>,
    mut walk_events: EventWriter<StartWalkEvent>,
    mut commands: Commands,
) {
    for event in events.iter() {
        let Ok((mut pathfinder, mut executing_path)) = query.get_mut(event.entity) else {
            continue;
        };
        pathfinder.goal = None;
        if event.force {
            executing_path.path.clear();
            executing_path.queued_path = None;
            walk_events.send(StartWalkEvent {
                entity: event.entity,
                direction: WalkDirection::None,
            });
            commands.entity(event.entity).remove::<ExecutingPath>();
        } else {
            executing_path.queued_path = Some(VecDeque::new());
        }
    }
}

fn stop_pathfinding_on_instance_change(
    mut query: Query<(Entity, &mut ExecutingPath), Changed<InstanceName>>,
    mut stop_pathfinding_events: EventWriter<StopPathfindingEvent>,
) {
    for (entity, mut executing_path) in &mut query {
        if !executing_path.path.is_empty() {
            debug!("instance changed, clearing path");
            executing_path.path.clear();
            stop_pathfinding_events.send(StopPathfindingEvent {
                entity,
                force: true,
            });
        }
    }
}

/// A component that makes bots run /particle commands while pathfinding to show
/// where they're going. This requires the bots to have server operator
/// permissions, and it'll make them spam *a lot* of commands.
///
/// ```
/// # use azalea::prelude::*;
/// # use azalea::pathfinder::PathfinderDebugParticles;
/// # #[derive(Component, Clone, Default)]
/// # pub struct State;
///
/// async fn handle(mut bot: Client, event: azalea::Event, state: State) -> anyhow::Result<()> {
///     match event {
///         azalea::Event::Init => {
///             bot.ecs
///                 .lock()
///                 .entity_mut(bot.entity)
///                 .insert(PathfinderDebugParticles);
///         }
///         _ => {}
///     }
///     Ok(())
/// }
/// ```
#[derive(Component)]
pub struct PathfinderDebugParticles;

fn debug_render_path_with_particles(
    mut query: Query<(Entity, &ExecutingPath), With<PathfinderDebugParticles>>,
    // chat_events is Option because the tests don't have SendChatEvent
    // and we have to use ResMut<Events> because bevy doesn't support Option<EventWriter>
    chat_events: Option<ResMut<Events<SendChatEvent>>>,
    mut tick_count: Local<usize>,
) {
    let Some(mut chat_events) = chat_events else {
        return;
    };
    if *tick_count >= 2 {
        *tick_count = 0;
    } else {
        *tick_count += 1;
        return;
    }
    for (entity, executing_path) in &mut query {
        if executing_path.path.is_empty() {
            continue;
        }

        let mut start = executing_path.last_reached_node;
        for (i, movement) in executing_path.path.iter().enumerate() {
            // /particle dust 0 1 1 1 ~ ~ ~ 0 0 0.2 0 100

            let end = movement.target;

            let start_vec3 = start.center();
            let end_vec3 = end.center();

            let step_count = (start_vec3.distance_to_sqr(&end_vec3).sqrt() * 4.0) as usize;

            let (r, g, b): (f64, f64, f64) = if i == 0 { (0., 1., 0.) } else { (0., 1., 1.) };

            // interpolate between the start and end positions
            for i in 0..step_count {
                let percent = i as f64 / step_count as f64;
                let pos = Vec3 {
                    x: start_vec3.x + (end_vec3.x - start_vec3.x) * percent,
                    y: start_vec3.y + (end_vec3.y - start_vec3.y) * percent,
                    z: start_vec3.z + (end_vec3.z - start_vec3.z) * percent,
                };
                let particle_command = format!(
                "/particle dust {r} {g} {b} {size} {start_x} {start_y} {start_z} {delta_x} {delta_y} {delta_z} 0 {count}",
                size = 1,
                start_x = pos.x,
                start_y = pos.y,
                start_z = pos.z,
                delta_x = 0,
                delta_y = 0,
                delta_z = 0,
                count = 1
            );
                chat_events.send(SendChatEvent {
                    entity,
                    content: particle_command,
                });
            }

            start = movement.target;
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

pub fn call_successors_fn(
    cached_world: &CachedWorld,
    mining_cache: &MiningCache,
    successors_fn: SuccessorsFn,
    pos: BlockPos,
) -> Vec<astar::Edge<BlockPos, moves::MoveData>> {
    let mut edges = Vec::with_capacity(16);
    let mut ctx = PathfinderCtx {
        edges: &mut edges,
        world: cached_world,
        mining_cache,
    };
    successors_fn(&mut ctx, pos);
    edges
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
    fn test_small_descend_and_parkour_2_block_gap() {
        let mut partial_chunks = PartialChunkStorage::default();
        let mut simulation = setup_simulation(
            &mut partial_chunks,
            BlockPos::new(0, 71, 0),
            BlockPos::new(0, 70, 5),
            vec![
                BlockPos::new(0, 70, 0),
                BlockPos::new(0, 70, 1),
                BlockPos::new(0, 69, 2),
                BlockPos::new(0, 69, 5),
            ],
        );
        for _ in 0..40 {
            simulation.tick();
        }
        assert_eq!(
            BlockPos::from(simulation.position()),
            BlockPos::new(0, 70, 5)
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

    #[test]
    fn test_2_gap_ascend_thrice() {
        let mut partial_chunks = PartialChunkStorage::default();
        let mut simulation = setup_simulation(
            &mut partial_chunks,
            BlockPos::new(0, 71, 0),
            BlockPos::new(3, 74, 0),
            vec![
                BlockPos::new(0, 70, 0),
                BlockPos::new(0, 71, 3),
                BlockPos::new(3, 72, 3),
                BlockPos::new(3, 73, 0),
            ],
        );
        for _ in 0..60 {
            simulation.tick();
        }
        assert_eq!(
            BlockPos::from(simulation.position()),
            BlockPos::new(3, 74, 0)
        );
    }

    #[test]
    fn test_consecutive_3_gap_parkour() {
        let mut partial_chunks = PartialChunkStorage::default();
        let mut simulation = setup_simulation(
            &mut partial_chunks,
            BlockPos::new(0, 71, 0),
            BlockPos::new(4, 71, 12),
            vec![
                BlockPos::new(0, 70, 0),
                BlockPos::new(0, 70, 4),
                BlockPos::new(0, 70, 8),
                BlockPos::new(0, 70, 12),
                BlockPos::new(4, 70, 12),
            ],
        );
        for _ in 0..80 {
            simulation.tick();
        }
        assert_eq!(
            BlockPos::from(simulation.position()),
            BlockPos::new(4, 71, 12)
        );
    }
}
