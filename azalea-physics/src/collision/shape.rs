use super::mergers::IndexMerger;
use crate::collision::{BitSetDiscreteVoxelShape, DiscreteVoxelShape, AABB};
use azalea_core::{binary_search, lcm, Axis, AxisCycle, EPSILON};
use std::{any::Any, cmp, num::NonZeroU32};

pub struct Shapes {}

pub fn block_shape() -> VoxelShape {
    let mut shape = BitSetDiscreteVoxelShape::new(1, 1, 1);
    shape.fill(0, 0, 0);
    VoxelShape::Cube(CubeVoxelShape::new(DiscreteVoxelShape::BitSet(shape)))
}

pub fn box_shape(
    min_x: f64,
    min_y: f64,
    min_z: f64,
    max_x: f64,
    max_y: f64,
    max_z: f64,
) -> VoxelShape {
    assert!(min_x >= 0.);
    assert!(min_y >= 0.);
    assert!(min_z >= 0.);
    assert!(max_x >= 0.);
    assert!(max_y >= 0.);
    assert!(max_z >= 0.);

    if max_x - min_x < EPSILON && max_y - min_y < EPSILON && max_z - min_z < EPSILON {
        return empty_shape();
    }

    let x_bits = find_bits(min_x, max_x);
    let y_bits = find_bits(min_y, max_y);
    let z_bits = find_bits(min_z, max_z);

    if x_bits < 0 || y_bits < 0 || z_bits < 0 {
        return VoxelShape::Array(ArrayVoxelShape::new(
            block_shape().shape(),
            vec![min_x, max_x],
            vec![min_y, max_y],
            vec![min_z, max_z],
        ));
    }
    if x_bits == 0 && y_bits == 0 && z_bits == 0 {
        return block_shape();
    }

    let x_bits = 1 << x_bits;
    let y_bits = 1 << y_bits;
    let z_bits = 1 << z_bits;
    let shape = BitSetDiscreteVoxelShape::with_filled_bounds(
        x_bits,
        y_bits,
        z_bits,
        (min_x * x_bits as f64).round() as i32,
        (min_y * y_bits as f64).round() as i32,
        (min_z * z_bits as f64).round() as i32,
        (max_x * x_bits as f64).round() as i32,
        (max_y * y_bits as f64).round() as i32,
        (max_z * z_bits as f64).round() as i32,
    );
    VoxelShape::Cube(CubeVoxelShape::new(DiscreteVoxelShape::BitSet(shape)))
}
pub fn empty_shape() -> VoxelShape {
    VoxelShape::Array(ArrayVoxelShape::new(
        DiscreteVoxelShape::BitSet(BitSetDiscreteVoxelShape::new(0, 0, 0)),
        vec![0.],
        vec![0.],
        vec![0.],
    ))
}

fn find_bits(min: f64, max: f64) -> i32 {
    if min < -EPSILON || max > 1. + EPSILON {
        return -1;
    }
    for bits in 0..=3 {
        let bits = 1 << bits;
        let min = min * bits as f64;
        let max = max * bits as f64;
        let min_ok = (min - min.round()).abs() < EPSILON * bits as f64;
        let max_ok = (max - max.round()).abs() < EPSILON * bits as f64;
        if min_ok && max_ok {
            return bits;
        }
    }
    -1
}

impl Shapes {
    pub fn or(a: VoxelShape, b: VoxelShape) -> VoxelShape {
        Self::join(a, b, |a, b| a || b)
    }

