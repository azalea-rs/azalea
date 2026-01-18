//! A pathfinding plugin to make bots able to traverse the world.
//!
//! For the new functions on `Client` that the pathfinder adds, see
//! [`PathfinderClientExt`].
//!
//! Note that the pathfinder is highly optimized, but it will be very slow if
//! it's not compiled with optimizations enabled.
//!
//! For smoother and more realistic path execution, also see
//! [`SimulationPathfinderExecutionPlugin`].
//!
//! Much of the pathfinder's code is based on [Baritone](https://github.com/cabaletta/baritone). <3
//!
//! [`SimulationPathfinderExecutionPlugin`]: execute::simulation::SimulationPathfinderExecutionPlugin

pub mod astar;
pub mod costs;
pub mod custom_state;
pub mod debug;
pub mod execute;
pub mod goals;
mod goto_event;
pub mod mining;
pub mod moves;
pub mod positions;
pub mod simulation;
#[cfg(test)]
mod tests;
pub mod world;

use std::{
    collections::VecDeque,
    sync::{
        Arc,
        atomic::{self, AtomicUsize},
    },
    thread,
    time::{Duration, Instant},
};

use astar::Edge;
use azalea_client::{StartWalkEvent, inventory::InventorySystems, movement::MoveEventsSystems};
use azalea_core::{
    position::{BlockPos, Vec3},
    tick::GameTick,
};
use azalea_entity::{LocalEntity, Position, inventory::Inventory, metadata::Player};
use azalea_world::{WorldName, Worlds};
use bevy_app::{PreUpdate, Update};
use bevy_ecs::prelude::*;
use bevy_tasks::{AsyncComputeTaskPool, Task};
use custom_state::{CustomPathfinderState, CustomPathfinderStateRef};
use futures_lite::future;
pub use goto_event::{GotoEvent, PathfinderOpts};
use parking_lot::RwLock;
use positions::RelBlockPos;
use tokio::sync::broadcast::error::RecvError;
use tracing::{debug, error, info, warn};

use self::{
    debug::debug_render_path_with_particles, goals::Goal, mining::MiningCache, moves::SuccessorsFn,
};
use crate::{
    Client, WalkDirection,
    app::{App, Plugin},
    ecs::{
        component::Component,
        entity::Entity,
        query::{With, Without},
        system::{Commands, Query, Res},
    },
    pathfinder::{
        astar::a_star, execute::DefaultPathfinderExecutionPlugin, moves::MovesCtx,
        world::CachedWorld,
    },
};

#[derive(Clone, Default)]
pub struct PathfinderPlugin;
impl Plugin for PathfinderPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<GotoEvent>()
            .add_message::<PathFoundEvent>()
            .add_message::<StopPathfindingEvent>()
            .add_systems(GameTick, debug_render_path_with_particles)
            .add_systems(PreUpdate, add_default_pathfinder)
            .add_systems(
                Update,
                (
                    goto_listener,
                    handle_tasks,
                    stop_pathfinding_on_world_change,
                    path_found_listener,
                    handle_stop_pathfinding_event,
                )
                    .chain()
                    .before(MoveEventsSystems)
                    .before(InventorySystems),
            )
            .add_plugins(DefaultPathfinderExecutionPlugin);
    }
}

/// A component that makes this client able to pathfind.
#[derive(Clone, Component, Default)]
#[non_exhaustive]
pub struct Pathfinder {
    pub goal: Option<Arc<dyn Goal>>,
    pub opts: Option<PathfinderOpts>,
    pub is_calculating: bool,
    pub goto_id: Arc<AtomicUsize>,
}

/// A component that's present on clients that are actively following a
/// pathfinder path.
#[derive(Clone, Component)]
pub struct ExecutingPath {
    pub path: VecDeque<astar::Edge<BlockPos, moves::MoveData>>,
    pub queued_path: Option<VecDeque<astar::Edge<BlockPos, moves::MoveData>>>,
    pub last_reached_node: BlockPos,
    // count ticks instead of using real time to make our timeouts more consistent, in case we lag
    // and our ticks take a while
    pub ticks_since_last_node_reached: usize,
    pub is_path_partial: bool,
}

#[derive(Clone, Debug, Message)]
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
    /// Whether the pathfinder is currently following a path.
    ///
    /// Also see [`Self::is_calculating_path`] and
    /// [`Self::is_goto_target_reached`].
    fn is_executing_path(&self) -> bool;
    /// Whether the pathfinder is currently calculating a path.
    ///
    /// Also see [`Self::is_executing_path`] and
    /// [`Self::is_goto_target_reached`].
    fn is_calculating_path(&self) -> bool;
}

