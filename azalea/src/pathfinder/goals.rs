//! The goals that a pathfinder can try to reach.

use std::{
    f32::consts::SQRT_2,
    fmt::{self, Debug},
};

use azalea_core::position::{BlockPos, Vec3};
use azalea_world::ChunkStorage;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::costs::{COST_HEURISTIC, FALL_N_BLOCKS_COST, JUMP_ONE_BLOCK_COST};

pub trait Goal: Debug + Send + Sync {
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
        // the second half of this condition is intended to fix issues when pathing to
        // non-full blocks
        n == self.0 || n.down(1) == self.0
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
impl RadiusGoal {
    pub fn new(pos: Vec3, radius: f32) -> Self {
        Self { pos, radius }
    }
}
impl Goal for RadiusGoal {
    fn heuristic(&self, n: BlockPos) -> f32 {
        let n = n.center();
        let dx = (self.pos.x - n.x) as f32;
        let dy = (self.pos.y - n.y) as f32;
        let dz = (self.pos.z - n.z) as f32;
        dx.powi(2) + dy.powi(2) + dz.powi(2)
    }
    fn success(&self, n: BlockPos) -> bool {
        let n = n.center();
        let dx = (self.pos.x - n.x) as f32;
        let dy = (self.pos.y - n.y) as f32;
        let dz = (self.pos.z - n.z) as f32;
        dx.powi(2) + dy.powi(2) + dz.powi(2) <= self.radius.powi(2)
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
#[derive(Clone)]
pub struct ReachBlockPosGoal {
    pub pos: BlockPos,
    pub distance: f64,
    pub chunk_storage: ChunkStorage,

    max_check_distance: i32,
}
impl ReachBlockPosGoal {
    pub fn new(pos: BlockPos, chunk_storage: ChunkStorage) -> Self {
        Self::new_with_distance(pos, 4.5, chunk_storage)
    }

    pub fn new_with_distance(pos: BlockPos, distance: f64, chunk_storage: ChunkStorage) -> Self {
        Self {
            pos,
            distance,
            chunk_storage,
            max_check_distance: (distance + 2.).ceil() as i32,
        }
    }
}
impl Goal for ReachBlockPosGoal {
    fn heuristic(&self, n: BlockPos) -> f32 {
        BlockPosGoal(self.pos).heuristic(n)
    }
    fn success(&self, n: BlockPos) -> bool {
        let head = n.up(1);
        // the player's head is in the block or adjacent to it, so assume that it's
        // always reachable (we do this to account for mining)
        if head == self.pos
            || head.north(1) == self.pos
            || head.south(1) == self.pos
            || head.east(1) == self.pos
            || head.west(1) == self.pos
            || head.up(1) == self.pos
            || head.down(1) == self.pos
        {
            return true;
        }

        // only do the expensive check if we're close enough
        let distance_squared = self.pos.distance_squared_to(n);
        if distance_squared > self.max_check_distance.pow(2) {
            return false;
        }

        let eye_position = n.center_bottom().up(1.62);
        let look_direction = crate::bot::direction_looking_at(eye_position, self.pos.center());
        let block_hit_result = azalea_client::interact::pick::pick_block(
            look_direction,
            eye_position,
            &self.chunk_storage,
            self.distance,
        );

        block_hit_result.block_pos == self.pos
    }
}
impl Debug for ReachBlockPosGoal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ReachBlockPosGoal")
            .field("pos", &self.pos)
            .field("distance", &self.distance)
            .field("max_check_distance", &self.max_check_distance)
            .finish()
    }
}
