//! A pathfinding plugin to make bots able to traverse the world.
//!
//! For the new functions on `Client` that the pathfinder adds, see
//! [`PathfinderClientExt`].
//!
//! Much of this code is based on [Baritone](https://github.com/cabaletta/baritone).

pub mod astar;
pub mod costs;
pub mod custom_state;
pub mod debug;
pub mod goals;
mod goto_event;
pub mod mining;
pub mod moves;
pub mod rel_block_pos;
pub mod simulation;
#[cfg(test)]
mod tests;
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
pub use goto_event::{GotoEvent, PathfinderOpts};
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
    WalkDirection,
    app::{App, Plugin},
    bot::{BotClientExt, JumpEvent, LookAtEvent},
    ecs::{
        component::Component,
        entity::Entity,
        query::{With, Without},
        system::{Commands, Query, Res},
    },
    pathfinder::{astar::a_star, moves::PathfinderCtx, world::CachedWorld},
};

#[derive(Clone, Default)]
pub struct PathfinderPlugin;
impl Plugin for PathfinderPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<GotoEvent>()
            .add_message::<PathFoundEvent>()
            .add_message::<StopPathfindingEvent>()
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
#[non_exhaustive]
pub struct Pathfinder {
    pub goal: Option<Arc<dyn Goal>>,
    pub opts: Option<PathfinderOpts>,
    pub is_calculating: bool,
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

#[derive(Message, Clone, Debug)]
#[non_exhaustive]
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
    fn goto(&self, goal: impl Goal + 'static) -> impl Future<Output = ()>;
    /// Same as [`Self::goto`], but allows you to set custom options for
    /// pathfinding, including disabling mining and setting custom moves.
    ///
    /// ```
    /// # use azalea::prelude::*;
    /// # use azalea::{BlockPos, pathfinder::{goals::BlockPosGoal, PathfinderOpts}};
    /// # async fn example(bot: &Client) {
    /// bot.goto_with_opts(
    ///     BlockPosGoal(BlockPos::new(0, 70, 0)),
    ///     PathfinderOpts::new().allow_mining(false),
    /// )
    /// .await;
    /// # }
    /// ```
    fn goto_with_opts(
        &self,
        goal: impl Goal + 'static,
        opts: PathfinderOpts,
    ) -> impl Future<Output = ()>;
    /// Start pathfinding to a given goal.
    ///
    /// ```
    /// # use azalea::prelude::*;
    /// # use azalea::{BlockPos, pathfinder::goals::BlockPosGoal};
    /// # fn example(bot: &Client) {
    /// bot.start_goto(BlockPosGoal(BlockPos::new(0, 70, 0)));
    /// # }
    /// ```
    fn start_goto(&self, goal: impl Goal + 'static);
    /// Same as [`Self::start_goto`], but allows you to set custom
    /// options for pathfinding, including disabling mining and setting custom
    /// moves.
    ///
    /// Also see [`Self::goto_with_opts`].
    fn start_goto_with_opts(&self, goal: impl Goal + 'static, opts: PathfinderOpts);
    /// Stop calculating a path, and stop moving once the current movement is
    /// finished.
    ///
    /// This behavior exists to prevent the bot from taking damage if
    /// `stop_pathfinding` was called while executing a parkour jump, but if
    /// it's undesirable then you may want to consider using
    /// [`Self::force_stop_pathfinding`] instead.
    fn stop_pathfinding(&self);
    /// Stop calculating a path and stop executing the current movement
    /// immediately.
    fn force_stop_pathfinding(&self);
    /// Waits forever until the bot no longer has a pathfinder goal.
    fn wait_until_goto_target_reached(&self) -> impl Future<Output = ()>;
    /// Returns true if the pathfinder has no active goal and isn't calculating
    /// a path.
    fn is_goto_target_reached(&self) -> bool;
}

impl PathfinderClientExt for azalea_client::Client {
    async fn goto(&self, goal: impl Goal + 'static) {
        self.goto_with_opts(goal, PathfinderOpts::new()).await;
    }
    async fn goto_with_opts(&self, goal: impl Goal + 'static, opts: PathfinderOpts) {
        self.start_goto_with_opts(goal, opts);
        self.wait_until_goto_target_reached().await;
    }
    fn start_goto(&self, goal: impl Goal + 'static) {
        self.start_goto_with_opts(goal, PathfinderOpts::new());
    }
    fn start_goto_with_opts(&self, goal: impl Goal + 'static, opts: PathfinderOpts) {
        self.ecs
            .lock()
            .write_message(GotoEvent::new(self.entity, goal, opts));
    }
    fn stop_pathfinding(&self) {
        self.ecs.lock().write_message(StopPathfindingEvent {
            entity: self.entity,
            force: false,
        });
    }
    fn force_stop_pathfinding(&self) {
        self.ecs.lock().write_message(StopPathfindingEvent {
            entity: self.entity,
            force: true,
        });
    }
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
        self.query_self::<Option<&Pathfinder>, _>(|p| {
            p.map(|p| p.goal.is_none() && !p.is_calculating)
                .unwrap_or(true)
        })
    }
}

#[derive(Component)]
pub struct ComputePath(Task<Option<PathFoundEvent>>);

#[allow(clippy::type_complexity)]
pub fn goto_listener(
    mut commands: Commands,
    mut events: MessageReader<GotoEvent>,
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
            pathfinder.opts = None;
            pathfinder.is_calculating = false;
            debug!("already at goal, not pathfinding");
            continue;
        }

        // we store the goal so it can be recalculated later if necessary
        pathfinder.goal = Some(event.goal.clone());
        pathfinder.opts = Some(event.opts.clone());
        pathfinder.is_calculating = true;

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

        let world_lock = instance_container
            .get(instance_name)
            .expect("Entity tried to pathfind but the entity isn't in a valid world");

        let goal = event.goal.clone();
        let entity = event.entity;

        let goto_id_atomic = pathfinder.goto_id.clone();

        let allow_mining = event.opts.allow_mining;
        let mining_cache = MiningCache::new(if allow_mining {
            Some(inventory.inventory_menu.clone())
        } else {
            None
        });

        let custom_state = custom_state.cloned().unwrap_or_default();
        let opts = event.opts.clone();
        let task = thread_pool.spawn(async move {
            calculate_path(CalculatePathCtx {
                entity,
                start,
                goal,
                world_lock,
                goto_id_atomic,
                mining_cache,
                custom_state,
                opts,
            })
        });

        commands.entity(event.entity).insert(ComputePath(task));
    }
}

