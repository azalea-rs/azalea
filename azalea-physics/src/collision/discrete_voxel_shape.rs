use std::cmp;

use azalea_core::{
    bitset::BitSet,
    direction::{Axis, AxisCycle},
    position::Vec3i,
};

use super::mergers::IndexMerger;

pub trait IntLineConsumer = FnMut(u32, u32, u32, u32, u32, u32);

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum DiscreteVoxelShape {
    BitSet(BitSetDiscreteVoxelShape),
}

impl DiscreteVoxelShape {
    #[inline]
    pub fn size(&self, axis: Axis) -> u32 {
        match self {
            DiscreteVoxelShape::BitSet(shape) => shape.size(axis),
        }
    }

    pub fn first_full(&self, axis: Axis) -> i32 {
        match self {
            DiscreteVoxelShape::BitSet(shape) => shape.first_full(axis),
        }
    }

    pub fn last_full(&self, axis: Axis) -> i32 {
        match self {
            DiscreteVoxelShape::BitSet(shape) => shape.last_full(axis),
        }
    }

    pub fn is_empty(&self) -> bool {
        if self.first_full(Axis::X) >= self.last_full(Axis::X) {
            return true;
        }
        if self.first_full(Axis::Y) >= self.last_full(Axis::Y) {
            return true;
        }
        if self.first_full(Axis::Z) >= self.last_full(Axis::Z) {
            return true;
        }
        false
    }

    pub fn is_full_wide(&self, pos: Vec3i) -> bool {
        if pos.x < 0 || pos.y < 0 || pos.z < 0 {
            return false;
        }
        let (x, y, z) = (pos.x as u32, pos.y as u32, pos.z as u32);

        (x < self.size(Axis::X) && y < self.size(Axis::Y) && z < self.size(Axis::Z))
            && (self.is_full(x, y, z))
    }

    pub fn is_full_wide_axis_cycle(&self, axis_cycle: AxisCycle, pos: Vec3i) -> bool {
        self.is_full_wide(Vec3i {
            x: axis_cycle.cycle_xyz(pos, Axis::X),
            y: axis_cycle.cycle_xyz(pos, Axis::Y),
            z: axis_cycle.cycle_xyz(pos, Axis::Z),
        })
    }

    pub fn is_full(&self, x: u32, y: u32, z: u32) -> bool {
        match self {
            DiscreteVoxelShape::BitSet(shape) => shape.is_full(x, y, z),
        }
    }

    pub fn for_all_boxes(&self, consumer: impl IntLineConsumer, swap: bool) {
        BitSetDiscreteVoxelShape::for_all_boxes(self, consumer, swap);
    }
}

#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct BitSetDiscreteVoxelShape {
    x_size: u32,
    y_size: u32,
    z_size: u32,

    storage: BitSet,
    min: Vec3i,
    max: Vec3i,
}

impl BitSetDiscreteVoxelShape {
    pub fn new(x_min: u32, y_min: u32, z_min: u32) -> Self {
        BitSetDiscreteVoxelShape {
            x_size: x_min,
            y_size: y_min,
            z_size: z_min,

            storage: BitSet::new((x_min * y_min * z_min).try_into().unwrap()),
            min: Vec3i {
                x: z_min.try_into().unwrap(),
                y: z_min.try_into().unwrap(),
                z: z_min.try_into().unwrap(),
            },
            max: Vec3i::ZERO,
        }
    }

    // yeah don't really feel like fixing this one
    #[allow(clippy::too_many_arguments)]
    pub fn with_filled_bounds(
        x_size: u32,
        y_size: u32,
        z_size: u32,
        min: Vec3i,
        max: Vec3i,
    ) -> Self {
        let mut shape = BitSetDiscreteVoxelShape::new(x_size, y_size, z_size);
        shape.min = min;
        shape.max = max;

        for x in min.x..max.x {
            for y in min.y..max.y {
                for z in min.z..max.z {
                    shape.fill_update_bounds(
                        x.try_into().unwrap(),
                        y.try_into().unwrap(),
                        z.try_into().unwrap(),
                        false,
                    );
                }
            }
        }

        shape
    }

