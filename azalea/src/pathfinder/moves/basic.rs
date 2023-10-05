use std::f32::consts::SQRT_2;

use azalea_client::{SprintDirection, StartSprintEvent, StartWalkEvent, WalkDirection};
use azalea_core::{
    direction::CardinalDirection,
    position::{BlockPos, Vec3},
};

use crate::{
    pathfinder::{astar, costs::*},
    JumpEvent, LookAtEvent,
};

use super::{default_is_reached, Edge, ExecuteCtx, IsReachedCtx, MoveData, PathfinderCtx};

pub fn basic_move(edges: &mut Vec<Edge>, ctx: &PathfinderCtx, node: BlockPos) {
    forward_move(edges, ctx, node);
    ascend_move(edges, ctx, node);
    descend_move(edges, ctx, node);
    diagonal_move(edges, ctx, node);
}

fn forward_move(edges: &mut Vec<Edge>, ctx: &PathfinderCtx, pos: BlockPos) {
    for dir in CardinalDirection::iter() {
        let offset = BlockPos::new(dir.x(), 0, dir.z());

        if !ctx.is_standable(pos + offset) {
            continue;
        }

        let cost = SPRINT_ONE_BLOCK_COST;

        edges.push(Edge {
            movement: astar::Movement {
                target: pos + offset,
                data: MoveData {
                    execute: &execute_forward_move,
                    is_reached: &default_is_reached,
                },
            },
            cost,
        })
    }
}

fn execute_forward_move(
    ExecuteCtx {
        entity,
        target,
        look_at_events,
        sprint_events,
        ..
    }: ExecuteCtx,
) {
    let center = target.center();
    look_at_events.send(LookAtEvent {
        entity,
        position: center,
    });
    sprint_events.send(StartSprintEvent {
        entity,
        direction: SprintDirection::Forward,
    });
}

fn ascend_move(edges: &mut Vec<Edge>, ctx: &PathfinderCtx, pos: BlockPos) {
    for dir in CardinalDirection::iter() {
        let offset = BlockPos::new(dir.x(), 1, dir.z());

        if !ctx.is_block_passable(pos.up(2)) {
            continue;
        }
        if !ctx.is_standable(pos + offset) {
            continue;
        }

        let cost = SPRINT_ONE_BLOCK_COST + JUMP_PENALTY + *JUMP_ONE_BLOCK_COST;

        edges.push(Edge {
            movement: astar::Movement {
                target: pos + offset,
                data: MoveData {
                    execute: &execute_ascend_move,
                    is_reached: &ascend_is_reached,
                },
            },
            cost,
        })
    }
}
fn execute_ascend_move(
    ExecuteCtx {
        entity,
        position,
        target,
        start,
        look_at_events,
        walk_events,
        jump_events,
        physics,
        ..
    }: ExecuteCtx,
) {
    let target_center = target.center();

    look_at_events.send(LookAtEvent {
        entity,
        position: target_center,
    });
    walk_events.send(StartWalkEvent {
        entity,
        direction: WalkDirection::Forward,
    });

    // these checks are to make sure we don't fall if our velocity is too high in
    // the wrong direction

    let x_axis = (start.x - target.x).abs(); // either 0 or 1
    let z_axis = (start.z - target.z).abs(); // either 0 or 1

    let flat_distance_to_next = x_axis as f64 * (target_center.x - position.x)
        + z_axis as f64 * (target_center.z - position.z);
    let side_distance = z_axis as f64 * (target_center.x - position.x).abs()
        + x_axis as f64 * (target_center.z - position.z).abs();

    let lateral_motion = x_axis as f64 * physics.delta.z + z_axis as f64 * physics.delta.x;
    if lateral_motion > 0.1 {
        return;
    }

    if flat_distance_to_next > 1.2 || side_distance > 0.2 {
        return;
    }

    if BlockPos::from(position) == start {
        jump_events.send(JumpEvent { entity });
    }
}
#[must_use]
pub fn ascend_is_reached(
    IsReachedCtx {
        position, target, ..
    }: IsReachedCtx,
) -> bool {
    BlockPos::from(position) == target || BlockPos::from(position) == target.down(1)
}

