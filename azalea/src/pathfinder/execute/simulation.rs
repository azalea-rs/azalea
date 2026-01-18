//! An alternative execution engine for the pathfinder that attempts to skip
//! nodes in the path by running simulations.
//!
//! See [`SimulationPathfinderExecutionPlugin`] for more information.

use std::{borrow::Cow, time::Instant};

use azalea_client::{
    PhysicsState, SprintDirection, StartSprintEvent, StartWalkEvent,
    local_player::WorldHolder,
    mining::{Mining, MiningSystems, StartMiningBlockEvent},
};
use azalea_core::{position::BlockPos, tick::GameTick};
use azalea_entity::{Attributes, LookDirection, Physics, Position, inventory::Inventory};
use azalea_physics::PhysicsSystems;
use bevy_app::{App, Plugin};
use bevy_ecs::{prelude::*, system::SystemState};
use tracing::{debug, trace};

use crate::{
    WalkDirection,
    bot::{JumpEvent, LookAtEvent, direction_looking_at},
    ecs::{
        entity::Entity,
        system::{Commands, Query},
    },
    pathfinder::{
        ExecutingPath,
        debug::debug_render_path_with_particles,
        moves::{ExecuteCtx, IsReachedCtx},
        simulation::{SimulatedPlayerBundle, Simulation},
    },
};

/// An alternative execution engine for the pathfinder that attempts to skip
/// nodes in the path by running simulations.
///
/// This allows it to smooth the path and sprint-jump without failing jumps or
/// looking unnatural. However, this comes at the cost of execution being more
/// expensive and potentially less stable.
///
/// To use it, simply add [`SimulationPathfinderExecutionPlugin`] as a plugin.
///
/// ```
/// use azalea::{
///     pathfinder::execute::simulation::SimulationPathfinderExecutionPlugin, swarm::prelude::*,
/// };
///
/// let builder = SwarmBuilder::new().add_plugins(SimulationPathfinderExecutionPlugin);
/// // ...
/// ```
///
/// [`DefaultPathfinderExecutionPlugin`]: super::DefaultPathfinderExecutionPlugin
pub struct SimulationPathfinderExecutionPlugin;
impl Plugin for SimulationPathfinderExecutionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            GameTick,
            (
                super::timeout_movement,
                super::patching::check_for_path_obstruction,
                super::check_node_reached,
                tick_execute_path,
                super::recalculate_near_end_of_path,
                super::recalculate_if_has_goal_but_no_path,
            )
                .chain()
                .after(PhysicsSystems)
                .after(azalea_client::movement::send_position)
                .after(MiningSystems)
                .after(debug_render_path_with_particles),
        );
    }
}

#[derive(Clone, Component, Debug)]
pub enum SimulatingPathState {
    Fail,
    Simulated(SimulatingPathOpts),
}
#[derive(Clone, Component, Debug)]
pub struct SimulatingPathOpts {
    pub start: BlockPos,
    pub target: BlockPos,
    pub jumping: bool,
    pub jump_until_target_distance: f64,
    pub jump_after_start_distance: f64,
    pub sprinting: bool,
    pub y_rot: f32,
}