    fn fill_update_bounds(&mut self, x: u32, y: u32, z: u32, update: bool) {
        self.storage.set(self.get_index(x, y, z));
        if update {
            let candidate_min = Vec3i::new(x as i32, y as i32, z as i32);
            self.min = self.min.min(candidate_min);
            let candidate_max = Vec3i::new((x + 1) as i32, (y + 1) as i32, (z + 1) as i32);
            self.max = self.max.max(candidate_max);
        }
    }

    pub fn fill(&mut self, x: u32, y: u32, z: u32) {
        self.fill_update_bounds(x, y, z, true);
    }

    fn get_index_from_size(x: u32, y: u32, z: u32, y_size: u32, z_size: u32) -> usize {
        ((x * y_size + y) * z_size + z) as usize
    }
    fn get_index(&self, x: u32, y: u32, z: u32) -> usize {
        Self::get_index_from_size(x, y, z, self.y_size, self.z_size)
    }

    pub fn join(
        var0: &DiscreteVoxelShape,
        var1: &DiscreteVoxelShape,
        var2: &IndexMerger,
        var3: &IndexMerger,
        var4: &IndexMerger,
        op: impl Fn(bool, bool) -> bool,
    ) -> Self {
        let mut var6 = BitSetDiscreteVoxelShape::new(
            (var2.size() - 1) as u32,
            (var3.size() - 1) as u32,
            (var4.size() - 1) as u32,
        );
        let mut var7: [i32; 6] = [
            2147483647,
            2147483647,
            2147483647,
            -2147483648,
            -2147483648,
            -2147483648,
        ];
        var2.for_merged_indexes(|x1: i32, x2: i32, x3: i32| {
            let mut var10 = [false];
            var3.for_merged_indexes(|y1: i32, y2: i32, y3: i32| {
                let mut var13 = [false];
                var4.for_merged_indexes(|z1: i32, z2: i32, z3: i32| {
                    if op(
                        var0.is_full_wide(Vec3i::new(x1, y1, z1)),
                        var1.is_full_wide(Vec3i::new(x2, y2, z2)),
                    ) {
                        var6.storage.set(var6.get_index(
                            x3.try_into().unwrap(),
                            y3.try_into().unwrap(),
                            z3.try_into().unwrap(),
                        ));
                        var7[2] = cmp::min(var7[2], z3);
                        var7[5] = cmp::max(var7[5], z3);
                        var13[0] = true;
                    }

                    true
                });
                if var13[0] {
                    var7[1] = cmp::min(var7[1], y3);
                    var7[4] = cmp::max(var7[4], y3);
                    var10[0] = true;
                }

                true
            });
            if var10[0] {
                var7[0] = cmp::min(var7[0], x3);
                var7[3] = cmp::max(var7[3], x3);
            }

            true
        });
        var6.min.x = var7[0];
        var6.min.y = var7[1];
        var6.min.z = var7[2];
        var6.max.x = var7[3] + 1;
        var6.max.y = var7[4] + 1;
        var6.max.z = var7[5] + 1;
        var6
    }

