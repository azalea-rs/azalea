use azalea_client::{SprintDirection, WalkDirection};
use azalea_core::{direction::CardinalDirection, position::BlockPos};
use tracing::trace;

use super::{Edge, ExecuteCtx, IsReachedCtx, MoveData, PathfinderCtx};
use crate::pathfinder::{astar, costs::*, rel_block_pos::RelBlockPos};

pub fn parkour_move(ctx: &mut PathfinderCtx, node: RelBlockPos) {
    parkour_forward_1_move(ctx, node);
    parkour_forward_2_move(ctx, node);
    parkour_forward_3_move(ctx, node);
}

fn parkour_forward_1_move(ctx: &mut PathfinderCtx, pos: RelBlockPos) {
    for dir in CardinalDirection::iter() {
        let gap_offset = RelBlockPos::new(dir.x(), 0, dir.z());
        let offset = RelBlockPos::new(dir.x() * 2, 0, dir.z() * 2);

        // make sure we actually have to jump
        if ctx.world.is_block_solid((pos + gap_offset).down(1)) {
            continue;
        }
        if !ctx.world.is_passable(pos + gap_offset) {
            continue;
        }

        let ascend: i32 = if ctx.world.is_standable(pos + offset.up(1)) {
            // ascend
            1
        } else if ctx.world.is_standable(pos + offset) {
            // forward
            0
        } else {
            continue;
        };

        // make sure we have space to jump
        if !ctx.world.is_block_passable((pos + gap_offset).up(2)) {
            continue;
        }

        // make sure there's not a block above us
        if !ctx.world.is_block_passable(pos.up(2)) {
            continue;
        }
        // make sure there's not a block above the target
        if !ctx.world.is_block_passable((pos + offset).up(2)) {
            continue;
        }

        let cost = JUMP_PENALTY + WALK_ONE_BLOCK_COST * 2. + CENTER_AFTER_FALL_COST;

        ctx.edges.push(Edge {
            movement: astar::Movement {
                target: pos + offset.up(ascend),
                data: MoveData {
                    execute: &execute_parkour_move,
                    is_reached: &parkour_is_reached,
                },
            },
            cost,
        })
    }
}

fn parkour_forward_2_move(ctx: &mut PathfinderCtx, pos: RelBlockPos) {
    'dir: for dir in CardinalDirection::iter() {
        let gap_1_offset = RelBlockPos::new(dir.x(), 0, dir.z());
        let gap_2_offset = RelBlockPos::new(dir.x() * 2, 0, dir.z() * 2);
        let offset = RelBlockPos::new(dir.x() * 3, 0, dir.z() * 3);

        // make sure we actually have to jump
        if ctx.world.is_block_solid((pos + gap_1_offset).down(1))
            || ctx.world.is_block_solid((pos + gap_2_offset).down(1))
        {
            continue;
        }

        let ascend: i32 = if ctx.world.is_standable(pos + offset.up(1)) {
            1
        } else if ctx.world.is_standable(pos + offset) {
            0
        } else {
            continue;
        };

        // make sure we have space to jump
        for offset in [gap_1_offset, gap_2_offset] {
            if !ctx.world.is_passable(pos + offset) {
                continue 'dir;
            }
            if !ctx.world.is_block_passable((pos + offset).up(2)) {
                continue 'dir;
            }
        }
        // make sure there's not a block above us
        if !ctx.world.is_block_passable(pos.up(2)) {
            continue;
        }
        // make sure there's not a block above the target
        if !ctx.world.is_block_passable((pos + offset).up(2)) {
            continue;
        }

        let cost = JUMP_PENALTY + WALK_ONE_BLOCK_COST * 3. + CENTER_AFTER_FALL_COST;

        ctx.edges.push(Edge {
            movement: astar::Movement {
                target: pos + offset.up(ascend),
                data: MoveData {
                    execute: &execute_parkour_move,
                    is_reached: &parkour_is_reached,
                },
            },
            cost,
        })
    }
}