#[allow(clippy::type_complexity)]
pub fn tick_execute_path(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut ExecutingPath,
        &mut LookDirection,
        &Position,
        &Physics,
        &PhysicsState,
        Option<&Mining>,
        &WorldHolder,
        &Attributes,
        &Inventory,
        Option<&SimulatingPathState>,
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
        mut look_direction,
        position,
        physics,
        physics_state,
        mining,
        world_holder,
        attributes,
        inventory,
        mut simulating_path_state,
    ) in &mut query
    {
        executing_path.ticks_since_last_node_reached += 1;

        if executing_path.ticks_since_last_node_reached == 1 {
            if let Some(SimulatingPathState::Simulated(s)) = simulating_path_state {
                // only reset the state if we just reached the end of the simulation path (for
                // performance)
                if s.target == executing_path.last_reached_node
                    // or if the current simulation target isn't in the path, reset too
                    || !executing_path
                        .path
                        .iter()
                        .any(|e| e.movement.target == s.target)
                {
                    simulating_path_state = None;
                }
            } else {
                simulating_path_state = None;
            }
        }

        let simulating_path_state = if let Some(simulating_path_state) = simulating_path_state {
            Cow::Borrowed(simulating_path_state)
        } else {
            let start = Instant::now();
            let new_state = run_simulations(
                &executing_path,
                world_holder,
                SimulatedPlayerBundle {
                    position: *position,
                    physics: physics.clone(),
                    physics_state: physics_state.clone(),
                    look_direction: *look_direction,
                    attributes: attributes.clone(),
                    inventory: inventory.clone(),
                },
            );
            debug!("found sim in {:?}: {new_state:?}", start.elapsed());
            commands.entity(entity).insert(new_state.clone());
            Cow::Owned(new_state)
        };

        match &*simulating_path_state {
            SimulatingPathState::Fail => {
                if let Some(edge) = executing_path.path.front() {
                    let mut ctx = ExecuteCtx {
                        entity,
                        target: edge.movement.target,
                        position: **position,
                        start: executing_path.last_reached_node,
                        physics,
                        is_currently_mining: mining.is_some(),
                        can_mine: true,
                        world: world_holder.shared.clone(),
                        menu: inventory.inventory_menu.clone(),

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
            SimulatingPathState::Simulated(SimulatingPathOpts {
                start,
                target,
                jumping,
                jump_until_target_distance,
                jump_after_start_distance,
                sprinting,
                y_rot,
            }) => {
                look_direction.update(LookDirection::new(*y_rot, 0.));

                if *sprinting {
                    sprint_events.write(StartSprintEvent {
                        entity,
                        direction: SprintDirection::Forward,
                    });
                } else if physics_state.was_sprinting {
                    walk_events.write(StartWalkEvent {
                        entity,
                        direction: WalkDirection::None,
                    });
                } else {
                    walk_events.write(StartWalkEvent {
                        entity,
                        direction: WalkDirection::Forward,
                    });
                }
                if *jumping
                    && target.center().horizontal_distance_squared_to(**position)
                        > jump_until_target_distance.powi(2)
                    && start.center().horizontal_distance_squared_to(**position)
                        > jump_after_start_distance.powi(2)
                {
                    jump_events.write(JumpEvent { entity });
                }
            }
        }

        //
    }
}

fn run_simulations(
    executing_path: &ExecutingPath,
    world_holder: &WorldHolder,
    player: SimulatedPlayerBundle,
) -> SimulatingPathState {
    let swimming = player.physics.is_in_water();

    let mut sim = Simulation::new(world_holder.shared.read().chunks.clone(), player.clone());

    for nodes_ahead in [20, 15, 10, 5, 4, 3, 2, 1, 0] {
        if nodes_ahead + 1 >= executing_path.path.len() {
            // don't simulate to the last node since it has stricter checks
            continue;
        }

        let mut results = Vec::new();

        if let Some(simulating_to) = executing_path.path.get(nodes_ahead) {
            let y_rot =
                direction_looking_at(*player.position, simulating_to.movement.target.center())
                    .y_rot();

            for jump_until_target_distance in [0., 1., 3.] {
                for jump_after_start_distance in [0., 0.5] {
                    for jumping in [true, false] {
                        if !jumping
                            && (jump_until_target_distance != 0. || jump_after_start_distance != 0.)
                        {
                            continue;
                        }

                        // this loop is left here in case you wanna try re-enabling walking, but
                        // it doesn't seem that useful
                        for sprinting in [true] {
                            if !sprinting && nodes_ahead > 2 {
                                continue;
                            }
                            if swimming {
                                if !sprinting
                                    || jump_until_target_distance > 0.
                                    || jump_after_start_distance > 0.
                                {
                                    continue;
                                }
                            } else if jump_until_target_distance == 0. {
                                continue;
                            }

                            let state = SimulatingPathOpts {
                                start: BlockPos::from(player.position),
                                target: simulating_to.movement.target,
                                jumping,
                                jump_until_target_distance,
                                jump_after_start_distance,
                                sprinting,
                                y_rot,
                            };
                            let sim_res = run_one_simulation(
                                &mut sim,
                                player.clone(),
                                state.clone(),
                                executing_path,
                                nodes_ahead,
                                if swimming {
                                    (nodes_ahead * 12) + 20
                                } else {
                                    (nodes_ahead * 4) + 20
                                },
                            );
                            if sim_res.success {
                                results.push((state, sim_res.ticks));
                            }
                        }
                    }
                }
            }
        }

        if !results.is_empty() {
            let fastest = results.iter().min_by_key(|r| r.1).unwrap().0.clone();
            return SimulatingPathState::Simulated(fastest);
        }
    }

    SimulatingPathState::Fail
}

struct SimulationResult {
    success: bool,
    ticks: usize,
}
fn run_one_simulation(
    sim: &mut Simulation,
    player: SimulatedPlayerBundle,
    state: SimulatingPathOpts,
    executing_path: &ExecutingPath,
    nodes_ahead: usize,
    timeout_ticks: usize,
) -> SimulationResult {
    let simulating_to = &executing_path.path[nodes_ahead];

    let start = BlockPos::from(player.position);
    sim.reset(player);

    let simulating_to_block = simulating_to.movement.target;

    let mut success = false;
    let mut total_ticks = 0;

    for ticks in 1..=timeout_ticks {
        let position = sim.position();
        let ecs = sim.app.world_mut();

        ecs.get_mut::<LookDirection>(sim.entity)
            .unwrap()
            .update(LookDirection::new(state.y_rot, 0.));

        if state.sprinting {
            ecs.write_message(StartSprintEvent {
                entity: sim.entity,
                direction: SprintDirection::Forward,
            });
        } else if ecs
            .get::<PhysicsState>(sim.entity)
            .map(|p| p.trying_to_sprint)
            .unwrap_or_default()
        {
            // have to let go for a tick to be able to start walking
            ecs.write_message(StartWalkEvent {
                entity: sim.entity,
                direction: WalkDirection::None,
            });
        } else {
            ecs.write_message(StartWalkEvent {
                entity: sim.entity,
                direction: WalkDirection::Forward,
            });
        }
        if state.jumping
            && simulating_to_block
                .center()
                .horizontal_distance_squared_to(position)
                > state.jump_until_target_distance.powi(2)
            && start.center().horizontal_distance_squared_to(position)
                > state.jump_after_start_distance.powi(2)
        {
            ecs.write_message(JumpEvent { entity: sim.entity });
        }

        sim.tick();

        let physics = sim.physics();
        if physics.horizontal_collision
            || physics.is_in_lava()
            || (physics.velocity.y < -0.7 && !physics.is_in_water())
        {
            // fail
            break;
        }

        if (simulating_to.movement.data.is_reached)(IsReachedCtx {
            target: simulating_to_block,
            start,
            position: sim.position(),
            physics: &physics,
        }) {
            success = true;
            total_ticks = ticks;
            break;
        }
    }

    if success {
        // now verify that the path is safe by continuing to the next node

        let mut followup_success = false;

        let next_node = &executing_path.path[nodes_ahead + 1];
        for _ in 1..=30 {
            // add ticks here so if we sort by ticks later it'll be more accurate
            total_ticks += 1;

            {
                let mut system_state = SystemState::<(
                    Commands,
                    Query<(&Position, &Physics, Option<&Mining>, &Inventory)>,
                    MessageWriter<LookAtEvent>,
                    MessageWriter<StartSprintEvent>,
                    MessageWriter<StartWalkEvent>,
                    MessageWriter<JumpEvent>,
                    MessageWriter<StartMiningBlockEvent>,
                )>::new(sim.app.world_mut());
                let (
                    mut commands,
                    query,
                    mut look_at_events,
                    mut sprint_events,
                    mut walk_events,
                    mut jump_events,
                    mut start_mining_events,
                ) = system_state.get_mut(sim.app.world_mut());

                let (position, physics, mining, inventory) = query.get(sim.entity).unwrap();

                if physics.horizontal_collision {
                    // if the simulated move made us hit a wall then it's bad
                    break;
                }
                if physics.velocity.y < -0.7 && !physics.is_in_water() {
                    break;
                }

                (next_node.movement.data.execute)(ExecuteCtx {
                    entity: sim.entity,
                    target: next_node.movement.target,
                    start: simulating_to_block,
                    position: **position,
                    physics,
                    is_currently_mining: mining.is_some(),
                    // don't modify the world from the simulation
                    can_mine: false,
                    world: sim.world.clone(),
                    menu: inventory.inventory_menu.clone(),

                    commands: &mut commands,
                    look_at_events: &mut look_at_events,
                    sprint_events: &mut sprint_events,
                    walk_events: &mut walk_events,
                    jump_events: &mut jump_events,
                    start_mining_events: &mut start_mining_events,
                });
                system_state.apply(sim.app.world_mut());
            }

            sim.tick();

            if (next_node.movement.data.is_reached)(IsReachedCtx {
                target: next_node.movement.target,
                start: simulating_to_block,
                position: sim.position(),
                physics: &sim.physics(),
            }) {
                followup_success = true;
                break;
            }
        }

        if !followup_success {
            debug!("followup failed");
            success = false;
        }
    }

    SimulationResult {
        success,
        ticks: total_ticks,
    }
}
