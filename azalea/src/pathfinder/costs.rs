use std::sync::LazyLock;

use num_traits::Float;

// based on https://github.com/cabaletta/baritone/blob/1.20.1/src/api/java/baritone/api/pathing/movement/ActionCosts.java
pub const WALK_ONE_BLOCK_COST: f32 = 20. / 4.317; // 4.633
pub const SPRINT_ONE_BLOCK_COST: f32 = 20. / 5.612; // 3.564
pub const WALK_OFF_BLOCK_COST: f32 = WALK_ONE_BLOCK_COST * 0.8; // 3.706
pub const SPRINT_MULTIPLIER: f32 = SPRINT_ONE_BLOCK_COST / WALK_ONE_BLOCK_COST; // 0.769
pub const JUMP_PENALTY: f32 = 2.;
pub const ENTER_WATER_PENALTY: f32 = 3.;
pub const CENTER_AFTER_FALL_COST: f32 = WALK_ONE_BLOCK_COST - WALK_OFF_BLOCK_COST; // 0.927
pub const WALK_ONE_IN_WATER_COST: f32 = 20. / 1.960; // 10.204

// explanation here:
// https://github.com/cabaletta/baritone/blob/f147519a5c291015d4f18c94558a3f1bdcdb9588/src/api/java/baritone/api/Settings.java#L405
// it's basically a multiplier used by some heuristics to convert x and z
// distance to ticks
pub const COST_HEURISTIC: f32 = 3.563;

// this one is also from baritone, it's helpful as a tiebreaker to avoid
// breaking blocks if it can be avoided
pub const BLOCK_BREAK_ADDITIONAL_PENALTY: f32 = 2.;

pub static FALL_1_25_BLOCKS_COST: LazyLock<f32> = LazyLock::new(|| distance_to_ticks(1.25));
pub static FALL_0_25_BLOCKS_COST: LazyLock<f32> = LazyLock::new(|| distance_to_ticks(0.25));
pub static JUMP_ONE_BLOCK_COST: LazyLock<f32> =
    LazyLock::new(|| *FALL_1_25_BLOCKS_COST - *FALL_0_25_BLOCKS_COST); // 3.163

// [0, 5.614727, 7.7880826, 9.468678, ..]
pub static FALL_N_BLOCKS_COST: LazyLock<[f32; 4097]> = LazyLock::new(|| {
    // mostly the same as calculating distance_to_ticks for every distance, but in
    // linear time complexity

    let mut fall_n_blocks_cost = [0.; 4097];
    let mut last_distance_blocks = 0;
    let mut current_distance = 0.;
    let mut tick_count = 0;

    'outer: loop {
        let fall_distance_per_tick = velocity(tick_count);

        let current_distance_blocks = (current_distance + fall_distance_per_tick) as usize;
        if current_distance_blocks > last_distance_blocks {
            for blocks in (last_distance_blocks + 1)..=current_distance_blocks {
                if blocks == fall_n_blocks_cost.len() {
                    break 'outer;
                }

                fall_n_blocks_cost[blocks] =
                    tick_count as f32 + (blocks as f32 - current_distance) / fall_distance_per_tick;
            }

            last_distance_blocks = current_distance_blocks;
        }

        current_distance += fall_distance_per_tick;
        tick_count += 1;
    }

    fall_n_blocks_cost
});

fn velocity(ticks: u32) -> f32 {
    (0.98.powi(ticks as i32) - 1.) * -3.92
}

fn distance_to_ticks(distance: f32) -> f32 {
    if distance == 0. {
        return 0.;
    }
    let mut tick_count = 0;
    let mut remaining_distance = distance;
    loop {
        let fall_distance_per_tick = velocity(tick_count);
        if fall_distance_per_tick >= remaining_distance {
            // add a bit extra to prefer smaller falls even if they're the same number of
            // ticks
            return (tick_count as f32) + (remaining_distance / fall_distance_per_tick);
        }
        remaining_distance -= fall_distance_per_tick;
        tick_count += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::pathfinder::costs::{FALL_N_BLOCKS_COST, distance_to_ticks};

    #[test]
    fn test_fall_n_blocks_cost() {
        for i in 0..4 {
            let a = FALL_N_BLOCKS_COST[i];
            let b = distance_to_ticks(i as f32);
            assert!((a - b).abs() < 0.1, "{a} != {b}");
        }
    }
}