impl PathfinderClientExt for Client {
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
            .write()
            .write_message(GotoEvent::new(self.entity, goal, opts));
    }
    fn stop_pathfinding(&self) {
        self.ecs.write().write_message(StopPathfindingEvent {
            entity: self.entity,
            force: false,
        });
    }
    fn force_stop_pathfinding(&self) {
        self.ecs.write().write_message(StopPathfindingEvent {
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
        self.get_component::<Pathfinder>()
            .is_none_or(|p| p.goal.is_none() && !p.is_calculating)
    }
    fn is_executing_path(&self) -> bool {
        self.get_component::<ExecutingPath>().is_some()
    }
    fn is_calculating_path(&self) -> bool {
        self.get_component::<Pathfinder>()
            .is_some_and(|p| p.is_calculating)
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
        Option<&mut ExecutingPath>,
        &Position,
        &WorldName,
        &Inventory,
        Option<&CustomPathfinderState>,
    )>,
    worlds: Res<Worlds>,
) {
    let thread_pool = AsyncComputeTaskPool::get();

    for event in events.read() {
        let Ok((mut pathfinder, executing_path, position, world_name, inventory, custom_state)) =
            query.get_mut(event.entity)
        else {
            warn!("got goto event for an entity that can't pathfind");
            continue;
        };

        let cur_pos = player_pos_to_block_pos(**position);

        if event.goal.success(cur_pos) {
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

        let start = if let Some(mut executing_path) = executing_path
            && { !executing_path.path.is_empty() }
        {
            // if we're currently pathfinding and got a goto event, start a little ahead

            let executing_path_limit = 50;
            // truncate the executing path so we can cleanly combine the two paths later
            executing_path.path.truncate(executing_path_limit);

            executing_path
                .path
                .back()
                .expect("path was just checked to not be empty")
                .movement
                .target
        } else {
            cur_pos
        };

        if start == cur_pos {
            info!("got goto {:?}, starting from {start:?}", event.goal);
        } else {
            info!(
                "got goto {:?}, starting from {start:?} (currently at {cur_pos:?})",
                event.goal,
            );
        }

        let world_lock = worlds
            .get(world_name)
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

/// Convert a player position to a block position, used internally in the
/// pathfinder.
///
/// This is almost the same as `BlockPos::from(position)`, except that non-full
/// blocks are handled correctly.
#[inline]
pub fn player_pos_to_block_pos(position: Vec3) -> BlockPos {
    // 0.5 to account for non-full blocks
    BlockPos::from(position.up(0.5))
}

pub struct CalculatePathCtx {
    pub entity: Entity,
    pub start: BlockPos,
    pub goal: Arc<dyn Goal>,
    pub world_lock: Arc<RwLock<azalea_world::World>>,
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
        cost,
    } = a_star(
        RelBlockPos::get_origin(origin),
        |n| ctx.goal.heuristic(n.apply(origin)),
        successors,
        |n| ctx.goal.success(n.apply(origin)),
        ctx.opts.min_timeout,
        ctx.opts.max_timeout,
    );
    let end_time = Instant::now();
    debug!("partial: {is_partial:?}, cost: {cost}");
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
        &WorldName,
        &Inventory,
        Option<&CustomPathfinderState>,
    )>,
    worlds: Res<Worlds>,
    mut commands: Commands,
) {
    for event in events.read() {
        let Ok((mut pathfinder, executing_path, world_name, inventory, custom_state)) =
            query.get_mut(event.entity)
        else {
            debug!("got path found event for an entity that can't pathfind");
            continue;
        };
        if let Some(path) = &event.path {
            if let Some(mut executing_path) = executing_path {
                let mut new_path = VecDeque::new();

                // combine the old and new paths if the first node of the new path is a
                // successor of the last node of the old path
                if let Some(last_node_of_current_path) = executing_path.path.back() {
                    let world_lock = worlds
                        .get(world_name)
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
                    ticks_since_last_node_reached: 0,
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

#[derive(Message)]
pub struct StopPathfindingEvent {
    pub entity: Entity,
    /// Whether we should stop moving immediately without waiting for the
    /// current movement to finish.
    ///
    /// This is usually set to false, since it might cause the bot to fall if it
    /// was in the middle of parkouring.
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

pub fn stop_pathfinding_on_world_change(
    mut query: Query<(Entity, &mut ExecutingPath), Changed<WorldName>>,
    mut stop_pathfinding_events: MessageWriter<StopPathfindingEvent>,
) {
    for (entity, mut executing_path) in &mut query {
        if !executing_path.path.is_empty() {
            debug!("world changed, clearing path");
            executing_path.path.clear();
            stop_pathfinding_events.write(StopPathfindingEvent {
                entity,
                force: true,
            });
        }
    }
}

pub fn call_successors_fn(
    cached_world: &CachedWorld,
    mining_cache: &MiningCache,
    custom_state: &CustomPathfinderStateRef,
    successors_fn: SuccessorsFn,
    pos: RelBlockPos,
) -> Vec<astar::Edge<RelBlockPos, moves::MoveData>> {
    let mut edges = Vec::with_capacity(16);
    let mut ctx = MovesCtx {
        edges: &mut edges,
        world: cached_world,
        mining_cache,
        custom_state,
    };
    successors_fn(&mut ctx, pos);
    edges
}
