//! The goals that a pathfinder can try to reach.

use std::f32::consts::SQRT_2;

use azalea_core::position::{BlockPos, Vec3};
use azalea_world::ChunkStorage;

use crate::utils::get_hit_result_while_looking_at;

use super::{
    block_box::BlockBox,
    costs::{COST_HEURISTIC, FALL_N_BLOCKS_COST, JUMP_ONE_BLOCK_COST},
};

pub trait Goal: Send + Sync {
    #[must_use]
    fn heuristic(&self, n: BlockPos) -> f32;
    #[must_use]
    fn success(&self, n: BlockPos) -> bool;
}

/// Move to the given block position. This is the most commonly used goal.
#[derive(Debug)]
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

/// Move to the given block position, ignoring the y axis.
#[derive(Debug)]
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
#[derive(Debug)]
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
#[derive(Debug)]
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

/// Try to reach all of the given goals.
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
#[derive(Debug)]
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

        let distance = (self.pos - n).length_squared();
        if distance > max_pick_range * max_pick_range {
            return false;
        }

        let block_hit_result = get_hit_result_while_looking_at(&self.chunk_storage, n, self.pos);

        block_hit_result == self.pos
    }
}

/// Move to a position inside of the given box (inclusive, so the corners are
/// included in the box).
#[derive(Debug)]
pub struct BoxGoal(pub BlockBox);

impl Goal for BoxGoal {
    fn heuristic(&self, n: BlockPos) -> f32 {
        let dx = if n.x < self.0.min().x {
            self.0.min().x - n.x
        } else if n.x > self.0.max().x {
            n.x - self.0.max().x
        } else {
            0
        };
        let dy = if n.y < self.0.min().y {
            self.0.min().y - n.y
        } else if n.y > self.0.max().y {
            n.y - self.0.max().y
        } else {
            0
        };
        let dz = if n.z < self.0.min().z {
            self.0.min().z - n.z
        } else if n.z > self.0.max().z {
            n.z - self.0.max().z
        } else {
            0
        };

        xz_heuristic(dx as f32, dz as f32) + y_heuristic(dy as f32)
    }

    fn success(&self, n: BlockPos) -> bool {
        n.x >= self.0.min().x
            && n.x <= self.0.max().x
            && n.y >= self.0.min().y
            && n.y <= self.0.max().y
            && n.z >= self.0.min().z
            && n.z <= self.0.max().z
    }
}

/// Move to a position where we can reach at least one block from the given box.
/// This is usually used when digging out an area.
#[derive(Debug)]
pub struct ReachBoxGoal {
    pub bb: BlockBox,
    pub chunk_storage: ChunkStorage,
}
impl Goal for ReachBoxGoal {
    fn heuristic(&self, n: BlockPos) -> f32 {
        BoxGoal(self.bb.clone()).heuristic(n)
    }

    fn success(&self, n: BlockPos) -> bool {
        // succeed if we're already in the box
        if self.bb.contains(n) {
            return true;
        }

        // only do the expensive check if we're close enough
        let max_pick_range = 6;

        let distance = self.bb.distance_squared_to(n);
        if distance > max_pick_range * max_pick_range {
            return false;
        }

        // look at the closest block
        let look_target = self.bb.closest_block_pos(n);
        let hit_result = get_hit_result_while_looking_at(&self.chunk_storage, n, look_target);

        self.bb.contains(hit_result)
    }
}