    pub fn collide(
        axis: &Axis,
        entity_box: &AABB,
        collision_boxes: &Vec<VoxelShape>,
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

    pub fn join(a: VoxelShape, b: VoxelShape, op: fn(bool, bool) -> bool) -> VoxelShape {
        Self::join_unoptimized(a, b, op).optimize()
    }

    pub fn join_unoptimized(
        a: VoxelShape,
        b: VoxelShape,
        op: fn(bool, bool) -> bool,
    ) -> VoxelShape {
        if op(false, false) {
            panic!("Illegal operation");
        };
        // if (a == b) {
        //     return if op(true, true) { a } else { empty_shape() };
        // }
        let op_true_false = op(true, false);
        let op_false_true = op(false, true);
        if a.is_empty() {
            return if op_false_true { b } else { empty_shape() };
        }
        if b.is_empty() {
            return if op_true_false { a } else { empty_shape() };
        }
        // IndexMerger var5 = createIndexMerger(1, a.getCoords(Direction.Axis.X), b.getCoords(Direction.Axis.X), var3, var4);
        // IndexMerger var6 = createIndexMerger(var5.size() - 1, a.getCoords(Direction.Axis.Y), b.getCoords(Direction.Axis.Y), var3, var4);
        // IndexMerger var7 = createIndexMerger((var5.size() - 1) * (var6.size() - 1), a.getCoords(Direction.Axis.Z), b.getCoords(Direction.Axis.Z), var3, var4);
        // BitSetDiscreteVoxelShape var8 = BitSetDiscreteVoxelShape.join(a.shape, b.shape, var5, var6, var7, op);
        // return (VoxelShape)(var5 instanceof DiscreteCubeMerger && var6 instanceof DiscreteCubeMerger && var7 instanceof DiscreteCubeMerger ? new CubeVoxelShape(var8) : new ArrayVoxelShape(var8, var5.getList(), var6.getList(), var7.getList()));
        let var5 = Self::create_index_merger(
            1,
            a.get_coords(Axis::X),
            b.get_coords(Axis::X),
            op_true_false,
            op_false_true,
        );
        let var6 = Self::create_index_merger(
            (var5.size() - 1).try_into().unwrap(),
            a.get_coords(Axis::Y),
            b.get_coords(Axis::Y),
            op_true_false,
            op_false_true,
        );
        let var7 = Self::create_index_merger(
            ((var5.size() - 1) * (var6.size() - 1)).try_into().unwrap(),
            a.get_coords(Axis::Z),
            b.get_coords(Axis::Z),
            op_true_false,
            op_false_true,
        );
        let var8 = BitSetDiscreteVoxelShape::join(&a.shape(), &b.shape(), &var5, &var6, &var7, &op);
        // if var5.is_discrete_cube_merger()
        if let IndexMerger::DiscreteCube { .. } = var5
            && let IndexMerger::DiscreteCube { .. } = var6
            && let IndexMerger::DiscreteCube { .. } = var7
        {
            VoxelShape::Cube(CubeVoxelShape::new(DiscreteVoxelShape::BitSet(var8)))
        } else {
            VoxelShape::Array(ArrayVoxelShape::new(
                DiscreteVoxelShape::BitSet(var8),
                var5.get_list(),
                var6.get_list(),
                var7.get_list(),
            ))
        }
    }

    pub fn create_index_merger(
        var0: i32,
        var1: Vec<f64>,
        var2: Vec<f64>,
        var3: bool,
        var4: bool,
    ) -> IndexMerger {
        // int var5 = var1.size() - 1;
        let var5 = var1.len() - 1;
        // int var6 = var2.size() - 1
        let var6 = var2.len() - 1;
        // if (var1 instanceof CubePointRange && var2 instanceof CubePointRange) {
        // downcast
        if (&var1 as &dyn Any).is::<CubePointRange>() && (&var2 as &dyn Any).is::<CubePointRange>()
        {
            // return new DiscreteCubeMerger(var0, var5, var6, var3, var4);
            let var7: i64 = lcm(var5 as u32, var6 as u32).try_into().unwrap();
            //    if ((long)var0 * var7 <= 256L) {
            if var0 as i64 * var7 <= 256 {
                return IndexMerger::new_discrete_cube(var5 as u32, var6 as u32);
            }
        }

        // if (var1.getDouble(var5) < var2.getDouble(0) - 1.0E-7D) {
        //    return new NonOverlappingMerger(var1, var2, false);
        // } else if (var2.getDouble(var6) < var1.getDouble(0) - 1.0E-7D) {
        //    return new NonOverlappingMerger(var2, var1, true);
        // } else {
        //    return (IndexMerger)(var5 == var6 && Objects.equals(var1, var2) ? new IdenticalMerger(var1) : new IndirectMerger(var1, var2, var3, var4));
        // }
        if var1[var5] < var2[0] - 1.0E-7 {
            IndexMerger::NonOverlapping {
                lower: var1,
                upper: var2,
                swap: false,
            }
        } else if var2[var6] < var1[0] - 1.0E-7 {
            IndexMerger::NonOverlapping {
                lower: var2,
                upper: var1,
                swap: true,
            }
        } else if var5 == var6 && var1 == var2 {
            IndexMerger::Identical { coords: var1 }
        } else {
            IndexMerger::new_indirect(&var1, &var2, var3, var4)
        }
    }
}

#[derive(Clone)]
pub enum VoxelShape {
    Array(ArrayVoxelShape),
    Cube(CubeVoxelShape),
}

impl VoxelShape {
    pub fn shape(&self) -> DiscreteVoxelShape {
        match self {
            VoxelShape::Array(s) => s.shape(),
            VoxelShape::Cube(s) => s.shape(),
        }
    }

