use std::sync::Arc;

use azalea_block::BlockState;
use azalea_client::{mining::Mining, InstanceHolder};
use azalea_core::position::BlockPos;
use azalea_entity::Position;
use azalea_world::ChunkStorage;
use tracing::info;

use crate::{
    auto_tool::StartMiningBlockWithAutoToolEvent,
    ecs::prelude::*,
    pathfinder::{
        self,
        block_box::BlockBox,
        goals::{Goal, OrGoals, ReachBlockPosGoal, ReachBoxGoal},
        ExecutingPath, GotoEvent, Pathfinder,
    },
    utils::get_reachable_blocks_around_player,
    LookAtEvent,
};

#[derive(Component, Clone)]
pub enum Process {
    MineArea {
        corner1: BlockPos,
        corner2: BlockPos,
    },
}

#[derive(Event)]
pub struct SetActiveProcessEvent {
    pub entity: Entity,
    pub process: Process,
}

pub fn set_active_pathfinder_process_listener(
    mut commands: Commands,
    mut events: EventReader<SetActiveProcessEvent>,
    mut stop_pathfinding_events: EventWriter<pathfinder::StopPathfindingEvent>,
) {
    for event in events.read() {
        stop_pathfinding_events.send(pathfinder::StopPathfindingEvent {
            entity: event.entity,
            force: false,
        });
        commands.entity(event.entity).insert(event.process.clone());
    }
}

fn is_block_mineable(block: BlockState) -> bool {
    !block.is_air()
}

/// Determine what layer should be mined first. This is from the top-down, so 0
/// means the top layer.
fn determine_layer(bb: &BlockBox, chunks: &ChunkStorage) -> usize {
    let mut layer = 0;
    let mut y = bb.max().y;
    while y >= bb.min().y {
        let mut x = bb.min().x;
        while x <= bb.max().x {
            let mut z = bb.min().z;
            while z <= bb.max().z {
                let block = chunks
                    .get_block_state(&BlockPos::new(x, y, z))
                    .unwrap_or_default();
                if is_block_mineable(block) {
                    return layer;
                }
                z += 1;
            }
            x += 1;
        }
        y -= 1;
        layer += 1;
    }
    layer
}

