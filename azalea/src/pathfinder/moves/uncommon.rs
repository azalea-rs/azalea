//! Some moves which aren't used often but make execution slightly cleaner.

use azalea_core::direction::CardinalDirection;

use crate::pathfinder::{
    astar::{self, Edge},
    costs::{CENTER_AFTER_FALL_COST, FALL_N_BLOCKS_COST, WALK_OFF_BLOCK_COST, WALK_ONE_BLOCK_COST},
    moves::{
        BARITONE_COMPAT, MoveData, MovesCtx,
        basic::{descend_is_reached, execute_descend_move},
    },
    positions::RelBlockPos,
};

pub fn uncommon_move(ctx: &mut MovesCtx, node: RelBlockPos) {
    if BARITONE_COMPAT {
        return;
    }
    descend_forward_1_move(ctx, node);
}

pub fn descend_forward_1_move(ctx: &mut MovesCtx, pos: RelBlockPos) {
    for dir in CardinalDirection::iter() {
        let dir_delta = RelBlockPos::new(dir.x(), 0, dir.z());
        let gap_horizontal_position = pos + dir_delta;
        let new_horizontal_position = pos + dir_delta * 2;

        let gap_fall_distance = ctx.world.fall_distance(gap_horizontal_position);
        let fall_distance = ctx.world.fall_distance(new_horizontal_position);

        if fall_distance == 0 || fall_distance > 3 || gap_fall_distance < fall_distance {
            continue;
        }

        let new_position = new_horizontal_position.down(fall_distance as i32);

        // check whether 2 blocks vertically forward are passable
        if !ctx.world.is_passable(new_horizontal_position) {
            continue;
        }
        if !ctx.world.is_passable(gap_horizontal_position) {
            continue;
        }
        // check whether we can stand on the target position
        if !ctx.world.is_standable(new_position) {
            continue;
        }

        let cost = WALK_OFF_BLOCK_COST
            + WALK_ONE_BLOCK_COST
            + f32::max(
                FALL_N_BLOCKS_COST
                    .get(fall_distance as usize)
                    .copied()
                    // avoid panicking if we fall more than the size of FALL_N_BLOCKS_COST
                    // probably not possible but just in case
                    .unwrap_or(f32::INFINITY),
                CENTER_AFTER_FALL_COST,
            );

        ctx.edges.push(Edge {
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
