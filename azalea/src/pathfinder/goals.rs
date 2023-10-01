use azalea_core::position::{BlockPos, Vec3};

use super::Goal;

pub struct BlockPosGoal(pub BlockPos);
impl Goal for BlockPosGoal {
    fn heuristic(&self, n: BlockPos) -> f32 {
        let dx = (self.0.x - n.x) as f32;
        let dy = (self.0.y - n.y) as f32;
        let dz = (self.0.z - n.z) as f32;
        dx * dx + dy * dy + dz * dz
    }
    fn success(&self, n: BlockPos) -> bool {
        n == self.0
    }
}

pub struct XZGoal {
    pub x: i32,
    pub z: i32,
}
impl Goal for XZGoal {
    fn heuristic(&self, n: BlockPos) -> f32 {
        let dx = (self.x - n.x) as f32;
        let dz = (self.z - n.z) as f32;
        dx * dx + dz * dz
    }
    fn success(&self, n: BlockPos) -> bool {
        n.x == self.x && n.z == self.z
    }
}

pub struct YGoal {
    pub y: i32,
}
impl Goal for YGoal {
    fn heuristic(&self, n: BlockPos) -> f32 {
        let dy = (self.y - n.y) as f32;
        dy * dy
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