pub struct CalculatePathCtx {
    pub entity: Entity,
    pub start: BlockPos,
    pub goal: Arc<dyn Goal>,
    pub world_lock: Arc<RwLock<azalea_world::Instance>>,
    pub goto_id_atomic: Arc<AtomicUsize>,
    pub mining_cache: MiningCache,
    pub custom_state: CustomPathfinderState,

    pub opts: PathfinderOpts,
}

/// Calculate the [`PathFoundEvent`] for the given pathfinder options.
///
/// You usually want to just use [`PathfinderClientExt::goto`] or send a
/// [`GotoEvent`] instead of calling this directly.
///
/// You are expected to immediately send the `PathFoundEvent` you received after
/// calling this function. `None` will be returned if the pathfinding was
/// interrupted by another path calculation.
pub fn calculate_path(ctx: CalculatePathCtx) -> Option<PathFoundEvent> {
    debug!("start: {:?}", ctx.start);

    let goto_id = ctx.goto_id_atomic.fetch_add(1, atomic::Ordering::SeqCst) + 1;

    let origin = ctx.start;
    let cached_world = CachedWorld::new(ctx.world_lock, origin);
    let successors = |pos: RelBlockPos| {
        call_successors_fn(
            &cached_world,
            &ctx.mining_cache,
            &ctx.custom_state.0.read(),
            ctx.opts.successors_fn,
            pos,
        )
    };

    let start_time = Instant::now();

    let astar::Path {
        movements,
        is_partial,
    } = a_star(
        RelBlockPos::get_origin(origin),
        |n| ctx.goal.heuristic(n.apply(origin)),
        successors,
        |n| ctx.goal.success(n.apply(origin)),
        ctx.opts.min_timeout,
        ctx.opts.max_timeout,
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

    let goto_id_now = ctx.goto_id_atomic.load(atomic::Ordering::SeqCst);
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
        entity: ctx.entity,
        start: ctx.start,
        path: Some(mapped_path),
        is_partial,
        successors_fn: ctx.opts.successors_fn,
        allow_mining: ctx.opts.allow_mining,
    })
}

