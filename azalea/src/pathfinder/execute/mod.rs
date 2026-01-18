pub mod patching;

use std::{cmp, time::Duration};

use azalea_block::{BlockState, BlockTrait};
use azalea_client::{
    StartSprintEvent, StartWalkEvent,
    local_player::WorldHolder,
    mining::{Mining, MiningSystems, StartMiningBlockEvent},
};
use azalea_core::tick::GameTick;
use azalea_entity::{Physics, Position, inventory::Inventory};
use azalea_physics::{PhysicsSystems, get_block_pos_below_that_affects_movement};
use azalea_world::{WorldName, Worlds};
use bevy_app::{App, Plugin};
use bevy_ecs::prelude::*;
use tracing::{debug, info, trace, warn};

use crate::{
    WalkDirection,
    bot::{JumpEvent, LookAtEvent},
    ecs::{
        entity::Entity,
        query::Without,
        system::{Commands, Query, Res},
    },
    pathfinder::{
        ExecutingPath, GotoEvent, Pathfinder,
        astar::PathfinderTimeout,
        custom_state::CustomPathfinderState,
        debug::debug_render_path_with_particles,
        execute,
        moves::{ExecuteCtx, IsReachedCtx},
        player_pos_to_block_pos,
    },
};

pub struct DefaultPathfinderExecutionPlugin;
impl Plugin for DefaultPathfinderExecutionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            // putting systems in the GameTick schedule makes them run every Minecraft tick
            // (every 50 milliseconds).
            GameTick,
            (
                execute::timeout_movement,
                execute::patching::check_for_path_obstruction,
                execute::check_node_reached,
                execute::tick_execute_path,
                execute::recalculate_near_end_of_path,
                execute::recalculate_if_has_goal_but_no_path,
            )
                .chain()
                .after(PhysicsSystems)
                .after(azalea_client::movement::send_position)
                .after(MiningSystems)
                .after(debug_render_path_with_particles),
        );
    }
}

#[allow(clippy::type_complexity)]
pub fn tick_execute_path(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut ExecutingPath,
        &Position,
        &Physics,
        Option<&Mining>,
        &WorldHolder,
        &Inventory,
    )>,
    mut look_at_events: MessageWriter<LookAtEvent>,
    mut sprint_events: MessageWriter<StartSprintEvent>,
    mut walk_events: MessageWriter<StartWalkEvent>,
    mut jump_events: MessageWriter<JumpEvent>,
    mut start_mining_events: MessageWriter<StartMiningBlockEvent>,
) {
    for (
        entity,
        mut executing_path,
        position,
        physics,
        mining,
        world_holder,
        inventory_component,
    ) in &mut query
    {
        executing_path.ticks_since_last_node_reached += 1;

        if let Some(edge) = executing_path.path.front() {
            let mut ctx = ExecuteCtx {
                entity,
                target: edge.movement.target,
                position: **position,
                start: executing_path.last_reached_node,
                physics,
                is_currently_mining: mining.is_some(),
                world: world_holder.shared.clone(),
                menu: inventory_component.inventory_menu.clone(),

                commands: &mut commands,
                look_at_events: &mut look_at_events,
                sprint_events: &mut sprint_events,
                walk_events: &mut walk_events,
                jump_events: &mut jump_events,
                start_mining_events: &mut start_mining_events,
            };
            ctx.on_tick_start();
            trace!(
                "executing move, position: {}, last_reached_node: {}",
                **position, executing_path.last_reached_node
            );
            (edge.movement.data.execute)(ctx);
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
        &WorldName,
    )>,
    mut walk_events: MessageWriter<StartWalkEvent>,
    mut commands: Commands,
    worlds: Res<Worlds>,
) {
    for (entity, mut pathfinder, mut executing_path, position, physics, world_name) in &mut query {
        let Some(world) = worlds.get(world_name) else {
            warn!("entity is pathfinding but not in a valid world");
            continue;
        };

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
                let extra_check = if i == executing_path.path.len() - 1 {
                    // be extra strict about the velocity and centering if we're on the last node so
                    // we don't fall off

                    let x_difference_from_center = position.x - (movement.target.x as f64 + 0.5);
                    let z_difference_from_center = position.z - (movement.target.z as f64 + 0.5);

                    let block_pos_below = get_block_pos_below_that_affects_movement(*position);

                    let block_state_below = {
                        let world = world.read();
                        world
                            .chunks
                            .get_block_state(block_pos_below)
                            .unwrap_or(BlockState::AIR)
                    };
                    let block_below: Box<dyn BlockTrait> = block_state_below.into();
                    // friction for normal blocks is 0.6, for ice it's 0.98
                    let block_friction = block_below.behavior().friction as f64;

                    // if the block has the default friction, this will multiply by 1
                    // for blocks like ice, it'll multiply by a higher number
                    let scaled_velocity = physics.velocity * (0.4 / (1. - block_friction));

                    let x_predicted_offset = (x_difference_from_center + scaled_velocity.x).abs();
                    let z_predicted_offset = (z_difference_from_center + scaled_velocity.z).abs();

                    // this is to make sure we don't fall off immediately after finishing the path
                    physics.on_ground()
                        && player_pos_to_block_pos(**position) == movement.target
                        // adding the delta like this isn't a perfect solution but it helps to make
                        // sure we don't keep going if our delta is high
                        && x_predicted_offset < 0.2
                        && z_predicted_offset < 0.2
                } else {
                    true
                };

                if (movement.data.is_reached)(is_reached_ctx) && extra_check {
                    executing_path.path = executing_path.path.split_off(i + 1);
                    executing_path.last_reached_node = movement.target;
                    executing_path.ticks_since_last_node_reached = 0;
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
pub fn timeout_movement(
    mut query: Query<(
        Entity,
        &mut Pathfinder,
        &mut ExecutingPath,
        &Position,
        Option<&Mining>,
        &WorldName,
        &Inventory,
        Option<&CustomPathfinderState>,
    )>,
    worlds: Res<Worlds>,
) {
    for (
        entity,
        mut pathfinder,
        mut executing_path,
        position,
        mining,
        world_name,
        inventory,
        custom_state,
    ) in &mut query
    {
        // don't timeout if we're mining
        if let Some(mining) = mining {
            // also make sure we're close enough to the block that's being mined
            if mining.pos.distance_squared_to(position.into()) < 6_i32.pow(2) {
                // also reset the ticks_since_last_node_reached so we don't timeout after we
                // finish mining
                executing_path.ticks_since_last_node_reached = 0;
                continue;
            }
        }

        if executing_path.ticks_since_last_node_reached > (2 * 20)
            && !pathfinder.is_calculating
            && !executing_path.path.is_empty()
        {
            warn!("pathfinder timeout, trying to patch path");
            executing_path.queued_path = None;
            let cur_pos = player_pos_to_block_pos(**position);
            executing_path.last_reached_node = cur_pos;

            let world_lock = worlds
                .get(world_name)
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
            patching::patch_path(
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
            executing_path.ticks_since_last_node_reached = 0
        }
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