fn parkour_forward_3_move(ctx: &mut PathfinderCtx, pos: RelBlockPos) {
    'dir: for dir in CardinalDirection::iter() {
        let gap_1_offset = RelBlockPos::new(dir.x(), 0, dir.z());
        let gap_2_offset = RelBlockPos::new(dir.x() * 2, 0, dir.z() * 2);
        let gap_3_offset = RelBlockPos::new(dir.x() * 3, 0, dir.z() * 3);
        let offset = RelBlockPos::new(dir.x() * 4, 0, dir.z() * 4);

        // make sure we actually have to jump
        if ctx.world.is_block_solid((pos + gap_1_offset).down(1))
            || ctx.world.is_block_solid((pos + gap_2_offset).down(1))
            || ctx.world.is_block_solid((pos + gap_3_offset).down(1))
        {
            continue;
        }

        if !ctx.world.is_standable(pos + offset) {
            continue;
        };

        // make sure we have space to jump
        for offset in [gap_1_offset, gap_2_offset, gap_3_offset] {
            if !ctx.world.is_passable(pos + offset) {
                continue 'dir;
            }
            if !ctx.world.is_block_passable((pos + offset).up(2)) {
                continue 'dir;
            }
        }
        // make sure there's not a block above us
        if !ctx.world.is_block_passable(pos.up(2)) {
            continue;
        }
        // make sure there's not a block above the target
        if !ctx.world.is_block_passable((pos + offset).up(2)) {
            continue;
        }

        let cost = JUMP_PENALTY + SPRINT_ONE_BLOCK_COST * 4. + CENTER_AFTER_FALL_COST;

        ctx.edges.push(Edge {
            movement: astar::Movement {
                target: pos + offset,
                data: MoveData {
                    execute: &execute_parkour_move,
                    is_reached: &parkour_is_reached,
                },
            },
            cost,
        })
    }
}

fn execute_parkour_move(mut ctx: ExecuteCtx) {
    let ExecuteCtx {
        position,
        target,
        start,
        ..
    } = ctx;

    let start_center = start.center();
    let target_center = target.center();

    let jump_distance = i32::max((target - start).x.abs(), (target - start).z.abs());

    let ascend: i32 = target.y - start.y;

    if jump_distance >= 4 || (ascend > 0 && jump_distance >= 3) {
        // 3 block gap OR 2 block gap with ascend
        ctx.sprint(SprintDirection::Forward);
    } else {
        ctx.walk(WalkDirection::Forward);
    }

    let x_dir = (target.x - start.x).clamp(-1, 1);
    let z_dir = (target.z - start.z).clamp(-1, 1);
    let dir = BlockPos::new(x_dir, 0, z_dir);
    let jump_at_pos = start + dir;

    let is_at_start_block = BlockPos::from(position) == start;
    let is_at_jump_block = BlockPos::from(position) == jump_at_pos;

    let required_distance_from_center = if jump_distance <= 2 {
        // 1 block gap
        0.0
    } else {
        0.6
    };
    let distance_from_start = f64::max(
        (position.x - start_center.x).abs(),
        (position.z - start_center.z).abs(),
    );

    if !is_at_start_block
        && !is_at_jump_block
        && (position.y - start.y as f64) < 0.094
        && distance_from_start < 0.85
    {
        // we have to be on the start block to jump
        ctx.look_at(start_center);
        trace!("looking at start_center");
    } else {
        ctx.look_at(target_center);
        trace!("looking at target_center");
    }

    if !is_at_start_block && is_at_jump_block && distance_from_start > required_distance_from_center
    {
        ctx.jump();
    }
}

#[must_use]
pub fn parkour_is_reached(
    IsReachedCtx {
        position, target, ..
    }: IsReachedCtx,
) -> bool {
    // 0.094 and not 0 for lilypads
    BlockPos::from(position) == target && (position.y - target.y as f64) < 0.094
}