// poll the tasks and send the PathFoundEvent if they're done
pub fn handle_tasks(
    mut commands: Commands,
    mut transform_tasks: Query<(Entity, &mut ComputePath)>,
    mut path_found_events: MessageWriter<PathFoundEvent>,
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
    mut events: MessageReader<PathFoundEvent>,
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
                if !pathfinder.opts.as_ref().is_some_and(|o| o.retry_on_no_path) {
                    debug!("retry_on_no_path is set to false, removing goal");
                    pathfinder.goal = None;
                }
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
            if mining.pos.distance_squared_to(position.into()) < 6_i32.pow(2) {
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
            let Some(opts) = pathfinder.opts.clone() else {
                warn!(
                    "pathfinder was going to patch path because of timeout, but pathfinder.opts was None"
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
                world_lock,
                custom_state,
                opts,
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
    mut walk_events: MessageWriter<StartWalkEvent>,
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
                            pathfinder.opts = None;
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
        let Some(opts) = pathfinder.opts.clone() else {
            continue;
        };

        let world_lock = instance_container
            .get(instance_name)
            .expect("Entity tried to pathfind but the entity isn't in a valid world");

        // obstruction check (the path we're executing isn't possible anymore)
        let origin = executing_path.last_reached_node;
        let cached_world = CachedWorld::new(world_lock, origin);
        let mining_cache = MiningCache::new(if opts.allow_mining {
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
                opts.successors_fn,
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

        let Some(opts) = pathfinder.opts.clone() else {
            error!("got PatchExecutingPathEvent but the bot has no pathfinder opts");
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
            world_lock,
            custom_state.clone(),
            opts,
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
    world_lock: Arc<RwLock<azalea_world::Instance>>,
    custom_state: CustomPathfinderState,
    opts: PathfinderOpts,
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
    let allow_mining = opts.allow_mining;

    let mining_cache = MiningCache::new(if allow_mining {
        Some(inventory.inventory_menu.clone())
    } else {
        None
    });

    // the timeout is small enough that this doesn't need to be async
    let path_found_event = calculate_path(CalculatePathCtx {
        entity,
        start: patch_start,
        goal,
        world_lock,
        goto_id_atomic,
        mining_cache,
        custom_state,
        opts: PathfinderOpts {
            min_timeout: PathfinderTimeout::Nodes(10_000),
            max_timeout: PathfinderTimeout::Nodes(10_000),
            ..opts
        },
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
    mut walk_events: MessageWriter<StartWalkEvent>,
    mut goto_events: MessageWriter<GotoEvent>,
    mut commands: Commands,
) {
    for (entity, mut pathfinder, mut executing_path) in &mut query {
        let Some(mut opts) = pathfinder.opts.clone() else {
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

                    opts.min_timeout = if executing_path.path.len() == 50 {
                        // we have quite some time until the node is reached, soooo we might as
                        // well burn some cpu cycles to get a good path
                        PathfinderTimeout::Time(Duration::from_secs(5))
                    } else {
                        PathfinderTimeout::Time(Duration::from_secs(1))
                    };

                    goto_events.write(GotoEvent { entity, goal, opts });
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
    mut look_at_events: MessageWriter<LookAtEvent>,
    mut sprint_events: MessageWriter<StartSprintEvent>,
    mut walk_events: MessageWriter<StartWalkEvent>,
    mut jump_events: MessageWriter<JumpEvent>,
    mut start_mining_events: MessageWriter<StartMiningBlockEvent>,
    mut set_selected_hotbar_slot_events: MessageWriter<SetSelectedHotbarSlotEvent>,
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
    mut goto_events: MessageWriter<GotoEvent>,
) {
    for (entity, mut pathfinder) in &mut query {
        if pathfinder.goal.is_some()
            && !pathfinder.is_calculating
            && let Some(goal) = pathfinder.goal.as_ref().cloned()
            && let Some(opts) = pathfinder.opts.clone()
        {
            debug!("Recalculating path because it has a goal but no ExecutingPath");
            goto_events.write(GotoEvent { entity, goal, opts });
            pathfinder.is_calculating = true;
        }
    }
}

#[derive(Message)]
pub struct StopPathfindingEvent {
    pub entity: Entity,
    /// If false, then let the current movement finish before stopping. If true,
    /// then stop moving immediately. This might cause the bot to fall if it was
    /// in the middle of parkouring.
    pub force: bool,
}

pub fn handle_stop_pathfinding_event(
    mut events: MessageReader<StopPathfindingEvent>,
    mut query: Query<(&mut Pathfinder, &mut ExecutingPath)>,
    mut walk_events: MessageWriter<StartWalkEvent>,
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
    mut stop_pathfinding_events: MessageWriter<StopPathfindingEvent>,
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
