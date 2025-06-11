//! A pathfinding plugin to make bots able to traverse the world.
//!
//! Much of this code is based on [Baritone](https://github.com/cabaletta/baritone).

pub mod astar;
pub mod costs;
pub mod custom_state;
pub mod debug;
pub mod goals;
pub mod mining;
pub mod moves;
pub mod rel_block_pos;
pub mod simulation;
pub mod world;

use std::{
    cmp,
    collections::VecDeque,
    ops::RangeInclusive,
    sync::{
        Arc,
        atomic::{self, AtomicUsize},
    },
    thread,
    time::{Duration, Instant},
};

use astar::{Edge, PathfinderTimeout};
use azalea_client::{
    StartSprintEvent, StartWalkEvent,
    inventory::{Inventory, InventorySet, SetSelectedHotbarSlotEvent},
    local_player::InstanceHolder,
    mining::{Mining, MiningSet, StartMiningBlockEvent},
    movement::MoveEventsSet,
};
use azalea_core::{position::BlockPos, tick::GameTick};
use azalea_entity::{LocalEntity, Physics, Position, metadata::Player};
use azalea_physics::PhysicsSet;
use azalea_world::{InstanceContainer, InstanceName};
use bevy_app::{PreUpdate, Update};
use bevy_ecs::prelude::*;
use bevy_tasks::{AsyncComputeTaskPool, Task};
use custom_state::{CustomPathfinderState, CustomPathfinderStateRef};
use futures_lite::future;
use goals::BlockPosGoal;
use parking_lot::RwLock;
use rel_block_pos::RelBlockPos;
use tokio::sync::broadcast::error::RecvError;
use tracing::{debug, error, info, trace, warn};

use self::{
    debug::debug_render_path_with_particles,
    goals::Goal,
    mining::MiningCache,
    moves::{ExecuteCtx, IsReachedCtx, SuccessorsFn},
};
use crate::{
    BotClientExt, WalkDirection,
    app::{App, Plugin},
    bot::{JumpEvent, LookAtEvent},
    ecs::{
        component::Component,
        entity::Entity,
        event::{EventReader, EventWriter},
        query::{With, Without},
        system::{Commands, Query, Res},
    },
    pathfinder::{astar::a_star, moves::PathfinderCtx, world::CachedWorld},
};

#[derive(Clone, Default)]
pub struct PathfinderPlugin;
impl Plugin for PathfinderPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GotoEvent>()
            .add_event::<PathFoundEvent>()
            .add_event::<StopPathfindingEvent>()
            .add_systems(
                // putting systems in the GameTick schedule makes them run every Minecraft tick
                // (every 50 milliseconds).
                GameTick,
                (
                    timeout_movement,
                    check_for_path_obstruction,
                    check_node_reached,
                    tick_execute_path,
                    debug_render_path_with_particles,
                    recalculate_near_end_of_path,
                    recalculate_if_has_goal_but_no_path,
                )
                    .chain()
                    .after(PhysicsSet)
                    .after(azalea_client::movement::send_position)
                    .after(MiningSet),
            )
            .add_systems(PreUpdate, add_default_pathfinder)
            .add_systems(
                Update,
                (
                    goto_listener,
                    handle_tasks,
                    stop_pathfinding_on_instance_change,
                    path_found_listener,
                    handle_stop_pathfinding_event,
                )
                    .chain()
                    .before(MoveEventsSet)
                    .before(InventorySet),
            );
    }
}

/// A component that makes this client able to pathfind.
#[derive(Component, Default, Clone)]
pub struct Pathfinder {
    pub goal: Option<Arc<dyn Goal>>,
    pub successors_fn: Option<SuccessorsFn>,
    pub is_calculating: bool,
    pub allow_mining: bool,

    pub min_timeout: Option<PathfinderTimeout>,
    pub max_timeout: Option<PathfinderTimeout>,

    pub goto_id: Arc<AtomicUsize>,
}

/// A component that's present on clients that are actively following a
/// pathfinder path.
#[derive(Component, Clone)]
pub struct ExecutingPath {
    pub path: VecDeque<astar::Edge<BlockPos, moves::MoveData>>,
    pub queued_path: Option<VecDeque<astar::Edge<BlockPos, moves::MoveData>>>,
    pub last_reached_node: BlockPos,
    pub last_node_reached_at: Instant,
    pub is_path_partial: bool,
}

/// Send this event to start pathfinding to the given goal.
///
/// Also see [`PathfinderClientExt::goto`].
///
/// This event is read by [`goto_listener`].
#[derive(Event)]
pub struct GotoEvent {
    /// The local bot entity that will do the pathfinding and execute the path.
    pub entity: Entity,
    pub goal: Arc<dyn Goal>,
    /// The function that's used for checking what moves are possible. Usually
    /// [`moves::default_move`].
    pub successors_fn: SuccessorsFn,

    /// Whether the bot is allowed to break blocks while pathfinding.
    pub allow_mining: bool,

    /// The minimum amount of time that should pass before the A* pathfinder
    /// function can return a timeout. It may take up to [`Self::max_timeout`]
    /// if it can't immediately find a usable path.
    ///
    /// A good default value for this is
    /// `PathfinderTimeout::Time(Duration::from_secs(1))`.
    ///
    /// Also see [`PathfinderTimeout::Nodes`]
    pub min_timeout: PathfinderTimeout,
    /// The absolute maximum amount of time that the pathfinder function can
    /// take to find a path. If it takes this long, it means no usable path was
    /// found (so it might be impossible).
    ///
    /// A good default value for this is
    /// `PathfinderTimeout::Time(Duration::from_secs(5))`.
    pub max_timeout: PathfinderTimeout,
}
#[derive(Event, Clone, Debug)]
pub struct PathFoundEvent {
    pub entity: Entity,
    pub start: BlockPos,
    pub path: Option<VecDeque<astar::Edge<BlockPos, moves::MoveData>>>,
    pub is_partial: bool,
    pub successors_fn: SuccessorsFn,
    pub allow_mining: bool,
}