#[allow(clippy::type_complexity)]
pub fn process_tick(
    mut commands: Commands,
    query: Query<(
        Entity,
        &Process,
        &Position,
        &InstanceHolder,
        &Pathfinder,
        Option<&Mining>,
        Option<&ExecutingPath>,
    )>,
    mut goto_events: EventWriter<GotoEvent>,
    mut look_at_events: EventWriter<LookAtEvent>,
    mut start_mining_block_events: EventWriter<StartMiningBlockWithAutoToolEvent>,
) {
    for (entity, process, position, instance_holder, pathfinder, mining, executing_path) in &query {
        match process {
            Process::MineArea { corner1, corner2 } => {
                if pathfinder.goal.is_some() || executing_path.is_some() {
                    // already pathfinding
                    println!("currently pathfinding");
                    continue;
                }

                if mining.is_some() {
                    // currently mining, so wait for that to finish
                    println!("currently mining");
                    continue;
                }

                let bb = BlockBox::new(*corner1, *corner2);
                let chunk_storage = instance_holder.instance.read().chunks.clone();
                let player_position = BlockPos::from(position);

                println!("player_position: {player_position}");

                // the index is from the top-down, so 0 means the top layer
                let layer_index = determine_layer(&bb, &chunk_storage);
                let layer_bb = BlockBox::new(
                    BlockPos::new(
                        bb.min().x,
                        i32::max(bb.min().y, bb.max().y - layer_index as i32),
                        bb.min().z,
                    ),
                    BlockPos::new(
                        bb.max().x,
                        i32::max(bb.min().y, bb.max().y - layer_index as i32),
                        bb.max().z,
                    ),
                );

                let reachable_blocks =
                    get_reachable_blocks_around_player(player_position, &chunk_storage);
                let mineable_blocks = reachable_blocks
                    .into_iter()
                    .filter(|block_pos| {
                        // must be within box
                        if !layer_bb.contains(*block_pos) {
                            return false;
                        }

                        // and must be mineable
                        let block = chunk_storage.get_block_state(block_pos).unwrap_or_default();

                        is_block_mineable(block)
                    })
                    .collect::<Vec<_>>();

                println!("mineable_blocks: {:?}", mineable_blocks);

                if !mineable_blocks.is_empty() {
                    // pick the closest one and mine it
                    let mut closest_block_pos = None;
                    let mut closest_distance = i32::MAX;
                    for block_pos in &mineable_blocks[1..] {
                        if block_pos.y < player_position.y {
                            // skip blocks below us at first
                            continue;
                        }
                        let distance = block_pos.distance_squared_to(&player_position);
                        if distance < closest_distance {
                            closest_block_pos = Some(*block_pos);
                            closest_distance = distance;
                        }
                    }

                    if closest_block_pos.is_none() {
                        // ok now check every block if the only ones around us are below
                        for block_pos in &mineable_blocks {
                            let distance = block_pos.distance_squared_to(&player_position);
                            if distance < closest_distance {
                                closest_block_pos = Some(*block_pos);
                                closest_distance = distance;
                            }
                        }
                    }

                    let closest_block_pos = closest_block_pos.expect(
                        "there must be a closest block because mineable_blocks wasn't empty",
                    );
                    look_at_events.send(LookAtEvent {
                        entity,
                        position: closest_block_pos.center(),
                    });
                    start_mining_block_events.send(StartMiningBlockWithAutoToolEvent {
                        entity,
                        position: closest_block_pos,
                    });

                    println!("start mining block {closest_block_pos:?}");
                    continue;
                }

                // no mineable blocks, so go towards the blocks that can be mined

                let goal: Arc<dyn Goal> = if bb.distance_squared_to(player_position) < 16 * 16 {
                    // already close enough to the box, path to the closest
                    // block instead

                    let mut block_positions_and_distances = Vec::new();
                    for x in layer_bb.min().x..=layer_bb.max().x {
                        for y in layer_bb.min().y..=layer_bb.max().y {
                            for z in layer_bb.min().z..=layer_bb.max().z {
                                let block_pos = BlockPos::new(x, y, z);

                                if !is_block_mineable(
                                    chunk_storage
                                        .get_block_state(&block_pos)
                                        .unwrap_or_default(),
                                ) {
                                    continue;
                                }

                                let distance = block_pos.distance_squared_to(&player_position);
                                block_positions_and_distances.push((block_pos, distance));
                            }
                        }
                    }

                    if block_positions_and_distances.is_empty() {
                        info!("MineArea process is done, no more blocks to mine!");
                        commands.entity(entity).remove::<Process>();
                        continue;
                    }

                    // use the closest 64 blocks as the goals

                    block_positions_and_distances.sort_by_key(|(_, distance)| *distance);
                    let mut goals = Vec::new();
                    for (block_pos, _) in block_positions_and_distances.into_iter().take(64) {
                        goals.push(ReachBlockPosGoal {
                            pos: block_pos,
                            chunk_storage: chunk_storage.clone(),
                        });
                    }

                    let reach_blocks_goal = OrGoals(goals);

                    println!("reaching for block");

                    Arc::new(reach_blocks_goal)
                } else {
                    println!("reaching for box because we're at {player_position}");

                    let reach_box_goal = ReachBoxGoal {
                        bb: bb.clone(),
                        chunk_storage: chunk_storage.clone(),
                    };

                    Arc::new(reach_box_goal)
                };

                goto_events.send(GotoEvent {
                    entity,
                    goal,
                    successors_fn: pathfinder::moves::default_move,
                    allow_mining: true,
                });
            }
        }
    }
}
