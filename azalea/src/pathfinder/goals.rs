use azalea_core::BlockPos;

use super::{Goal, Node, VerticalVel};

pub struct BlockPosGoal {
    pub pos: BlockPos,
}
impl Goal for BlockPosGoal {
    fn heuristic(&self, n: &Node) -> f32 {
        let dx = (self.pos.x - n.pos.x) as f32;
        let dy = (self.pos.y - n.pos.y) as f32;
        let dz = (self.pos.z - n.pos.z) as f32;
        dx * dx + dy * dy + dz * dz
    }
    fn success(&self, n: &Node) -> bool {
        n.pos == self.pos
    }
    fn goal_node(&self) -> Node {
        Node {
            pos: self.pos,
            vertical_vel: VerticalVel::None,
        }
    }
}

impl From<BlockPos> for BlockPosGoal {
    fn from(pos: BlockPos) -> Self {
        Self { pos }
    }
}
