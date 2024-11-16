//! The goals that a pathfinder can try to reach.

use azalea_core::position::{BlockPos, Vec3};
use azalea_world::ChunkStorage;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::f32::consts::SQRT_2;

use super::costs::{COST_HEURISTIC, FALL_N_BLOCKS_COST, JUMP_ONE_BLOCK_COST};

pub trait Goal {
    #[must_use]
    fn heuristic(&self, n: BlockPos) -> f32;
    #[must_use]
    fn success(&self, n: BlockPos) -> bool;
}

/// Move to the given block position.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct BlockPosGoal(pub BlockPos);
impl Goal for BlockPosGoal {
    fn heuristic(&self, n: BlockPos) -> f32 {
        let dx = (self.0.x - n.x) as f32;
        let dy = (self.0.y - n.y) as f32;
        let dz = (self.0.z - n.z) as f32;

        xz_heuristic(dx, dz) + y_heuristic(dy)
    }
    fn success(&self, n: BlockPos) -> bool {
        n == self.0
    }
}

fn xz_heuristic(dx: f32, dz: f32) -> f32 {
    let x = dx.abs();
    let z = dz.abs();

    let diagonal;
    let straight;

    if x < z {
        straight = z - x;
        diagonal = x;
    } else {
        straight = x - z;
        diagonal = z;
    }

    (diagonal * SQRT_2 + straight) * COST_HEURISTIC
}

/// Move to the given block position, ignoring the y-axis.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct XZGoal {
    pub x: i32,
    pub z: i32,
}
impl Goal for XZGoal {
    fn heuristic(&self, n: BlockPos) -> f32 {
        let dx = (self.x - n.x) as f32;
        let dz = (self.z - n.z) as f32;
        xz_heuristic(dx, dz)
    }
    fn success(&self, n: BlockPos) -> bool {
        n.x == self.x && n.z == self.z
    }
}

fn y_heuristic(dy: f32) -> f32 {
    if dy > 0.0 {
        *JUMP_ONE_BLOCK_COST * dy
    } else {
        FALL_N_BLOCKS_COST[2] / 2. * -dy
    }
}

/// Move to the given y coordinate.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct YGoal {
    pub y: i32,
}
impl Goal for YGoal {
    fn heuristic(&self, n: BlockPos) -> f32 {
        let dy = (self.y - n.y) as f32;
        y_heuristic(dy)
    }
    fn success(&self, n: BlockPos) -> bool {
        n.y == self.y
    }
}

/// Get within the given radius of the given position.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct RadiusGoal {
    pub pos: Vec3,
    pub radius: f32,
}
impl Goal for RadiusGoal {
    fn heuristic(&self, n: BlockPos) -> f32 {
        let n = n.center();
        let dx = (self.pos.x - n.x) as f32;
        let dy = (self.pos.y - n.y) as f32;
        let dz = (self.pos.z - n.z) as f32;
        dx * dx + dy * dy + dz * dz
    }
    fn success(&self, n: BlockPos) -> bool {
        let n = n.center();
        let dx = (self.pos.x - n.x) as f32;
        let dy = (self.pos.y - n.y) as f32;
        let dz = (self.pos.z - n.z) as f32;
        dx * dx + dy * dy + dz * dz <= self.radius * self.radius
    }
}

/// Do the opposite of the given goal.
#[derive(Debug)]
pub struct InverseGoal<T: Goal>(pub T);
impl<T: Goal> Goal for InverseGoal<T> {
    fn heuristic(&self, n: BlockPos) -> f32 {
        -self.0.heuristic(n)
    }
    fn success(&self, n: BlockPos) -> bool {
        !self.0.success(n)
    }
}

/// Do either of the given goals, whichever is closer.
#[derive(Debug)]
pub struct OrGoal<T: Goal, U: Goal>(pub T, pub U);
impl<T: Goal, U: Goal> Goal for OrGoal<T, U> {
    fn heuristic(&self, n: BlockPos) -> f32 {
        self.0.heuristic(n).min(self.1.heuristic(n))
    }
    fn success(&self, n: BlockPos) -> bool {
        self.0.success(n) || self.1.success(n)
    }
}

/// Do any of the given goals, whichever is closest.
#[derive(Debug)]
pub struct OrGoals<T: Goal>(pub Vec<T>);
impl<T: Goal> Goal for OrGoals<T> {
    fn heuristic(&self, n: BlockPos) -> f32 {
        self.0
            .iter()
            .map(|goal| goal.heuristic(n))
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(f32::INFINITY)
    }
    fn success(&self, n: BlockPos) -> bool {
        self.0.iter().any(|goal| goal.success(n))
    }
}

/// Try to reach both of the given goals.
#[derive(Debug)]
pub struct AndGoal<T: Goal, U: Goal>(pub T, pub U);
impl<T: Goal, U: Goal> Goal for AndGoal<T, U> {
    fn heuristic(&self, n: BlockPos) -> f32 {
        self.0.heuristic(n).max(self.1.heuristic(n))
    }
    fn success(&self, n: BlockPos) -> bool {
        self.0.success(n) && self.1.success(n)
    }
}

/// Try to reach all the given goals.
#[derive(Debug)]
pub struct AndGoals<T: Goal>(pub Vec<T>);
impl<T: Goal> Goal for AndGoals<T> {
    fn heuristic(&self, n: BlockPos) -> f32 {
        self.0
            .iter()
            .map(|goal| goal.heuristic(n))
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(f32::INFINITY)
    }
    fn success(&self, n: BlockPos) -> bool {
        self.0.iter().all(|goal| goal.success(n))
    }
}

/// Move to a position where we can reach the given block.
#[derive(Clone, Debug)]
pub struct ReachBlockPosGoal {
    pub pos: BlockPos,
    pub chunk_storage: ChunkStorage,
}
impl Goal for ReachBlockPosGoal {
    fn heuristic(&self, n: BlockPos) -> f32 {
        BlockPosGoal(self.pos).heuristic(n)
    }
    fn success(&self, n: BlockPos) -> bool {
        // only do the expensive check if we're close enough
        let max_pick_range = 6;
        let actual_pick_range = 4.5;

        let distance = (self.pos - n).length_sqr();
        if distance > max_pick_range * max_pick_range {
            return false;
        }

        let eye_position = n.to_vec3_floored() + Vec3::new(0.5, 1.62, 0.5);
        let look_direction = crate::direction_looking_at(&eye_position, &self.pos.center());
        let block_hit_result = azalea_client::interact::pick(
            &look_direction,
            &eye_position,
            &self.chunk_storage,
            actual_pick_range,
        );

        block_hit_result.block_pos == self.pos
    }
}
