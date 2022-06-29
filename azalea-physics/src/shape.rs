use std::ops::{Add, Index};

use azalea_core::Direction;

use crate::{BitSetDiscreteVoxelShape, DiscreteVoxelShape};

pub struct Shapes {}

pub fn block_shape() -> Box<dyn VoxelShape> {
    let mut shape = BitSetDiscreteVoxelShape::new(1, 1, 1);
    shape.fill(0, 0, 0);
    VoxelShape::new(Box::new(shape))
}

pub trait VoxelShape {
    fn shape(&self) -> Box<dyn DiscreteVoxelShape>;

    fn get_x_coords(&self) -> Vec<f64>;
    fn get_y_coords(&self) -> Vec<f64>;
    fn get_z_coords(&self) -> Vec<f64>;

    fn move_relative(&self, x: f64, y: f64, z: f64) -> ArrayVoxelShape {
        if self.shape().is_empty() {
            return Shapes::empty();
        }
        // TODO: just offset the vecs now instead of using an OffsetVec
        ArrayVoxelShape::new(
            self.shape(),
            self.get_x_coords().iter().map(|c| c + x).collect(),
            self.get_y_coords().iter().map(|c| c + y).collect(),
            self.get_z_coords().iter().map(|c| c + z).collect(),
        )
    }
}

pub struct ArrayVoxelShape {
    shape: Box<dyn DiscreteVoxelShape>,
    faces: Option<Vec<Box<dyn VoxelShape>>>,

    pub xs: Vec<f64>,
    pub ys: Vec<f64>,
    pub zs: Vec<f64>,
}

impl ArrayVoxelShape {
    // ArrayVoxelShape(DiscreteVoxelShape var1, DoubleList var2, DoubleList var3, DoubleList var4) {
    //     super(var1);
    //     int var5 = var1.getXSize() + 1;
    //     int var6 = var1.getYSize() + 1;
    //     int var7 = var1.getZSize() + 1;
    //     if (var5 == var2.size() && var6 == var3.size() && var7 == var4.size()) {
    //        this.xs = var2;
    //        this.ys = var3;
    //        this.zs = var4;
    //     } else {
    //        throw (IllegalArgumentException)Util.pauseInIde(new IllegalArgumentException("Lengths of point arrays must be consistent with the size of the VoxelShape."));
    //     }
    // }
    pub fn new(
        shape: Box<dyn DiscreteVoxelShape>,
        xs: Vec<f64>,
        ys: Vec<f64>,
        zs: Vec<f64>,
    ) -> Self {
        let x_size = shape.x_size() + 1;
        let y_size = shape.y_size() + 1;
        let z_size = shape.z_size() + 1;

        // Lengths of point arrays must be consistent with the size of the VoxelShape.
        assert_eq!(x_size, xs.len());
        assert_eq!(y_size, ys.len());
        assert_eq!(z_size, zs.len());

        Self {
            faces: None,
            shape,
            xs,
            ys,
            zs,
        }
    }
}

// mojang moment
// this is probably for an optimization and could probably be optimized more
/// A Vec that adds the given offset when indexing into it.
pub struct OffsetVec<T> {
    delegate: Vec<T>,
    offset: T,
}

impl<T> OffsetVec<T>
where
    T: Add<Output = T>,
{
    pub fn new(delegate: Vec<T>, offset: T) -> Self {
        Self { delegate, offset }
    }
    pub fn index(&self, index: usize) -> T {
        self.delegate[index] + self.offset
    }
    pub fn len(&self) -> usize {
        self.delegate.len()
    }
}