    pub fn for_all_boxes(
        var0: &DiscreteVoxelShape,
        mut consumer: impl IntLineConsumer,
        var2: bool,
    ) {
        let mut var3 = BitSetDiscreteVoxelShape::from(var0);
        for y in 0..var3.y_size {
            for x in 0..var3.x_size {
                let mut var6 = None;
                for z in 0..=var3.z_size {
                    if var3.is_full_wide(x, y, z) {
                        if var2 {
                            if var6.is_none() {
                                var6 = Some(z);
                            }
                        } else {
                            consumer(x, y, z, x + 1, y + 1, z + 1);
                        }
                    } else if var6.is_some() {
                        let mut var8 = x;
                        let mut var9 = y;
                        var3.clear_z_strip(var6.unwrap(), z, x, y);
                        while var3.is_z_strip_full(var6.unwrap(), z, var8 + 1, y) {
                            var3.clear_z_strip(var6.unwrap(), z, var8 + 1, y);
                            var8 += 1;
                        }
                        while var3.is_xz_rectangle_full(x, var8 + 1, var6.unwrap(), z, var9 + 1) {
                            for var10 in x..=var8 {
                                var3.clear_z_strip(var6.unwrap(), z, var10, var9 + 1);
                            }
                            var9 += 1;
                        }
                        consumer(x, y, var6.unwrap(), var8 + 1, var9 + 1, z);
                        var6 = None;
                    }
                }
            }
        }
    }

    fn is_z_strip_full(&self, var1: u32, var2: u32, var3: u32, var4: u32) -> bool {
        if var3 < self.x_size && var4 < self.y_size {
            self.storage
                .next_clear_bit(self.get_index(var3, var4, var1))
                >= self.get_index(var3, var4, var2)
        } else {
            false
        }
    }

    fn is_xz_rectangle_full(&self, var1: u32, var2: u32, var3: u32, var4: u32, var5: u32) -> bool {
        for var6 in var1..var2 {
            if !self.is_z_strip_full(var3, var4, var6, var5) {
                return false;
            }
        }
        true
    }

    fn clear_z_strip(&mut self, var1: u32, var2: u32, var3: u32, var4: u32) {
        self.storage
            .clear(self.get_index(var3, var4, var1)..self.get_index(var3, var4, var2));
    }
}

impl BitSetDiscreteVoxelShape {
    #[inline]
    fn size(&self, axis: Axis) -> u32 {
        axis.choose(self.x_size, self.y_size, self.z_size)
    }

    fn first_full(&self, axis: Axis) -> i32 {
        axis.choose(self.min.x, self.min.y, self.min.z)
    }

    fn last_full(&self, axis: Axis) -> i32 {
        axis.choose(self.max.x, self.max.y, self.max.z)
    }

    fn is_full(&self, x: u32, y: u32, z: u32) -> bool {
        self.storage.get(self.get_index(x, y, z)).unwrap_or(false)
    }

    fn is_full_wide(&self, x: u32, y: u32, z: u32) -> bool {
        (x < self.size(Axis::X) && y < self.size(Axis::Y) && z < self.size(Axis::Z))
            && (self.is_full(x, y, z))
    }
}

impl From<&DiscreteVoxelShape> for BitSetDiscreteVoxelShape {
    fn from(shape: &DiscreteVoxelShape) -> Self {
        let x_size = shape.size(Axis::X);
        let y_size = shape.size(Axis::Y);
        let z_size = shape.size(Axis::Z);
        let mut storage;
        // more things could be added to DiscreteVoxelShape in the future
        #[allow(irrefutable_let_patterns)]
        if let DiscreteVoxelShape::BitSet(shape) = shape {
            storage = shape.storage.clone();
        } else {
            storage = BitSet::new((x_size * y_size * z_size) as usize);
            for x in 0..x_size {
                for y in 0..y_size {
                    for z in 0..z_size {
                        if shape.is_full(x, y, z) {
                            storage.set(Self::get_index_from_size(x, y, z, y_size, z_size));
                        }
                    }
                }
            }
        }

        Self {
            x_size,
            y_size,
            z_size,
            storage,
            min: Vec3i {
                x: shape.first_full(Axis::X),
                y: shape.first_full(Axis::Y),
                z: shape.first_full(Axis::Z),
            },
            max: Vec3i {
                x: shape.last_full(Axis::X),
                y: shape.last_full(Axis::Y),
                z: shape.last_full(Axis::Z),
            },
        }
    }
}