    pub fn get_coords(&self, axis: Axis) -> Vec<f64> {
        match self {
            VoxelShape::Array(s) => s.get_coords(axis),
            VoxelShape::Cube(s) => s.get_coords(axis),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.shape().is_empty()
    }

    // TODO: optimization: should this be changed to return ArrayVoxelShape?
    // i might change the implementation of empty_shape in the future so not 100% sure
    pub fn move_relative(&self, x: f64, y: f64, z: f64) -> VoxelShape {
        // if self.shape().is_empty() {
        //     return empty_shape();
        // }

        // Box::new(ArrayVoxelShape::new(
        //     self.shape(),
        //     self.get_coords(Axis::X).iter().map(|c| c + x).collect(),
        //     self.get_coords(Axis::Y).iter().map(|c| c + y).collect(),
        //     self.get_coords(Axis::Z).iter().map(|c| c + z).collect(),
        // ))
        match self {
            // VoxelShape::Array(s) => s.move_relative(x, y, z),
            // VoxelShape::Cube(s) => s.move_relative(x, y, z),
            _ => {
                if self.shape().is_empty() {
                    return empty_shape();
                }

                VoxelShape::Array(ArrayVoxelShape::new(
                    self.shape(),
                    self.get_coords(Axis::X).iter().map(|c| c + x).collect(),
                    self.get_coords(Axis::Y).iter().map(|c| c + y).collect(),
                    self.get_coords(Axis::Z).iter().map(|c| c + z).collect(),
                ))
            }
        }
    }

    pub fn get(&self, axis: Axis, index: usize) -> f64 {
        // self.get_coords(axis)[index]
        match self {
            VoxelShape::Array(s) => s.get_coords(axis)[index],
            VoxelShape::Cube(s) => s.get_coords(axis)[index],
            // _ => self.get_coords(axis)[index],
        }
    }

    pub fn find_index(&self, axis: Axis, coord: f64) -> i32 {
        // let r = binary_search(0, (self.shape().size(axis) + 1) as i32, &|t| {
        //     coord < self.get(axis, t as usize)
        // }) - 1;
        // r
        match self {
            VoxelShape::Cube(s) => s.find_index(axis, coord),
            _ => {
                binary_search(0, (self.shape().size(axis) + 1) as i32, &|t| {
                    coord < self.get(axis, t as usize)
                }) - 1
            }
        }
    }

    pub fn collide(&self, axis: &Axis, entity_box: &AABB, movement: f64) -> f64 {
        // self.collide_x(AxisCycle::between(*axis, Axis::X), entity_box, movement)
        match self {
            _ => self.collide_x(AxisCycle::between(*axis, Axis::X), entity_box, movement),
        }
    }
    pub fn collide_x(&self, axis_cycle: AxisCycle, entity_box: &AABB, mut movement: f64) -> f64 {
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

        let max_x = entity_box.max(&x_axis);
        let min_x = entity_box.min(&x_axis);

        // i gave up on names at this point (these are the obfuscated names from fernflower)
        let var13 = self.find_index(x_axis, min_x + EPSILON);
        let var14 = self.find_index(x_axis, max_x - EPSILON);

        let var15 = cmp::max(
            0,
            self.find_index(y_axis, entity_box.min(&y_axis) + EPSILON),
        );
        let var16 = cmp::min(
            self.shape().size(y_axis) as i32,
            self.find_index(y_axis, entity_box.max(&y_axis) - EPSILON) + 1,
        );

        let var17 = cmp::max(
            0,
            self.find_index(z_axis, entity_box.min(&z_axis) + EPSILON),
        );
        let var18 = cmp::min(
            self.shape().size(z_axis) as i32,
            self.find_index(z_axis, entity_box.max(&z_axis) - EPSILON) + 1,
        );

        let var19 = self.shape().size(x_axis);
        if movement > 0. {
            for var20 in var14 + 1..(var19 as i32) {
                for var21 in var15..var16 {
                    for var22 in var17..var18 {
                        if self.shape().is_full_wide_axis_cycle(
                            inverse_axis_cycle,
                            var20.try_into().unwrap(),
                            var21.try_into().unwrap(),
                            var22.try_into().unwrap(),
                        ) {
                            let var23 = self.get(x_axis, var20 as usize) - max_x;
                            if var23 >= -EPSILON {
                                movement = f64::min(movement, var23);
                            }
                            return movement;
                        }
                    }
                }
            }
        } else if movement < 0. && var13 > 0 {
            for var20 in (var13 - 1)..=0 {
                for var21 in var15..var16 {
                    for var22 in var17..var18 {
                        if self.shape().is_full_wide_axis_cycle(
                            inverse_axis_cycle,
                            var20.try_into().unwrap(),
                            var21.try_into().unwrap(),
                            var22.try_into().unwrap(),
                        ) {
                            let var23 = self.get(x_axis, (var20 + 1) as usize) - min_x;
                            if var23 <= EPSILON {
                                movement = f64::max(movement, var23);
                            }
                            return movement;
                        }
                    }
                }
            }
        }

        movement
    }

    // public VoxelShape optimize() {
    //     VoxelShape[] var1 = new VoxelShape[]{Shapes.empty()};
    //     this.forAllBoxes((var1x, var3, var5, var7, var9, var11) -> {
    //         var1[0] = Shapes.joinUnoptimized(var1[0], Shapes.box(var1x, var3, var5, var7, var9, var11), BooleanOp.OR);
    //     });
    //     return var1[0];
    // }
    fn optimize(&self) -> VoxelShape {
        // let mut var1 = empty_shape();
        // self.for_all_boxes(|var1x, var3, var5, var7, var9, var11| {
        //     var1 = Shapes::join_unoptimized(
        //         var1,
        //         box_shape(var1x, var3, var5, var7, var9, var11),
        //         |a, b| a || b,
        //     );
        // });
        // var1
        let mut var1 = empty_shape();
        self.for_all_boxes(|var1x, var3, var5, var7, var9, var11| {
            var1 = Shapes::join_unoptimized(
                var1.clone(),
                box_shape(var1x, var3, var5, var7, var9, var11),
                |a, b| a || b,
            );
        });
        var1
    }

    // public void forAllBoxes(Shapes.DoubleLineConsumer var1) {
    //     DoubleList var2 = this.getCoords(Direction.Axis.X);
    //     DoubleList var3 = this.getCoords(Direction.Axis.Y);
    //     DoubleList var4 = this.getCoords(Direction.Axis.Z);
    //     this.shape.forAllBoxes((var4x, var5, var6, var7, var8, var9) -> {
    //     var1.consume(var2.getDouble(var4x), var3.getDouble(var5), var4.getDouble(var6), var2.getDouble(var7), var3.getDouble(var8), var4.getDouble(var9));
    //     }, true);
    // }
    pub fn for_all_boxes(&self, mut consumer: impl FnMut(f64, f64, f64, f64, f64, f64))
    where
        Self: Sized,
    {
        // let x_coords = self.get_coords(Axis::X);
        // let y_coords = self.get_coords(Axis::Y);
        // let z_coords = self.get_coords(Axis::Z);
        // self.shape().for_all_boxes(
        //     |var4x, var5, var6, var7, var8, var9| {
        //         consumer(
        //             x_coords[var4x as usize],
        //             y_coords[var5 as usize],
        //             z_coords[var6 as usize],
        //             x_coords[var7 as usize],
        //             y_coords[var8 as usize],
        //             z_coords[var9 as usize],
        //         )
        //     },
        //     true,
        // );
        let x_coords = self.get_coords(Axis::X);
        let y_coords = self.get_coords(Axis::Y);
        let z_coords = self.get_coords(Axis::Z);
        self.shape().for_all_boxes(
            |var4x, var5, var6, var7, var8, var9| {
                consumer(
                    x_coords[var4x as usize],
                    y_coords[var5 as usize],
                    z_coords[var6 as usize],
                    x_coords[var7 as usize],
                    y_coords[var8 as usize],
                    z_coords[var9 as usize],
                )
            },
            true,
        );
    }
}

#[derive(Clone)]
pub struct ArrayVoxelShape {
    shape: DiscreteVoxelShape,
    // TODO: check where faces is used in minecraft
    #[allow(dead_code)]
    faces: Option<Vec<VoxelShape>>,

