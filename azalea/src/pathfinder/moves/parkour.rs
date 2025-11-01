use azalea_client::{SprintDirection, WalkDirection};

use super::{Edge, ExecuteCtx, IsReachedCtx, MoveData, PathfinderCtx};
use crate::pathfinder::{astar, costs::*, player_pos_to_block_pos, rel_block_pos::RelBlockPos};

pub fn parkour_move(ctx: &mut PathfinderCtx, node: RelBlockPos) {
    if !ctx.world.is_block_solid(node.down(1)) {
        return;
    }

    let distance = 5;

    for dx in -distance..=distance {
        for dz in -distance..=distance {
            if ((-1..=1).contains(&dx) && (-1..=1).contains(&dz))
                || dx * dx + dz * dz > distance * distance
            {
                continue;
            }

            parkour_direction_move(ctx, node, dx, dz, dx.abs().max(dz.abs()));
        }
    }
}

fn parkour_direction_move(
    ctx: &mut PathfinderCtx,
    pos: RelBlockPos,
    dx: i16,
    dz: i16,
    distance: i16,
) {
    let target_offset = RelBlockPos::new(dx, 0, dz);
    let target_pos = pos + target_offset;

    if !are_gaps_valid(ctx, pos, dx, dz)
        || (!ctx.world.is_block_passable(pos.up(2))
            || !ctx.world.is_block_passable(target_pos.up(2)))
    {
        return;
    }

    if let Some((target, cost)) = find_landing_position(ctx, target_pos, distance - 1) {
        ctx.edges.push(Edge {
            movement: astar::Movement {
                target,
                data: MoveData {
                    execute: &execute_parkour_move,
                    is_reached: &parkour_is_reached,
                },
            },
            cost,
        });
    }
}

fn are_gaps_valid(ctx: &mut PathfinderCtx, pos: RelBlockPos, dx: i16, dz: i16) -> bool {
    let line = get_line_bresenham(0, 0, dx, dz);

    line.iter()
        .enumerate()
        .skip(1)
        .take(line.len().saturating_sub(2))
        .all(|(_, &(x, z))| {
            let gap_pos = pos + RelBlockPos::new(x, 0, z);
            !ctx.world.is_block_solid(gap_pos.down(1))
                && ctx.world.is_passable(gap_pos)
                && ctx.world.is_block_passable(gap_pos.up(2))
        })
}

fn get_line_bresenham(x0: i16, y0: i16, x1: i16, y1: i16) -> Vec<(i16, i16)> {
    let mut points = Vec::new();
    let (dx, dy) = ((x1 - x0).abs(), (y1 - y0).abs());
    let (sx, sy) = (x0.cmp(&x1).reverse() as i16, y0.cmp(&y1).reverse() as i16);
    let mut err = dx - dy;
    let (mut x, mut y) = (x0, y0);

    loop {
        points.push((x, y));
        if x == x1 && y == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }
    points
}

fn find_landing_position(
    ctx: &mut PathfinderCtx,
    base_target: RelBlockPos,
    distance: i16,
) -> Option<(RelBlockPos, f32)> {
    generate_height_checks(distance)
        .into_iter()
        .find_map(|(height_offset, fall_cost)| {
            let target = base_target.up(height_offset);
            if ctx.world.is_standable(target) {
                let movement_cost = if distance >= 4 {
                    SPRINT_ONE_BLOCK_COST
                } else {
                    WALK_ONE_BLOCK_COST
                };
                let actual_distance =
                    ((base_target.x as f32).powi(2) + (base_target.z as f32).powi(2)).sqrt();
                let cost = JUMP_PENALTY
                    + movement_cost * actual_distance
                    + fall_cost
                    + CENTER_AFTER_FALL_COST;
                Some((target, cost))
            } else {
                None
            }
        })
}

fn generate_height_checks(distance: i16) -> Vec<(i32, f32)> {
    match distance {
        2 => vec![(1, 0.0), (0, 0.0)],
        3 => vec![
            (1, 0.0),
            (0, FALL_N_BLOCKS_COST[1]),
            (-1, FALL_N_BLOCKS_COST[2]),
        ],
        _ => {
            let mut checks = Vec::with_capacity(5);
            if distance <= 3 {
                checks.push((1, 0.0));
            }
            checks.push((0, FALL_N_BLOCKS_COST[1]));
            checks.extend((1..=3).map(|h| (-(h as i32), FALL_N_BLOCKS_COST[h])));
            checks
        }
    }
}

fn execute_parkour_move(mut ctx: ExecuteCtx) {
    let delta = ctx.target - ctx.start;
    let jump_distance = (delta.x as f64).hypot(delta.z as f64);
    
    if jump_distance >= 3.0 {
        ctx.sprint(SprintDirection::Forward);
    } else {
        ctx.walk(WalkDirection::Forward);
    }

    let should_jump = [(delta.x, ctx.start.x, ctx.position.x), (delta.z, ctx.start.z, ctx.position.z)]
        .iter()
        .any(|&(d, start, pos)| {
            if d == 0 { return false }
            
            let edge = if d > 0 { start + 1 } else { start } as f64;
            (d > 0 && pos >= edge) || (d < 0 && pos <= edge)
        });

    ctx.look_at(ctx.target.center());
    
    if should_jump {
        ctx.jump();
    }
}

#[must_use]
pub fn parkour_is_reached(ctx: IsReachedCtx) -> bool {
    player_pos_to_block_pos(ctx.position) == ctx.target
        && (ctx.position.y - (ctx.target.y as f64) < 0.094 || ctx.physics.on_ground())
}
