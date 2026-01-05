use std::sync::LazyLock;

use num_traits::Float;

// based on https://github.com/cabaletta/baritone/blob/1.20.1/src/api/java/baritone/api/pathing/movement/ActionCosts.java
pub const WALK_ONE_BLOCK_COST: f32 = 20. / 4.317; // 4.633
pub const SPRINT_ONE_BLOCK_COST: f32 = 20. / 5.612; // 3.564
pub const WALK_OFF_BLOCK_COST: f32 = WALK_ONE_BLOCK_COST * 0.8;
pub const SPRINT_MULTIPLIER: f32 = SPRINT_ONE_BLOCK_COST / WALK_ONE_BLOCK_COST;
pub const JUMP_PENALTY: f32 = 2.;
pub const CENTER_AFTER_FALL_COST: f32 = WALK_ONE_BLOCK_COST - WALK_OFF_BLOCK_COST; // 0.927
pub const WALK_ONE_IN_WATER_COST: f32 = 20. / 1.960; // 10.204

// explanation here:
// https://github.com/cabaletta/baritone/blob/f147519a5c291015d4f18c94558a3f1bdcdb9588/src/api/java/baritone/api/Settings.java#L405
// it's basically just the heuristic multiplier
pub const COST_HEURISTIC: f32 = 3.563;

// this one is also from baritone, it's helpful as a tiebreaker to avoid
// breaking blocks if it can be avoided
pub const BLOCK_BREAK_ADDITIONAL_PENALTY: f32 = 2.;

pub static FALL_1_25_BLOCKS_COST: LazyLock<f32> = LazyLock::new(|| distance_to_ticks(1.25));
pub static FALL_0_25_BLOCKS_COST: LazyLock<f32> = LazyLock::new(|| distance_to_ticks(0.25));
pub static JUMP_ONE_BLOCK_COST: LazyLock<f32> =
    LazyLock::new(|| *FALL_1_25_BLOCKS_COST - *FALL_0_25_BLOCKS_COST); // 3.163

pub static FALL_N_BLOCKS_COST: LazyLock<[f32; 4097]> = LazyLock::new(|| {
    let mut fall_n_blocks_cost = [0.; 4097];

    let mut distance = 0;

    // this is the same as calling distance_to_ticks a bunch of times but more
    // efficient
    let mut temp_distance = distance as f32;
    let mut tick_count = 0;
    loop {
        let fall_distance = velocity(tick_count);
        if temp_distance <= fall_distance {
            fall_n_blocks_cost[distance] = tick_count as f32 + temp_distance / fall_distance;
            distance += 1;
            if distance >= fall_n_blocks_cost.len() {
                break;
            }
        }
        temp_distance -= fall_distance;
        tick_count += 1;
    }

    fall_n_blocks_cost
});

fn velocity(ticks: usize) -> f32 {
    (0.98.powi(ticks.try_into().unwrap()) - 1.) * -3.92
}

fn distance_to_ticks(distance: f32) -> f32 {
    if distance == 0. {
        // Avoid 0/0 NaN
        return 0.;
    }
    let mut temp_distance = distance;
    let mut tick_count = 0;
    loop {
        let fall_distance = velocity(tick_count);
        if temp_distance <= fall_distance {
            return tick_count as f32 + temp_distance / fall_distance;
        }
        temp_distance -= fall_distance;
        tick_count += 1;
    }
}
