use azalea_core::{binary_search, Axis, AxisCycle};

use crate::{BitSetDiscreteVoxelShape, DiscreteVoxelShape, AABB, EPSILON};
use std::{cmp, ops::Add};

pub struct Shapes {}

pub fn block_shape() -> Box<dyn VoxelShape> {
    let mut shape = BitSetDiscreteVoxelShape::new(1, 1, 1);
    shape.fill(0, 0, 0);
    Box::new(CubeVoxelShape::new(Box::new(shape)))
}
pub fn empty_shape() -> Box<dyn VoxelShape> {
    Box::new(ArrayVoxelShape::new(
        Box::new(BitSetDiscreteVoxelShape::new(0, 0, 0)),
        vec![0.],
        vec![0.],
        vec![0.],
    ))
}

impl Shapes {
    pub fn collide(
        axis: &Axis,
        entity_box: &AABB,
        collision_boxes: &Vec<Box<dyn VoxelShape>>,
        mut movement: f64,
    ) -> f64 {
        for shape in collision_boxes {
            if movement.abs() < EPSILON {
                return 0.;
            }
            movement = shape.collide(axis, entity_box, movement);
        }
        movement
    }
}

pub trait VoxelShape {
    fn shape(&self) -> Box<dyn DiscreteVoxelShape>;

    fn get_coords(&self, axis: Axis) -> Vec<f64>;

    // TODO: optimization: should this be changed to return ArrayVoxelShape?
    // i might change the implementation of empty_shape in the future so not 100% sure
    fn move_relative(&self, x: f64, y: f64, z: f64) -> Box<dyn VoxelShape> {
        if self.shape().is_empty() {
            return empty_shape();
        }
        Box::new(ArrayVoxelShape::new(
            self.shape(),
            self.get_coords(Axis::X).iter().map(|c| c + x).collect(),
            self.get_coords(Axis::Y).iter().map(|c| c + y).collect(),
            self.get_coords(Axis::Z).iter().map(|c| c + z).collect(),
        ))
    }

    fn get(&self, axis: Axis, index: usize) -> f64 {
        self.get_coords(axis)[index]
    }

    fn find_index(&self, axis: Axis, coord: f64) -> u32 {
        binary_search(0, self.shape().size(axis) + 1, &|t| {
            coord < self.get(axis, t as usize)
        }) - 1
    }

    fn collide(&self, axis: &Axis, entity_box: &AABB, movement: f64) -> f64 {
        self.collide_x(AxisCycle::between(*axis, Axis::X), entity_box, movement)
    }
    fn collide_x(&self, axis_cycle: AxisCycle, entity_box: &AABB, mut movement: f64) -> f64 {
        if self.shape().is_empty() {
            return movement;
        }
        if movement.abs() < EPSILON {
            return 0.;
        }

        let inverse_axis_cycle = axis_cycle.inverse();

        // probably not good names but idk what this does
        let x_axis = inverse_axis_cycle.cycle(Axis::X);
        let y_axis = inverse_axis_cycle.cycle(Axis::Y);
        let z_axis = inverse_axis_cycle.cycle(Axis::Z);

        // i gave up on names at this point (these are the obfuscated names from fernflower)
        let var9 = entity_box.max(&x_axis);
        let var11 = entity_box.min(&x_axis);

        let var13 = self.find_index(x_axis, var11 + EPSILON);
        let var14 = self.find_index(x_axis, var9 - EPSILON);

        let var15 = cmp::max(
            0,
            self.find_index(y_axis, entity_box.min(&y_axis) + EPSILON),
        );
        let var16 = cmp::min(
            self.shape().size(y_axis),
            self.find_index(y_axis, entity_box.max(&y_axis) - EPSILON) + 1,
        );

        let var17 = cmp::max(
            0,
            self.find_index(z_axis, entity_box.min(&z_axis) + EPSILON),
        );
        let var18 = cmp::min(
            self.shape().size(z_axis),
            self.find_index(z_axis, entity_box.max(&z_axis) - EPSILON) + 1,
        );

        let var19 = self.shape().size(x_axis);

        if movement > 0. {
            for var20 in var14 + 1..var19 {
                for var21 in var15..var16 {
                    for var22 in var17..var18 {
                        if self.shape().is_full_wide_axis_cycle(
                            inverse_axis_cycle,
                            var20,
                            var21,
                            var22,
                        ) {
                            let var23 = self.get(x_axis, var20 as usize) - var9;
                            if var23 >= -EPSILON {
                                movement = f64::min(movement, var23);
                            }
                            return movement;
                        }
                    }
                }
            }
        } else if movement < 0. {
            for var20 in (var13 - 1)..=0 {
                for var21 in var15..var16 {
                    for var22 in var17..var18 {
                        if self.shape().is_full_wide_axis_cycle(
                            inverse_axis_cycle,
                            var20,
                            var21,
                            var22,
                        ) {
                            let var23 = self.get(x_axis, (var20 + 1) as usize) - var11;
                            if var23 <= EPSILON {
                                movement = f64::max(movement, var23);
                            }
                            return movement;
                        }
                    }
                }
            }
        }

        return movement;
    }
}

pub struct ArrayVoxelShape {
    shape: Box<dyn DiscreteVoxelShape>,
    faces: Option<Vec<Box<dyn VoxelShape>>>,

    pub xs: Vec<f64>,
    pub ys: Vec<f64>,
    pub zs: Vec<f64>,
}

pub struct CubeVoxelShape {
    shape: Box<dyn DiscreteVoxelShape>,
    faces: Option<Vec<Box<dyn VoxelShape>>>,
}

impl ArrayVoxelShape {
    pub fn new(
        shape: Box<dyn DiscreteVoxelShape>,
        xs: Vec<f64>,
        ys: Vec<f64>,
        zs: Vec<f64>,
    ) -> Self {
        let x_size = shape.size(Axis::X) + 1;
        let y_size = shape.size(Axis::Y) + 1;
        let z_size = shape.size(Axis::Z) + 1;

        // Lengths of point arrays must be consistent with the size of the VoxelShape.
        assert_eq!(x_size, xs.len() as u32);
        assert_eq!(y_size, ys.len() as u32);
        assert_eq!(z_size, zs.len() as u32);

        Self {
            faces: None,
            shape,
            xs,
            ys,
            zs,
        }
    }
}

impl CubeVoxelShape {
    pub fn new(shape: Box<dyn DiscreteVoxelShape>) -> Self {
        Self { shape, faces: None }
    }
}

impl VoxelShape for ArrayVoxelShape {
    fn shape(&self) -> Box<dyn DiscreteVoxelShape> {
        self.shape.clone()
    }

    fn get_coords(&self, axis: Axis) -> Vec<f64> {
        axis.choose(self.xs.clone(), self.ys.clone(), self.zs.clone())
    }
}

impl VoxelShape for CubeVoxelShape {
    fn shape(&self) -> Box<dyn DiscreteVoxelShape> {
        self.shape.clone()
    }

    fn get_coords(&self, axis: Axis) -> Vec<f64> {
        let size = self.shape.size(axis);
        let mut parts = Vec::with_capacity(size as usize);
        for i in 0..size {
            parts.push(i as f64 / size as f64);
        }
        parts
    }
}