    pub xs: Vec<f64>,
    pub ys: Vec<f64>,
    pub zs: Vec<f64>,
}

#[derive(Clone)]
pub struct CubeVoxelShape {
    shape: DiscreteVoxelShape,
    // TODO: check where faces is used in minecraft
    #[allow(dead_code)]
    faces: Option<Vec<VoxelShape>>,
}

impl ArrayVoxelShape {
    pub fn new(shape: DiscreteVoxelShape, xs: Vec<f64>, ys: Vec<f64>, zs: Vec<f64>) -> Self {
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
    pub fn new(shape: DiscreteVoxelShape) -> Self {
        Self { shape, faces: None }
    }
}

impl ArrayVoxelShape {
    fn shape(&self) -> DiscreteVoxelShape {
        self.shape.clone()
    }

    fn get_coords(&self, axis: Axis) -> Vec<f64> {
        axis.choose(self.xs.clone(), self.ys.clone(), self.zs.clone())
    }
}

impl CubeVoxelShape {
    fn shape(&self) -> DiscreteVoxelShape {
        self.shape.clone()
    }

    fn get_coords(&self, axis: Axis) -> Vec<f64> {
        let size = self.shape.size(axis);
        let mut parts = Vec::with_capacity(size as usize);
        for i in 0..=size {
            parts.push(i as f64 / size as f64);
        }
        parts
    }

