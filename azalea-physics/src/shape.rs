use azalea_core::Direction;

use crate::{BitSetDiscreteVoxelShape, DiscreteVoxelShape};

pub struct VoxelShape {
    shape: BitSetDiscreteVoxelShape,
    faces: Vec<VoxelShape>,
}

pub struct Shapes {}

pub fn block_shape() -> VoxelShape {
    let mut shape = BitSetDiscreteVoxelShape::new(1, 1, 1);
    shape.fill(0, 0, 0);
    VoxelShape::new(shape)
}

impl VoxelShape {
    pub fn new(shape: BitSetDiscreteVoxelShape) -> Self {
        VoxelShape {
            shape,
            faces: Vec::new(),
        }
    }

    // public VoxelShape move(double var1, double var3, double var5) {
    //     return (VoxelShape)(this.isEmpty() ? Shapes.empty() : new ArrayVoxelShape(this.shape, new OffsetDoubleList(this.getCoords(Direction.Axis.X), var1), new OffsetDoubleList(this.getCoords(Direction.Axis.Y), var3), new OffsetDoubleList(this.getCoords(Direction.Axis.Z), var5)));
    // }
    pub fn move_relative(&self, x: i32, y: i32, z: i32) -> Self {
        if self.is_empty() {
            return Shapes::empty();
        }
        ArrayVoxelShape::new(
            self.shape,
            OffsetDoubleList::new(self.get_coords(Direction::Axis::X), x),
            OffsetDoubleList::new(self.get_coords(Direction::Axis::Y), y),
            OffsetDoubleList::new(self.get_coords(Direction::Axis::Z), z),
        )
    }
}
