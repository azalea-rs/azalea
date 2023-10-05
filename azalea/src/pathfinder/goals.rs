use std::f32::consts::SQRT_2;

use azalea_core::position::{BlockPos, Vec3};

use super::{
    costs::{FALL_ONE_BLOCK_COST, JUMP_ONE_BLOCK_COST},
    Goal,
};

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

    diagonal * SQRT_2 + straight
}

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
    if dy < 0.0 {
        FALL_ONE_BLOCK_COST * -dy
    } else {
        *JUMP_ONE_BLOCK_COST * dy
    }
}

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

pub struct InverseGoal<T: Goal>(pub T);
impl<T: Goal> Goal for InverseGoal<T> {
    fn heuristic(&self, n: BlockPos) -> f32 {
        -self.0.heuristic(n)
    }
    fn success(&self, n: BlockPos) -> bool {
        !self.0.success(n)
    }
}

pub struct OrGoal<T: Goal, U: Goal>(pub T, pub U);
impl<T: Goal, U: Goal> Goal for OrGoal<T, U> {
    fn heuristic(&self, n: BlockPos) -> f32 {
        self.0.heuristic(n).min(self.1.heuristic(n))
    }
    fn success(&self, n: BlockPos) -> bool {
        self.0.success(n) || self.1.success(n)
    }
}

pub struct AndGoal<T: Goal, U: Goal>(pub T, pub U);
impl<T: Goal, U: Goal> Goal for AndGoal<T, U> {
    fn heuristic(&self, n: BlockPos) -> f32 {
        self.0.heuristic(n).max(self.1.heuristic(n))
    }
    fn success(&self, n: BlockPos) -> bool {
        self.0.success(n) && self.1.success(n)
    }
}
