use azalea_core::BlockPos;
use azalea_world::Dimension;

trait Move {
    fn can_execute(&self, dim: &Dimension, pos: &BlockPos) -> bool;
    /// Returns by how much the entity's position should be changed when this move is executed.
    fn offset(&self) -> BlockPos;
}

pub struct NorthMove {}
impl Move for NorthMove {
    fn can_execute(&self, dim: &Dimension, pos: &BlockPos) -> bool {
        dim.get_block_state(&(pos + &self.offset())).is_some()
    }
    fn offset(&self) -> BlockPos {
        BlockPos::new(0, 0, -1)
    }
}