#[allow(clippy::type_complexity)]
pub fn add_default_pathfinder(
    mut commands: Commands,
    mut query: Query<Entity, (Without<Pathfinder>, With<LocalEntity>, With<Player>)>,
) {
    for entity in &mut query {
        commands.entity(entity).insert(Pathfinder::default());
    }
}

pub trait PathfinderClientExt {
    fn goto(&self, goal: impl Goal + 'static) -> impl Future<Output = ()>;
    fn start_goto(&self, goal: impl Goal + 'static);
    fn start_goto_without_mining(&self, goal: impl Goal + 'static);
    fn stop_pathfinding(&self);
    fn wait_until_goto_target_reached(&self) -> impl Future<Output = ()>;
    fn is_goto_target_reached(&self) -> bool;
}

impl PathfinderClientExt for azalea_client::Client {
    /// Pathfind to the given goal and wait until either the target is reached
    /// or the pathfinding is canceled.
    ///
    /// You can use [`Self::start_goto`] instead if you don't want to wait.
    ///
    /// ```
    /// # use azalea::prelude::*;
    /// # use azalea::{BlockPos, pathfinder::goals::BlockPosGoal};
    /// # async fn example(bot: &Client) {
    /// bot.goto(BlockPosGoal(BlockPos::new(0, 70, 0))).await;
    /// # }
    /// ```
    async fn goto(&self, goal: impl Goal + 'static) {
        self.start_goto(goal);
        self.wait_until_goto_target_reached().await;
    }

    /// Start pathfinding to a given goal.
    ///
    /// ```
    /// # use azalea::prelude::*;
    /// # use azalea::{BlockPos, pathfinder::goals::BlockPosGoal};
    /// # fn example(bot: &Client) {
    /// bot.start_goto(BlockPosGoal(BlockPos::new(0, 70, 0)));
    /// # }
    /// ```
    fn start_goto(&self, goal: impl Goal + 'static) {
        self.ecs.lock().send_event(GotoEvent {
            entity: self.entity,
            goal: Arc::new(goal),
            successors_fn: moves::default_move,
            allow_mining: true,
            min_timeout: PathfinderTimeout::Time(Duration::from_secs(1)),
            max_timeout: PathfinderTimeout::Time(Duration::from_secs(5)),
        });
    }

    /// Same as [`start_goto`](Self::start_goto). but the bot won't break any
    /// blocks while executing the path.
    fn start_goto_without_mining(&self, goal: impl Goal + 'static) {
        self.ecs.lock().send_event(GotoEvent {
            entity: self.entity,
            goal: Arc::new(goal),
            successors_fn: moves::default_move,
            allow_mining: false,
            min_timeout: PathfinderTimeout::Time(Duration::from_secs(1)),
            max_timeout: PathfinderTimeout::Time(Duration::from_secs(5)),
        });
    }

    fn stop_pathfinding(&self) {
        self.ecs.lock().send_event(StopPathfindingEvent {
            entity: self.entity,
            force: false,
        });
    }

    /// Waits forever until the bot no longer has a pathfinder goal.
    async fn wait_until_goto_target_reached(&self) {
        // we do this to make sure the event got handled before we start checking
        // is_goto_target_reached
        self.wait_updates(1).await;

        let mut tick_broadcaster = self.get_tick_broadcaster();
        while !self.is_goto_target_reached() {
            // check every tick
            match tick_broadcaster.recv().await {
                Ok(_) => (),
                Err(RecvError::Closed) => return,
                Err(err) => warn!("{err}"),
            };
        }
    }

    fn is_goto_target_reached(&self) -> bool {
        self.map_get_component::<Pathfinder, _>(|p| p.goal.is_none() && !p.is_calculating)
            .unwrap_or(true)
    }
}

#[derive(Component)]
pub struct ComputePath(Task<Option<PathFoundEvent>>);

#[allow(clippy::type_complexity)]
pub fn goto_listener(
    mut commands: Commands,
    mut events: EventReader<GotoEvent>,
    mut query: Query<(
        &mut Pathfinder,
        Option<&ExecutingPath>,
        &Position,
        &InstanceName,
        &Inventory,
        Option<&CustomPathfinderState>,
    )>,
    instance_container: Res<InstanceContainer>,
) {
    let thread_pool = AsyncComputeTaskPool::get();

    for event in events.read() {
        let Ok((mut pathfinder, executing_path, position, instance_name, inventory, custom_state)) =
            query.get_mut(event.entity)
        else {
            warn!("got goto event for an entity that can't pathfind");
            continue;
        };

        if event.goal.success(BlockPos::from(position)) {
            // we're already at the goal, nothing to do
            pathfinder.goal = None;
            pathfinder.successors_fn = None;
            pathfinder.is_calculating = false;
            debug!("already at goal, not pathfinding");
            continue;
        }

        // we store the goal so it can be recalculated later if necessary
        pathfinder.goal = Some(event.goal.clone());
        pathfinder.successors_fn = Some(event.successors_fn);
        pathfinder.is_calculating = true;
        pathfinder.allow_mining = event.allow_mining;
        pathfinder.min_timeout = Some(event.min_timeout);
        pathfinder.max_timeout = Some(event.max_timeout);

        let start = if let Some(executing_path) = executing_path
            && let Some(final_node) = executing_path.path.back()
        {
            // if we're currently pathfinding and got a goto event, start a little ahead
            executing_path
                .path
                .get(50)
                .unwrap_or(final_node)
                .movement
                .target
        } else {
            BlockPos::from(position)
        };

        if start == BlockPos::from(position) {
            info!("got goto {:?}, starting from {start:?}", event.goal);
        } else {
            info!(
                "got goto {:?}, starting from {start:?} (currently at {:?})",
                event.goal,
                BlockPos::from(position)
            );
        }

        let successors_fn: moves::SuccessorsFn = event.successors_fn;

        let world_lock = instance_container
            .get(instance_name)
            .expect("Entity tried to pathfind but the entity isn't in a valid world");

        let goal = event.goal.clone();
        let entity = event.entity;

        let goto_id_atomic = pathfinder.goto_id.clone();

        let allow_mining = event.allow_mining;
        let mining_cache = MiningCache::new(if allow_mining {
            Some(inventory.inventory_menu.clone())
        } else {
            None
        });

        let custom_state = custom_state.cloned().unwrap_or_default();

        let min_timeout = event.min_timeout;
        let max_timeout = event.max_timeout;

        let task = thread_pool.spawn(async move {
            calculate_path(CalculatePathOpts {
                entity,
                start,
                goal,
                successors_fn,
                world_lock,
                goto_id_atomic,
                allow_mining,
                mining_cache,
                custom_state,
                min_timeout,
                max_timeout,
            })
        });

        commands.entity(event.entity).insert(ComputePath(task));
    }
}

pub struct CalculatePathOpts {
    pub entity: Entity,
    pub start: BlockPos,
    pub goal: Arc<dyn Goal>,
    pub successors_fn: SuccessorsFn,
    pub world_lock: Arc<RwLock<azalea_world::Instance>>,
    pub goto_id_atomic: Arc<AtomicUsize>,
    pub allow_mining: bool,
    pub mining_cache: MiningCache,
    pub custom_state: CustomPathfinderState,
    /// Also see [`GotoEvent::min_timeout`].
    pub min_timeout: PathfinderTimeout,
    pub max_timeout: PathfinderTimeout,
}

/// Calculate the [`PathFoundEvent`] for the given pathfinder options.
///
/// You usually want to just use [`PathfinderClientExt::goto`] or send a
/// [`GotoEvent`] instead of calling this directly.
///
/// You are expected to immediately send the `PathFoundEvent` you received after
/// calling this function. `None` will be returned if the pathfinding was
/// interrupted by another path calculation.
pub fn calculate_path(opts: CalculatePathOpts) -> Option<PathFoundEvent> {
    debug!("start: {:?}", opts.start);

    let goto_id = opts.goto_id_atomic.fetch_add(1, atomic::Ordering::SeqCst) + 1;

    let origin = opts.start;
    let cached_world = CachedWorld::new(opts.world_lock, origin);
    let successors = |pos: RelBlockPos| {
        call_successors_fn(
            &cached_world,
            &opts.mining_cache,
            &opts.custom_state.0.read(),
            opts.successors_fn,
            pos,
        )
    };

    let start_time = Instant::now();

    let astar::Path {
        movements,
        is_partial,
    } = a_star(
        RelBlockPos::get_origin(origin),
        |n| opts.goal.heuristic(n.apply(origin)),
        successors,
        |n| opts.goal.success(n.apply(origin)),
        opts.min_timeout,
        opts.max_timeout,
    );
    let end_time = Instant::now();
    debug!("partial: {is_partial:?}");
    let duration = end_time - start_time;
    if is_partial {
        if movements.is_empty() {
            info!("Pathfinder took {duration:?} (empty path)");
        } else {
            info!("Pathfinder took {duration:?} (incomplete path)");
        }
        // wait a bit so it's not a busy loop
        thread::sleep(Duration::from_millis(100));
    } else {
        info!("Pathfinder took {duration:?}");
    }

    debug!("Path:");
    for movement in &movements {
        debug!("  {}", movement.target.apply(origin));
    }

    let path = movements.into_iter().collect::<VecDeque<_>>();

    let goto_id_now = opts.goto_id_atomic.load(atomic::Ordering::SeqCst);
    if goto_id != goto_id_now {
        // we must've done another goto while calculating this path, so throw it away
        warn!("finished calculating a path, but it's outdated");
        return None;
    }

    if path.is_empty() && is_partial {
        debug!("this path is empty, we might be stuck :(");
    }

    let mut mapped_path = VecDeque::with_capacity(path.len());
    let mut current_position = RelBlockPos::get_origin(origin);
    for movement in path {
        let mut found_edge = None;
        for edge in successors(current_position) {
            if edge.movement.target == movement.target {
                found_edge = Some(edge);
                break;
            }
        }

        let found_edge = found_edge.expect(
            "path should always still be possible because we're using the same world cache",
        );
        current_position = found_edge.movement.target;

        // we don't just clone the found_edge because we're using BlockPos instead of
        // RelBlockPos as the target type
        mapped_path.push_back(Edge {
            movement: astar::Movement {
                target: movement.target.apply(origin),
                data: movement.data,
            },
            cost: found_edge.cost,
        });
    }

    Some(PathFoundEvent {
        entity: opts.entity,
        start: opts.start,
        path: Some(mapped_path),
        is_partial,
        successors_fn: opts.successors_fn,
        allow_mining: opts.allow_mining,
    })
}

// poll the tasks and send the PathFoundEvent if they're done
pub fn handle_tasks(
    mut commands: Commands,
    mut transform_tasks: Query<(Entity, &mut ComputePath)>,
    mut path_found_events: EventWriter<PathFoundEvent>,
) {
    for (entity, mut task) in &mut transform_tasks {
        if let Some(optional_path_found_event) = future::block_on(future::poll_once(&mut task.0)) {
            if let Some(path_found_event) = optional_path_found_event {
                path_found_events.write(path_found_event);
            }

            // Task is complete, so remove task component from entity
            commands.entity(entity).remove::<ComputePath>();
        }
    }
}

// set the path for the target entity when we get the PathFoundEvent
#[allow(clippy::type_complexity)]
pub fn path_found_listener(
    mut events: EventReader<PathFoundEvent>,
    mut query: Query<(
        &mut Pathfinder,
        Option<&mut ExecutingPath>,
        &InstanceName,
        &Inventory,
        Option<&CustomPathfinderState>,
    )>,
    instance_container: Res<InstanceContainer>,
    mut commands: Commands,
) {
    for event in events.read() {
        let (mut pathfinder, executing_path, instance_name, inventory, custom_state) = query
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
                    let origin = event.start;
                    let successors_fn: moves::SuccessorsFn = event.successors_fn;
                    let cached_world = CachedWorld::new(world_lock, origin);
                    let mining_cache = MiningCache::new(if event.allow_mining {
                        Some(inventory.inventory_menu.clone())
                    } else {
                        None
                    });
                    let custom_state = custom_state.cloned().unwrap_or_default();
                    let custom_state_ref = custom_state.0.read();
                    let successors = |pos: RelBlockPos| {
                        call_successors_fn(
                            &cached_world,
                            &mining_cache,
                            &custom_state_ref,
                            successors_fn,
                            pos,
                        )
                    };

                    if let Some(first_node_of_new_path) = path.front() {
                        let last_target_of_current_path = RelBlockPos::from_origin(
                            origin,
                            last_node_of_current_path.movement.target,
                        );
                        let first_target_of_new_path = RelBlockPos::from_origin(
                            origin,
                            first_node_of_new_path.movement.target,
                        );

                        if successors(last_target_of_current_path)
                            .iter()
                            .any(|edge| edge.movement.target == first_target_of_new_path)
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

#[allow(clippy::type_complexity)]
pub fn timeout_movement(
    mut query: Query<(
        Entity,
        &mut Pathfinder,
        &mut ExecutingPath,
        &Position,
        Option<&Mining>,
        &InstanceName,
        &Inventory,
        Option<&CustomPathfinderState>,
    )>,
    instance_container: Res<InstanceContainer>,
) {
    for (
        entity,
        mut pathfinder,
        mut executing_path,
        position,
        mining,
        instance_name,
        inventory,
        custom_state,
    ) in &mut query
    {
        // don't timeout if we're mining
        if let Some(mining) = mining {
            // also make sure we're close enough to the block that's being mined
            if mining.pos.distance_squared_to(&BlockPos::from(position)) < 6_i32.pow(2) {
                // also reset the last_node_reached_at so we don't timeout after we finish
                // mining
                executing_path.last_node_reached_at = Instant::now();
                continue;
            }
        }

        if executing_path.last_node_reached_at.elapsed() > Duration::from_secs(2)
            && !pathfinder.is_calculating
            && !executing_path.path.is_empty()
        {
            warn!("pathfinder timeout, trying to patch path");
            executing_path.queued_path = None;
            executing_path.last_reached_node = BlockPos::from(position);

            let world_lock = instance_container
                .get(instance_name)
                .expect("Entity tried to pathfind but the entity isn't in a valid world");
            let Some(successors_fn) = pathfinder.successors_fn else {
                warn!(
                    "pathfinder was going to patch path because of timeout, but there was no successors_fn"
                );
                return;
            };

            let custom_state = custom_state.cloned().unwrap_or_default();

            // try to fix the path without recalculating everything.
            // (though, it'll still get fully recalculated by `recalculate_near_end_of_path`
            // if the new path is too short)
            patch_path(
                0..=cmp::min(20, executing_path.path.len() - 1),
                &mut executing_path,
                &mut pathfinder,
                inventory,
                entity,
                successors_fn,
                world_lock,
                custom_state,
            );
            // reset last_node_reached_at so we don't immediately try to patch again
            executing_path.last_node_reached_at = Instant::now();
        }
    }
}

pub fn check_node_reached(
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
            for (i, edge) in executing_path
                .path
                .clone()
                .into_iter()
                .enumerate()
                .take(20)
                .rev()
            {
                let movement = edge.movement;
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
                    physics.on_ground()
                    // 0.5 to handle non-full blocks
                    && BlockPos::from(position.up(0.5)) == movement.target
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
                    trace!("reached node {}", movement.target);

                    if let Some(new_path) = executing_path.queued_path.take() {
                        debug!(
                            "swapped path to {:?}",
                            new_path.iter().take(10).collect::<Vec<_>>()
                        );
                        executing_path.path = new_path;

                        if executing_path.path.is_empty() {
                            info!("the path we just swapped to was empty, so reached end of path");
                            walk_events.write(StartWalkEvent {
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
                        walk_events.write(StartWalkEvent {
                            entity,
                            direction: WalkDirection::None,
                        });
                        commands.entity(entity).remove::<ExecutingPath>();
                        if let Some(goal) = pathfinder.goal.clone()
                            && goal.success(movement.target)
                        {
                            info!("goal was reached!");
                            pathfinder.goal = None;
                            pathfinder.successors_fn = None;
                        }
                    }

                    break;
                }
            }
            break;
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn check_for_path_obstruction(
    mut query: Query<(
        Entity,
        &mut Pathfinder,
        &mut ExecutingPath,
        &InstanceName,
        &Inventory,
        Option<&CustomPathfinderState>,
    )>,
    instance_container: Res<InstanceContainer>,
) {
    for (entity, mut pathfinder, mut executing_path, instance_name, inventory, custom_state) in
        &mut query
    {
        let Some(successors_fn) = pathfinder.successors_fn else {
            continue;
        };

        let world_lock = instance_container
            .get(instance_name)
            .expect("Entity tried to pathfind but the entity isn't in a valid world");

        // obstruction check (the path we're executing isn't possible anymore)
        let origin = executing_path.last_reached_node;
        let cached_world = CachedWorld::new(world_lock, origin);
        let mining_cache = MiningCache::new(if pathfinder.allow_mining {
            Some(inventory.inventory_menu.clone())
        } else {
            None
        });
        let custom_state = custom_state.cloned().unwrap_or_default();
        let custom_state_ref = custom_state.0.read();
        let successors = |pos: RelBlockPos| {
            call_successors_fn(
                &cached_world,
                &mining_cache,
                &custom_state_ref,
                successors_fn,
                pos,
            )
        };

        let Some(obstructed_index) = check_path_obstructed(
            origin,
            RelBlockPos::from_origin(origin, executing_path.last_reached_node),
            &executing_path.path,
            successors,
        ) else {
            continue;
        };

        drop(custom_state_ref);

        warn!(
            "path obstructed at index {obstructed_index} (starting at {:?})",
            executing_path.last_reached_node,
        );
        debug!("obstructed path: {:?}", executing_path.path);
        // if it's near the end, don't bother recalculating a patch, just truncate and
        // mark it as partial
        if obstructed_index + 5 > executing_path.path.len() {
            debug!(
                "obstruction is near the end of the path, truncating and marking path as partial"
            );
            executing_path.path.truncate(obstructed_index);
            executing_path.is_path_partial = true;
            continue;
        }

        let Some(successors_fn) = pathfinder.successors_fn else {
            error!("got PatchExecutingPathEvent but the bot has no successors_fn");
            continue;
        };

        let world_lock = instance_container
            .get(instance_name)
            .expect("Entity tried to pathfind but the entity isn't in a valid world");

        // patch up to 20 nodes
        let patch_end_index = cmp::min(obstructed_index + 20, executing_path.path.len() - 1);

        patch_path(
            obstructed_index..=patch_end_index,
            &mut executing_path,
            &mut pathfinder,
            inventory,
            entity,
            successors_fn,
            world_lock,
            custom_state.clone(),
        );
    }
}

/// update the given [`ExecutingPath`] to recalculate the path of the nodes in
/// the given index range.
///
/// You should avoid making the range too large, since the timeout for the A*
/// calculation is very low. About 20 nodes is a good amount.
#[allow(clippy::too_many_arguments)]
fn patch_path(
    patch_nodes: RangeInclusive<usize>,
    executing_path: &mut ExecutingPath,
    pathfinder: &mut Pathfinder,
    inventory: &Inventory,
    entity: Entity,
    successors_fn: SuccessorsFn,
    world_lock: Arc<RwLock<azalea_world::Instance>>,
    custom_state: CustomPathfinderState,
) {
    let patch_start = if *patch_nodes.start() == 0 {
        executing_path.last_reached_node
    } else {
        executing_path.path[*patch_nodes.start() - 1]
            .movement
            .target
    };

    let patch_end = executing_path.path[*patch_nodes.end()].movement.target;

    // this doesn't override the main goal, it's just the goal for this A*
    // calculation
    let goal = Arc::new(BlockPosGoal(patch_end));

    let goto_id_atomic = pathfinder.goto_id.clone();

    let allow_mining = pathfinder.allow_mining;
    let mining_cache = MiningCache::new(if allow_mining {
        Some(inventory.inventory_menu.clone())
    } else {
        None
    });

    // the timeout is small enough that this doesn't need to be async
    let path_found_event = calculate_path(CalculatePathOpts {
        entity,
        start: patch_start,
        goal,
        successors_fn,
        world_lock,
        goto_id_atomic,
        allow_mining,
        mining_cache,
        custom_state,
        min_timeout: PathfinderTimeout::Nodes(10_000),
        max_timeout: PathfinderTimeout::Nodes(10_000),
    });

    // this is necessary in case we interrupted another ongoing path calculation
    pathfinder.is_calculating = false;

    debug!("obstruction patch: {path_found_event:?}");

    let mut new_path = VecDeque::new();
    if *patch_nodes.start() > 0 {
        new_path.extend(
            executing_path
                .path
                .iter()
                .take(*patch_nodes.start())
                .cloned(),
        );
    }

    let mut is_patch_complete = false;
    if let Some(path_found_event) = path_found_event {
        if let Some(found_path_patch) = path_found_event.path
            && !found_path_patch.is_empty()
        {
            new_path.extend(found_path_patch);

            if !path_found_event.is_partial {
                new_path.extend(executing_path.path.iter().skip(*patch_nodes.end()).cloned());
                is_patch_complete = true;
                debug!("the patch is not partial :)");
            } else {
                debug!("the patch is partial, throwing away rest of path :(");
            }
        }
    } else {
        // no path found, rip
    }

    executing_path.path = new_path;
    if !is_patch_complete {
        executing_path.is_path_partial = true;
    }
}

pub fn recalculate_near_end_of_path(
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
        if (executing_path.path.len() == 50 || executing_path.path.len() < 5)
            && !pathfinder.is_calculating
            && executing_path.is_path_partial
        {
            match pathfinder.goal.as_ref().cloned() {
                Some(goal) => {
                    debug!("Recalculating path because it's empty or ends soon");
                    debug!(
                        "recalculate_near_end_of_path executing_path.is_path_partial: {}",
                        executing_path.is_path_partial
                    );
                    goto_events.write(GotoEvent {
                        entity,
                        goal,
                        successors_fn,
                        allow_mining: pathfinder.allow_mining,
                        min_timeout: if executing_path.path.len() == 50 {
                            // we have quite some time until the node is reached, soooo we might as
                            // well burn some cpu cycles to get a good path
                            PathfinderTimeout::Time(Duration::from_secs(5))
                        } else {
                            PathfinderTimeout::Time(Duration::from_secs(1))
                        },
                        max_timeout: pathfinder.max_timeout.expect("max_timeout should be set"),
                    });
                    pathfinder.is_calculating = true;

                    if executing_path.path.is_empty() {
                        if let Some(new_path) = executing_path.queued_path.take() {
                            executing_path.path = new_path;
                            if executing_path.path.is_empty() {
                                info!(
                                    "the path we just swapped to was empty, so reached end of path"
                                );
                                walk_events.write(StartWalkEvent {
                                    entity,
                                    direction: WalkDirection::None,
                                });
                                commands.entity(entity).remove::<ExecutingPath>();
                                break;
                            }
                        } else {
                            walk_events.write(StartWalkEvent {
                                entity,
                                direction: WalkDirection::None,
                            });
                            commands.entity(entity).remove::<ExecutingPath>();
                        }
                    }
                }
                _ => {
                    if executing_path.path.is_empty() {
                        // idk when this can happen but stop moving just in case
                        walk_events.write(StartWalkEvent {
                            entity,
                            direction: WalkDirection::None,
                        });
                    }
                }
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn tick_execute_path(
    mut query: Query<(
        Entity,
        &mut ExecutingPath,
        &Position,
        &Physics,
        Option<&Mining>,
        &InstanceHolder,
        &Inventory,
    )>,
    mut look_at_events: EventWriter<LookAtEvent>,
    mut sprint_events: EventWriter<StartSprintEvent>,
    mut walk_events: EventWriter<StartWalkEvent>,
    mut jump_events: EventWriter<JumpEvent>,
    mut start_mining_events: EventWriter<StartMiningBlockEvent>,
    mut set_selected_hotbar_slot_events: EventWriter<SetSelectedHotbarSlotEvent>,
) {
    for (entity, executing_path, position, physics, mining, instance_holder, inventory_component) in
        &mut query
    {
        if let Some(edge) = executing_path.path.front() {
            let ctx = ExecuteCtx {
                entity,
                target: edge.movement.target,
                position: **position,
                start: executing_path.last_reached_node,
                physics,
                is_currently_mining: mining.is_some(),
                instance: instance_holder.instance.clone(),
                menu: inventory_component.inventory_menu.clone(),

                look_at_events: &mut look_at_events,
                sprint_events: &mut sprint_events,
                walk_events: &mut walk_events,
                jump_events: &mut jump_events,
                start_mining_events: &mut start_mining_events,
                set_selected_hotbar_slot_events: &mut set_selected_hotbar_slot_events,
            };
            trace!(
                "executing move, position: {}, last_reached_node: {}",
                **position, executing_path.last_reached_node
            );
            (edge.movement.data.execute)(ctx);
        }
    }
}

pub fn recalculate_if_has_goal_but_no_path(
    mut query: Query<(Entity, &mut Pathfinder), Without<ExecutingPath>>,
    mut goto_events: EventWriter<GotoEvent>,
) {
    for (entity, mut pathfinder) in &mut query {
        if pathfinder.goal.is_some()
            && !pathfinder.is_calculating
            && let Some(goal) = pathfinder.goal.as_ref().cloned()
        {
            debug!("Recalculating path because it has a goal but no ExecutingPath");
            goto_events.write(GotoEvent {
                entity,
                goal,
                successors_fn: pathfinder.successors_fn.unwrap(),
                allow_mining: pathfinder.allow_mining,
                min_timeout: pathfinder.min_timeout.expect("min_timeout should be set"),
                max_timeout: pathfinder.max_timeout.expect("max_timeout should be set"),
            });
            pathfinder.is_calculating = true;
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

pub fn handle_stop_pathfinding_event(
    mut events: EventReader<StopPathfindingEvent>,
    mut query: Query<(&mut Pathfinder, &mut ExecutingPath)>,
    mut walk_events: EventWriter<StartWalkEvent>,
    mut commands: Commands,
) {
    for event in events.read() {
        // stop computing any path that's being computed
        commands.entity(event.entity).remove::<ComputePath>();

        let Ok((mut pathfinder, mut executing_path)) = query.get_mut(event.entity) else {
            continue;
        };
        pathfinder.goal = None;
        if event.force {
            executing_path.path.clear();
            executing_path.queued_path = None;
        } else {
            // switch to an empty path as soon as it can
            executing_path.queued_path = Some(VecDeque::new());
            // make sure it doesn't recalculate
            executing_path.is_path_partial = false;
        }

        if executing_path.path.is_empty() {
            walk_events.write(StartWalkEvent {
                entity: event.entity,
                direction: WalkDirection::None,
            });
            commands.entity(event.entity).remove::<ExecutingPath>();
        }
    }
}

pub fn stop_pathfinding_on_instance_change(
    mut query: Query<(Entity, &mut ExecutingPath), Changed<InstanceName>>,
    mut stop_pathfinding_events: EventWriter<StopPathfindingEvent>,
) {
    for (entity, mut executing_path) in &mut query {
        if !executing_path.path.is_empty() {
            debug!("instance changed, clearing path");
            executing_path.path.clear();
            stop_pathfinding_events.write(StopPathfindingEvent {
                entity,
                force: true,
            });
        }
    }
}

/// Checks whether the path has been obstructed, and returns Some(index) if it
/// has been. The index is of the first obstructed node.
pub fn check_path_obstructed<SuccessorsFn>(
    origin: BlockPos,
    mut current_position: RelBlockPos,
    path: &VecDeque<astar::Edge<BlockPos, moves::MoveData>>,
    successors_fn: SuccessorsFn,
) -> Option<usize>
where
    SuccessorsFn: Fn(RelBlockPos) -> Vec<astar::Edge<RelBlockPos, moves::MoveData>>,
{
    for (i, edge) in path.iter().enumerate() {
        let movement_target = RelBlockPos::from_origin(origin, edge.movement.target);

        let mut found_edge = None;
        for candidate_edge in successors_fn(current_position) {
            if candidate_edge.movement.target == movement_target {
                found_edge = Some(candidate_edge);
                break;
            }
        }

        current_position = movement_target;
        // if found_edge is None or the cost increased, then return the index
        if found_edge
            .map(|found_edge| found_edge.cost > edge.cost)
            .unwrap_or(true)
        {
            // if the node that we're currently executing was obstructed then it's often too
            // late to change the path, so it's usually better to just ignore this case :/
            if i == 0 {
                warn!("path obstructed at index 0, ignoring");
                continue;
            }

            return Some(i);
        }
    }

    None
}

pub fn call_successors_fn(
    cached_world: &CachedWorld,
    mining_cache: &MiningCache,
    custom_state: &CustomPathfinderStateRef,
    successors_fn: SuccessorsFn,
    pos: RelBlockPos,
) -> Vec<astar::Edge<RelBlockPos, moves::MoveData>> {
    let mut edges = Vec::with_capacity(16);
    let mut ctx = PathfinderCtx {
        edges: &mut edges,
        world: cached_world,
        mining_cache,
        custom_state,
    };
    successors_fn(&mut ctx, pos);
    edges
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashSet,
        sync::Arc,
        thread,
        time::{Duration, Instant},
    };

    use azalea_block::BlockState;
    use azalea_core::position::{BlockPos, ChunkPos, Vec3};
    use azalea_world::{Chunk, ChunkStorage, PartialChunkStorage};

    use super::{
        GotoEvent,
        astar::PathfinderTimeout,
        goals::BlockPosGoal,
        moves,
        simulation::{SimulatedPlayerBundle, Simulation},
    };

    fn setup_blockposgoal_simulation(
        partial_chunks: &mut PartialChunkStorage,
        start_pos: BlockPos,
        end_pos: BlockPos,
        solid_blocks: &[BlockPos],
    ) -> Simulation {
        let mut simulation = setup_simulation_world(partial_chunks, start_pos, solid_blocks, &[]);

        // you can uncomment this while debugging tests to get trace logs
        // simulation.app.add_plugins(bevy_log::LogPlugin {
        //     level: bevy_log::Level::TRACE,
        //     filter: "".to_string(),
        //     ..Default::default()
        // });

        simulation.app.world_mut().send_event(GotoEvent {
            entity: simulation.entity,
            goal: Arc::new(BlockPosGoal(end_pos)),
            successors_fn: moves::default_move,
            allow_mining: false,
            min_timeout: PathfinderTimeout::Nodes(1_000_000),
            max_timeout: PathfinderTimeout::Nodes(5_000_000),
        });
        simulation
    }

    fn setup_simulation_world(
        partial_chunks: &mut PartialChunkStorage,
        start_pos: BlockPos,
        solid_blocks: &[BlockPos],
        extra_blocks: &[(BlockPos, BlockState)],
    ) -> Simulation {
        let mut chunk_positions = HashSet::new();
        for block_pos in solid_blocks {
            chunk_positions.insert(ChunkPos::from(block_pos));
        }
        for (block_pos, _) in extra_blocks {
            chunk_positions.insert(ChunkPos::from(block_pos));
        }

        let mut chunks = ChunkStorage::default();
        for chunk_pos in chunk_positions {
            partial_chunks.set(&chunk_pos, Some(Chunk::default()), &mut chunks);
        }
        for block_pos in solid_blocks {
            chunks.set_block_state(*block_pos, azalea_registry::Block::Stone.into());
        }
        for (block_pos, block_state) in extra_blocks {
            chunks.set_block_state(*block_pos, *block_state);
        }

        let player = SimulatedPlayerBundle::new(Vec3::new(
            start_pos.x as f64 + 0.5,
            start_pos.y as f64,
            start_pos.z as f64 + 0.5,
        ));
        Simulation::new(chunks, player)
    }

    pub fn assert_simulation_reaches(simulation: &mut Simulation, ticks: usize, end_pos: BlockPos) {
        wait_until_bot_starts_moving(simulation);
        for _ in 0..ticks {
            simulation.tick();
        }
        assert_eq!(BlockPos::from(simulation.position()), end_pos);
    }

    pub fn wait_until_bot_starts_moving(simulation: &mut Simulation) {
        let start_pos = simulation.position();
        let start_time = Instant::now();
        while simulation.position() == start_pos
            && !simulation.is_mining()
            && start_time.elapsed() < Duration::from_millis(500)
        {
            simulation.tick();
            thread::yield_now();
        }
    }

    #[test]
    fn test_simple_forward() {
        let mut partial_chunks = PartialChunkStorage::default();
        let mut simulation = setup_blockposgoal_simulation(
            &mut partial_chunks,
            BlockPos::new(0, 71, 0),
            BlockPos::new(0, 71, 1),
            &[BlockPos::new(0, 70, 0), BlockPos::new(0, 70, 1)],
        );
        assert_simulation_reaches(&mut simulation, 20, BlockPos::new(0, 71, 1));
    }

    #[test]
    fn test_double_diagonal_with_walls() {
        let mut partial_chunks = PartialChunkStorage::default();
        let mut simulation = setup_blockposgoal_simulation(
            &mut partial_chunks,
            BlockPos::new(0, 71, 0),
            BlockPos::new(2, 71, 2),
            &[
                BlockPos::new(0, 70, 0),
                BlockPos::new(1, 70, 1),
                BlockPos::new(2, 70, 2),
                BlockPos::new(1, 72, 0),
                BlockPos::new(2, 72, 1),
            ],
        );
        assert_simulation_reaches(&mut simulation, 30, BlockPos::new(2, 71, 2));
    }

    #[test]
    fn test_jump_with_sideways_momentum() {
        let mut partial_chunks = PartialChunkStorage::default();
        let mut simulation = setup_blockposgoal_simulation(
            &mut partial_chunks,
            BlockPos::new(0, 71, 3),
            BlockPos::new(5, 76, 0),
            &[
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
        assert_simulation_reaches(&mut simulation, 120, BlockPos::new(5, 76, 0));
    }

    #[test]
    fn test_parkour_2_block_gap() {
        let mut partial_chunks = PartialChunkStorage::default();
        let mut simulation = setup_blockposgoal_simulation(
            &mut partial_chunks,
            BlockPos::new(0, 71, 0),
            BlockPos::new(0, 71, 3),
            &[BlockPos::new(0, 70, 0), BlockPos::new(0, 70, 3)],
        );
        assert_simulation_reaches(&mut simulation, 40, BlockPos::new(0, 71, 3));
    }

    #[test]
    fn test_descend_and_parkour_2_block_gap() {
        let mut partial_chunks = PartialChunkStorage::default();
        let mut simulation = setup_blockposgoal_simulation(
            &mut partial_chunks,
            BlockPos::new(0, 71, 0),
            BlockPos::new(3, 67, 4),
            &[
                BlockPos::new(0, 70, 0),
                BlockPos::new(0, 69, 1),
                BlockPos::new(0, 68, 2),
                BlockPos::new(0, 67, 3),
                BlockPos::new(0, 66, 4),
                BlockPos::new(3, 66, 4),
            ],
        );
        assert_simulation_reaches(&mut simulation, 100, BlockPos::new(3, 67, 4));
    }

    #[test]
    fn test_small_descend_and_parkour_2_block_gap() {
        let mut partial_chunks = PartialChunkStorage::default();
        let mut simulation = setup_blockposgoal_simulation(
            &mut partial_chunks,
            BlockPos::new(0, 71, 0),
            BlockPos::new(0, 70, 5),
            &[
                BlockPos::new(0, 70, 0),
                BlockPos::new(0, 70, 1),
                BlockPos::new(0, 69, 2),
                BlockPos::new(0, 69, 5),
            ],
        );
        assert_simulation_reaches(&mut simulation, 40, BlockPos::new(0, 70, 5));
    }

    #[test]
    fn test_quickly_descend() {
        let mut partial_chunks = PartialChunkStorage::default();
        let mut simulation = setup_blockposgoal_simulation(
            &mut partial_chunks,
            BlockPos::new(0, 71, 0),
            BlockPos::new(0, 68, 3),
            &[
                BlockPos::new(0, 70, 0),
                BlockPos::new(0, 69, 1),
                BlockPos::new(0, 68, 2),
                BlockPos::new(0, 67, 3),
            ],
        );
        assert_simulation_reaches(&mut simulation, 60, BlockPos::new(0, 68, 3));
    }

    #[test]
    fn test_2_gap_ascend_thrice() {
        let mut partial_chunks = PartialChunkStorage::default();
        let mut simulation = setup_blockposgoal_simulation(
            &mut partial_chunks,
            BlockPos::new(0, 71, 0),
            BlockPos::new(3, 74, 0),
            &[
                BlockPos::new(0, 70, 0),
                BlockPos::new(0, 71, 3),
                BlockPos::new(3, 72, 3),
                BlockPos::new(3, 73, 0),
            ],
        );
        assert_simulation_reaches(&mut simulation, 60, BlockPos::new(3, 74, 0));
    }

    #[test]
    fn test_consecutive_3_gap_parkour() {
        let mut partial_chunks = PartialChunkStorage::default();
        let mut simulation = setup_blockposgoal_simulation(
            &mut partial_chunks,
            BlockPos::new(0, 71, 0),
            BlockPos::new(4, 71, 12),
            &[
                BlockPos::new(0, 70, 0),
                BlockPos::new(0, 70, 4),
                BlockPos::new(0, 70, 8),
                BlockPos::new(0, 70, 12),
                BlockPos::new(4, 70, 12),
            ],
        );
        assert_simulation_reaches(&mut simulation, 80, BlockPos::new(4, 71, 12));
    }

    #[test]
    fn test_jumps_with_more_sideways_momentum() {
        let mut partial_chunks = PartialChunkStorage::default();
        let mut simulation = setup_blockposgoal_simulation(
            &mut partial_chunks,
            BlockPos::new(0, 71, 0),
            BlockPos::new(4, 74, 9),
            &[
                BlockPos::new(0, 70, 0),
                BlockPos::new(0, 70, 1),
                BlockPos::new(0, 70, 2),
                BlockPos::new(0, 71, 3),
                BlockPos::new(0, 72, 6),
                BlockPos::new(0, 73, 9),
                // this is the point where the bot might fall if it has too much momentum
                BlockPos::new(2, 73, 9),
                BlockPos::new(4, 73, 9),
            ],
        );
        assert_simulation_reaches(&mut simulation, 80, BlockPos::new(4, 74, 9));
    }

    #[test]
    fn test_mine_through_non_colliding_block() {
        let mut partial_chunks = PartialChunkStorage::default();

        let mut simulation = setup_simulation_world(
            &mut partial_chunks,
            // the pathfinder can't actually dig straight down, so we start a block to the side so
            // it can descend correctly
            BlockPos::new(0, 72, 1),
            &[BlockPos::new(0, 71, 1)],
            &[
                (
                    BlockPos::new(0, 71, 0),
                    azalea_registry::Block::SculkVein.into(),
                ),
                (
                    BlockPos::new(0, 70, 0),
                    azalea_registry::Block::GrassBlock.into(),
                ),
                // this is an extra check to make sure that we don't accidentally break the block
                // below (since tnt will break instantly)
                (BlockPos::new(0, 69, 0), azalea_registry::Block::Tnt.into()),
            ],
        );

        simulation.app.world_mut().send_event(GotoEvent {
            entity: simulation.entity,
            goal: Arc::new(BlockPosGoal(BlockPos::new(0, 69, 0))),
            successors_fn: moves::default_move,
            allow_mining: true,
            min_timeout: PathfinderTimeout::Nodes(1_000_000),
            max_timeout: PathfinderTimeout::Nodes(5_000_000),
        });

        assert_simulation_reaches(&mut simulation, 200, BlockPos::new(0, 70, 0));
    }
}