fn descend_move(edges: &mut Vec<Edge>, ctx: &PathfinderCtx, pos: BlockPos) {
    for dir in CardinalDirection::iter() {
        let dir_delta = BlockPos::new(dir.x(), 0, dir.z());
        let new_horizontal_position = pos + dir_delta;
        let fall_distance = ctx.fall_distance(new_horizontal_position);
        if fall_distance == 0 || fall_distance > 3 {
            continue;
        }
        let new_position = new_horizontal_position.down(fall_distance as i32);

        // check whether 3 blocks vertically forward are passable
        if !ctx.is_passable(new_horizontal_position) {
            continue;
        }
        // check whether we can stand on the target position
        if !ctx.is_standable(new_position) {
            continue;
        }

        let cost = SPRINT_ONE_BLOCK_COST
            + WALK_OFF_BLOCK_COST
            + FALL_ONE_BLOCK_COST * fall_distance as f32;

        edges.push(Edge {
            movement: astar::Movement {
                target: new_position,
                data: MoveData {
                    execute: &execute_descend_move,
                    is_reached: &descend_is_reached,
                },
            },
            cost,
        })
    }
}
fn execute_descend_move(
    ExecuteCtx {
        entity,
        target,
        start,
        look_at_events,
        walk_events,
        position,
        ..
    }: ExecuteCtx,
) {
    let start_center = start.center();
    let center = target.center();
    let horizontal_distance_from_target = (center - position).horizontal_distance_sqr().sqrt();
    let horizontal_distance_from_start =
        (start.center() - position).horizontal_distance_sqr().sqrt();

    let dest_ahead = Vec3::new(
        start_center.x + (center.x - start_center.x) * 1.5,
        center.y,
        start_center.z + (center.z - start_center.z) * 1.5,
    );

    if BlockPos::from(position) != target || horizontal_distance_from_target > 0.25 {
        if horizontal_distance_from_start < 1.25 {
            // this basically just exists to avoid doing spins while we're falling
            look_at_events.send(LookAtEvent {
                entity,
                position: dest_ahead,
            });
            walk_events.send(StartWalkEvent {
                entity,
                direction: WalkDirection::Forward,
            });
        } else {
            look_at_events.send(LookAtEvent {
                entity,
                position: center,
            });
            walk_events.send(StartWalkEvent {
                entity,
                direction: WalkDirection::Forward,
            });
        }
    } else {
        walk_events.send(StartWalkEvent {
            entity,
            direction: WalkDirection::None,
        });
    }
}

#[must_use]
pub fn descend_is_reached(
    IsReachedCtx {
        target,
        start,
        position,
        ..
    }: IsReachedCtx,
) -> bool {
    let dest_ahead = BlockPos::new(
        start.x + (target.x - start.x) * 2,
        target.y,
        start.z + (target.z - start.z) * 2,
    );

    (BlockPos::from(position) == target || BlockPos::from(position) == dest_ahead)
        && (position.y - target.y as f64) < 0.5
}

fn diagonal_move(edges: &mut Vec<Edge>, ctx: &PathfinderCtx, pos: BlockPos) {
    for dir in CardinalDirection::iter() {
        let right = dir.right();
        let offset = BlockPos::new(dir.x() + right.x(), 0, dir.z() + right.z());

        if !ctx.is_passable(BlockPos::new(pos.x + dir.x(), pos.y, pos.z + dir.z()))
            && !ctx.is_passable(BlockPos::new(
                pos.x + dir.right().x(),
                pos.y,
                pos.z + dir.right().z(),
            ))
        {
            continue;
        }
        if !ctx.is_standable(pos + offset) {
            continue;
        }
        // +0.001 so it doesn't unnecessarily go diagonal sometimes
        let cost = SPRINT_ONE_BLOCK_COST * SQRT_2 + 0.001;

        edges.push(Edge {
            movement: astar::Movement {
                target: pos + offset,
                data: MoveData {
                    execute: &execute_diagonal_move,
                    is_reached: &default_is_reached,
                },
            },
            cost,
        })
    }
}
fn execute_diagonal_move(
    ExecuteCtx {
        entity,
        target,
        look_at_events,
        sprint_events,
        ..
    }: ExecuteCtx,
) {
    let center = target.center();
    look_at_events.send(LookAtEvent {
        entity,
        position: center,
    });
    sprint_events.send(StartSprintEvent {
        entity,
        direction: SprintDirection::Forward,
    });
}