    fn find_index(&self, axis: Axis, coord: f64) -> i32 {
        let n = self.shape().size(axis);
        (f64::clamp(coord * (n as f64), -1f64, n as f64)) as i32
    }
}

// public class CubePointRange extends AbstractDoubleList {
//     private final int parts;

//     CubePointRange(int var1) {
//        super();
//        if (var1 <= 0) {
//           throw new IllegalArgumentException("Need at least 1 part");
//        } else {
//           this.parts = var1;
//        }
//     }

//     public double getDouble(int var1) {
//        return (double)var1 / (double)this.parts;
//     }

//     public int size() {
//        return this.parts + 1;
//     }
// }
pub struct CubePointRange {
    /// Needs at least 1 part
    pub parts: NonZeroU32,
}
impl CubePointRange {
    pub fn get_double(&self, index: u32) -> f64 {
        index as f64 / self.parts.get() as f64
    }

    pub fn size(&self) -> u32 {
        self.parts.get() + 1
    }

    pub fn iter(&self) -> Vec<f64> {
        (0..=self.parts.get()).map(|i| self.get_double(i)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_shape() {
        let shape = block_shape();
        assert_eq!(shape.shape().size(Axis::X), 1);
        assert_eq!(shape.shape().size(Axis::Y), 1);
        assert_eq!(shape.shape().size(Axis::Z), 1);

        assert_eq!(shape.get_coords(Axis::X).len(), 2);
        assert_eq!(shape.get_coords(Axis::Y).len(), 2);
        assert_eq!(shape.get_coords(Axis::Z).len(), 2);
    }
}
