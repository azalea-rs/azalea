use azalea_client::{SprintDirection, StartSprintEvent, StartWalkEvent, WalkDirection};
use azalea_core::{BlockPos, CardinalDirection};
use azalea_world::Instance;

use crate::{
    pathfinder::{astar, costs::*},
    JumpEvent, LookAtEvent,
};

use super::{
    default_is_reached, is_block_passable, is_block_solid, is_passable, is_standable, Edge,
    ExecuteCtx, MoveData,
};

pub fn parkour_move(world: &Instance, node: BlockPos) -> Vec<Edge> {
    let mut edges = Vec::new();
    edges.extend(parkour_forward_1_move(world, node));
    edges.extend(parkour_headhitter_forward_1_move(world, node));
    edges.extend(parkour_forward_2_move(world, node));
    edges
}

fn parkour_forward_1_move(world: &Instance, pos: BlockPos) -> Vec<Edge> {
    let mut edges = Vec::new();
    for dir in CardinalDirection::iter() {
        let gap_offset = BlockPos::new(dir.x() * 1, 0, dir.z() * 1);
        let offset = BlockPos::new(dir.x() * 2, 0, dir.z() * 2);

        if !is_standable(&(pos + offset), world) {
            continue;
        }
        if !is_passable(&(pos + gap_offset), world) {
            continue;
        }
        if !is_block_passable(&(pos + gap_offset).up(2), world) {
            continue;
        }
        // make sure we actually have to jump
        if is_block_solid(&(pos + gap_offset).down(1), world) {
            continue;
        }
        // make sure it's not a headhitter
        if !is_block_passable(&pos.up(2), world) {
            continue;
        }

        let cost = *JUMP_ONE_BLOCK_COST + SPRINT_ONE_BLOCK_COST + SPRINT_ONE_BLOCK_COST;

        edges.push(Edge {
            movement: astar::Movement {
                target: pos + offset,
                data: MoveData {
                    execute: &execute_parkour_move,
                    is_reached: &default_is_reached,
                },
            },
            cost,
        })
    }

    edges
}

fn parkour_forward_2_move(world: &Instance, pos: BlockPos) -> Vec<Edge> {
    let mut edges = Vec::new();
    for dir in CardinalDirection::iter() {
        let gap_1_offset = BlockPos::new(dir.x() * 1, 0, dir.z() * 1);
        let gap_2_offset = BlockPos::new(dir.x() * 2, 0, dir.z() * 2);
        let offset = BlockPos::new(dir.x() * 3, 0, dir.z() * 3);

        if !is_standable(&(pos + offset), world) {
            continue;
        }
        if !is_passable(&(pos + gap_1_offset), world) {
            continue;
        }
        if !is_block_passable(&(pos + gap_1_offset).up(2), world) {
            continue;
        }
        if !is_passable(&(pos + gap_2_offset), world) {
            continue;
        }
        if !is_block_passable(&(pos + gap_2_offset).up(2), world) {
            continue;
        }
        // make sure we actually have to jump
        if is_block_solid(&(pos + gap_1_offset).down(1), world) {
            continue;
        }
        // make sure it's not a headhitter
        if !is_block_passable(&pos.up(2), world) {
            continue;
        }

        let cost = *JUMP_ONE_BLOCK_COST
            + SPRINT_ONE_BLOCK_COST
            + SPRINT_ONE_BLOCK_COST
            + SPRINT_ONE_BLOCK_COST;

        edges.push(Edge {
            movement: astar::Movement {
                target: pos + offset,
                data: MoveData {
                    execute: &execute_parkour_move,
                    is_reached: &default_is_reached,
                },
            },
            cost,
        })
    }

    edges
}

fn parkour_headhitter_forward_1_move(world: &Instance, pos: BlockPos) -> Vec<Edge> {
    let mut edges = Vec::new();
    for dir in CardinalDirection::iter() {
        let gap_offset = BlockPos::new(dir.x() * 1, 0, dir.z() * 1);
        let offset = BlockPos::new(dir.x() * 2, 0, dir.z() * 2);

        if !is_standable(&(pos + offset), world) {
            continue;
        }
        if !is_passable(&(pos + gap_offset), world) {
            continue;
        }
        if !is_block_passable(&(pos + gap_offset).up(2), world) {
            continue;
        }
        // make sure we actually have to jump
        if is_block_solid(&(pos + gap_offset).down(1), world) {
            continue;
        }
        // make sure it is a headhitter
        if !is_block_solid(&pos.up(2), world) {
            continue;
        }

        let cost = *JUMP_ONE_BLOCK_COST + WALK_ONE_BLOCK_COST + WALK_ONE_BLOCK_COST;

        edges.push(Edge {
            movement: astar::Movement {
                target: pos + offset,
                data: MoveData {
                    execute: &execute_headhitter_parkour_move,
                    is_reached: &default_is_reached,
                },
            },
            cost,
        })
    }

    edges
}

fn execute_parkour_move(
    ExecuteCtx {
        entity,
        target,
        start,
        look_at_events,
        sprint_events,
        walk_events,
        jump_events,
        ..
    }: ExecuteCtx,
) {
    let center = target.center();
    look_at_events.send(LookAtEvent {
        entity,
        position: center,
    });

    let jump_distance = i32::max((target - start).x.abs(), (target - start).z.abs());

    if jump_distance > 2 {
        sprint_events.send(StartSprintEvent {
            entity,
            direction: SprintDirection::Forward,
        });
    } else {
        walk_events.send(StartWalkEvent {
            entity,
            direction: WalkDirection::Forward,
        });
    }

    jump_events.send(JumpEvent { entity });
}

fn execute_headhitter_parkour_move(
    ExecuteCtx {
        entity,
        target,
        start,
        position,
        look_at_events,
        sprint_events,
        walk_events,
        jump_events,
        ..
    }: ExecuteCtx,
) {
    let center = target.center();
    look_at_events.send(LookAtEvent {
        entity,
        position: center,
    });

    let jump_distance = i32::max((target - start).x.abs(), (target - start).z.abs());

    if jump_distance > 2 {
        sprint_events.send(StartSprintEvent {
            entity,
            direction: SprintDirection::Forward,
        });
    } else {
        walk_events.send(StartWalkEvent {
            entity,
            direction: WalkDirection::Forward,
        });
    }

    let start_center = start.center();
    let distance_from_start = f64::max(
        (start_center.x as f64 - position.x).abs(),
        (start_center.z as f64 - position.z).abs(),
    );

    if distance_from_start > 0.75 {
        jump_events.send(JumpEvent { entity });
    }
}
