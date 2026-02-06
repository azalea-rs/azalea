use std::{cmp, collections::VecDeque, ops::RangeInclusive, sync::Arc};

use azalea_core::position::BlockPos;
use azalea_entity::inventory::Inventory;
use azalea_world::{WorldName, Worlds};
use bevy_ecs::{
    entity::Entity,
    system::{Query, Res},
};
use parking_lot::RwLock;
use tracing::{debug, error, warn};

use crate::pathfinder::{
    CalculatePathCtx, ExecutingPath, Pathfinder, PathfinderOpts,
    astar::{self, PathfinderTimeout},
    calculate_path, call_successors_fn,
    custom_state::CustomPathfinderState,
    goals::BlockPosGoal,
    mining::MiningCache,
    moves,
    positions::RelBlockPos,
    world::CachedWorld,
};

#[allow(clippy::type_complexity)]
pub fn check_for_path_obstruction(
    mut query: Query<(
        Entity,
        &mut Pathfinder,
        &mut ExecutingPath,
        &WorldName,
        &Inventory,
        Option<&CustomPathfinderState>,
    )>,
    worlds: Res<Worlds>,
) {
    for (entity, mut pathfinder, mut executing_path, world_name, inventory, custom_state) in
        &mut query
    {
        let Some(opts) = pathfinder.opts.clone() else {
            continue;
        };

        let world_lock = worlds
            .get(world_name)
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

        let world_lock = worlds
            .get(world_name)
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

/// Update the given [`ExecutingPath`] to recalculate the path of the nodes in
/// the given index range.
///
/// You should avoid making the range too large, since the timeout for the A*
/// calculation is very low. About 20 nodes is a good amount.
#[allow(clippy::too_many_arguments)]
pub fn patch_path(
    patch_nodes: RangeInclusive<usize>,
    executing_path: &mut ExecutingPath,
    pathfinder: &mut Pathfinder,
    inventory: &Inventory,
    entity: Entity,
    world_lock: Arc<RwLock<azalea_world::World>>,
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

/// Checks whether the path has been obstructed, and returns Some(index) if it
/// has been.
///
/// The index is of the first obstructed node.
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
                warn!("path obstructed at index 0 ({edge:?}), ignoring");
                continue;
            }

            return Some(i);
        }
    }

    None
}
