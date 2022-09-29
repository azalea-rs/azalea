use super::mergers::IndexMerger;
use azalea_core::{Axis, AxisCycle, BitSet};

// TODO: every impl of DiscreteVoxelShape could be turned into a single enum as an optimization

pub trait IntLineConsumer = FnMut(u32, u32, u32, u32, u32, u32);

#[derive(Clone)]
pub enum DiscreteVoxelShape {
    BitSet(BitSetDiscreteVoxelShape),
}

impl DiscreteVoxelShape {
    // pub trait DiscreteVoxelShape: Send + Sync {
    //     fn size(&self, axis: Axis) -> u32;
    pub fn size(&self, axis: Axis) -> u32 {
        match self {
            DiscreteVoxelShape::BitSet(shape) => shape.size(axis),
        }
    }

    //     fn first_full_x(&self) -> u32;
    //     fn first_full_y(&self) -> u32;
    //     fn first_full_z(&self) -> u32;
    pub fn first_full_x(&self) -> u32 {
        match self {
            DiscreteVoxelShape::BitSet(shape) => shape.first_full_x(),
        }
    }
    pub fn first_full_y(&self) -> u32 {
        match self {
            DiscreteVoxelShape::BitSet(shape) => shape.first_full_y(),
        }
    }
    pub fn first_full_z(&self) -> u32 {
        match self {
            DiscreteVoxelShape::BitSet(shape) => shape.first_full_z(),
        }
    }

    //     fn last_full_x(&self) -> u32;
    //     fn last_full_y(&self) -> u32;
    //     fn last_full_z(&self) -> u32;
    pub fn last_full_x(&self) -> u32 {
        match self {
            DiscreteVoxelShape::BitSet(shape) => shape.last_full_x(),
        }
    }
    pub fn last_full_y(&self) -> u32 {
        match self {
            DiscreteVoxelShape::BitSet(shape) => shape.last_full_y(),
        }
    }
    pub fn last_full_z(&self) -> u32 {
        match self {
            DiscreteVoxelShape::BitSet(shape) => shape.last_full_z(),
        }
    }

    //     fn is_empty(&self) -> bool {
    //         if self.first_full_x() >= self.last_full_x() {
    //             return true;
    //         }
    //         if self.first_full_y() >= self.last_full_y() {
    //             return true;
    //         }
    //         if self.first_full_x() >= self.last_full_x() {
    //             return true;
    //         }
    //         false
    //     }
    pub fn is_empty(&self) -> bool {
        match self {
            _ => {
                if self.first_full_x() >= self.last_full_x() {
                    return true;
                }
                if self.first_full_y() >= self.last_full_y() {
                    return true;
                }
                if self.first_full_x() >= self.last_full_x() {
                    return true;
                }
                false
            }
        }
    }

    //     fn is_full_wide(&self, x: u32, y: u32, z: u32) -> bool {
    //         (x < self.size(Axis::X) && y < self.size(Axis::Y) && z < self.size(Axis::Z))
    //             && (self.is_full(x, y, z))
    //     }
    pub fn is_full_wide(&self, x: u32, y: u32, z: u32) -> bool {
        (x < self.size(Axis::X) && y < self.size(Axis::Y) && z < self.size(Axis::Z))
            && (self.is_full(x, y, z))
    }
    //     fn is_full_wide_axis_cycle(&self, axis_cycle: AxisCycle, x: u32, y: u32, z: u32) -> bool {
    //         self.is_full_wide(
    //             axis_cycle.cycle_xyz(x, y, z, Axis::X),
    //             axis_cycle.cycle_xyz(x, y, z, Axis::Y),
    //             axis_cycle.cycle_xyz(x, y, z, Axis::Z),
    //         )
    //     }
    pub fn is_full_wide_axis_cycle(&self, axis_cycle: AxisCycle, x: u32, y: u32, z: u32) -> bool {
        self.is_full_wide(
            axis_cycle.cycle_xyz(x, y, z, Axis::X),
            axis_cycle.cycle_xyz(x, y, z, Axis::Y),
            axis_cycle.cycle_xyz(x, y, z, Axis::Z),
        )
    }

    //     fn is_full(&self, x: u32, y: u32, z: u32) -> bool;
    pub fn is_full(&self, x: u32, y: u32, z: u32) -> bool {
        match self {
            DiscreteVoxelShape::BitSet(shape) => shape.is_full(x, y, z),
        }
    }

    //     // public void forAllBoxes(DiscreteVoxelShape.IntLineConsumer var1, boolean var2) {
    //     //     BitSetDiscreteVoxelShape.forAllBoxes(this, var1, var2);
    //     // }
    //     fn for_all_boxes(&self, consumer: impl IntLineConsumer, swap: bool)
    //     where
    //         Self: Sized,
    //     {
    //         BitSetDiscreteVoxelShape::for_all_boxes(self, consumer, swap);
    //     }
    // }
    pub fn for_all_boxes(&self, consumer: impl IntLineConsumer, swap: bool) {
        BitSetDiscreteVoxelShape::for_all_boxes(self, consumer, swap)
    }
}

#[derive(Default, Clone, Eq, PartialEq)]
pub struct BitSetDiscreteVoxelShape {
    x_size: u32,
    y_size: u32,
    z_size: u32,

    storage: BitSet,
    x_min: i32,
    y_min: i32,
    z_min: i32,
    x_max: i32,
    y_max: i32,
    z_max: i32,
}

impl BitSetDiscreteVoxelShape {
    // public BitSetDiscreteVoxelShape(int var1, int var2, int var3) {
    // 	super(var1, var2, var3);
    // 	this.storage = new BitSet(var1 * var2 * var3);
    // 	this.xMin = var1;
    // 	this.yMin = var2;
    // 	this.zMin = var3;
    // }
    pub fn new(x_min: u32, y_min: u32, z_min: u32) -> Self {
        BitSetDiscreteVoxelShape {
            x_size: x_min,
            y_size: y_min,
            z_size: z_min,

            storage: BitSet::new((x_min * y_min * z_min).try_into().unwrap()),
            x_min: z_min.try_into().unwrap(),
            y_min: z_min.try_into().unwrap(),
            z_min: z_min.try_into().unwrap(),
            x_max: 0,
            y_max: 0,
            z_max: 0,
        }
    }

    //    public static BitSetDiscreteVoxelShape withFilledBounds(int var0, int var1, int var2, int var3, int var4, int var5, int var6, int var7, int var8) {
    //     BitSetDiscreteVoxelShape var9 = new BitSetDiscreteVoxelShape(var0, var1, var2);
    //     var9.xMin = var3;
    //     var9.yMin = var4;
    //     var9.zMin = var5;
    //     var9.xMax = var6;
    //     var9.yMax = var7;
    //     var9.zMax = var8;

    //     for(int var10 = var3; var10 < var6; ++var10) {
    //        for(int var11 = var4; var11 < var7; ++var11) {
    //           for(int var12 = var5; var12 < var8; ++var12) {
    //              var9.fillUpdateBounds(var10, var11, var12, false);
    //           }
    //        }
    //     }

    //     return var9;
    //  }
    pub fn with_filled_bounds(
        x_size: u32,
        y_size: u32,
        z_size: u32,
        x_min: i32,
        y_min: i32,
        z_min: i32,
        x_max: i32,
        y_max: i32,
        z_max: i32,
    ) -> Self {
        let mut shape = BitSetDiscreteVoxelShape::new(x_size, y_size, z_size);
        shape.x_min = x_min;
        shape.y_min = y_min;
        shape.z_min = z_min;
        shape.x_max = x_max;
        shape.y_max = y_max;
        shape.z_max = z_max;

        for x in x_min..x_max {
            for y in y_min..y_max {
                for z in z_min..z_max {
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

    // private void fillUpdateBounds(int var1, int var2, int var3, boolean var4) {
    // 	this.storage.set(this.getIndex(var1, var2, var3));
    // 	if (var4) {
    // 	   this.xMin = Math.min(this.xMin, var1);
    // 	   this.yMin = Math.min(this.yMin, var2);
    // 	   this.zMin = Math.min(this.zMin, var3);
    // 	   this.xMax = Math.max(this.xMax, var1 + 1);
    // 	   this.yMax = Math.max(this.yMax, var2 + 1);
    // 	   this.zMax = Math.max(this.zMax, var3 + 1);
    // 	}
    // }
    fn fill_update_bounds(&mut self, x: u32, y: u32, z: u32, update: bool) {
        self.storage.set(self.get_index(x, y, z));
        if update {
            self.x_min = std::cmp::min(self.x_min, x as i32);
            self.y_min = std::cmp::min(self.y_min, y as i32);
            self.z_min = std::cmp::min(self.z_min, z as i32);
            self.x_max = std::cmp::max(self.x_max, (x + 1) as i32);
            self.y_max = std::cmp::max(self.y_max, (y + 1) as i32);
            self.z_max = std::cmp::max(self.z_max, (z + 1) as i32);
        }
    }

    // public void fill(int var1, int var2, int var3) {
    // 	this.fillUpdateBounds(var1, var2, var3, true);
    // }
    pub fn fill(&mut self, x: u32, y: u32, z: u32) {
        self.fill_update_bounds(x, y, z, true);
    }

    // protected int getIndex(int var1, int var2, int var3) {
    //     return (var1 * this.ySize + var2) * this.zSize + var3;
    // }
    fn get_index_from_size(x: u32, y: u32, z: u32, y_size: u32, z_size: u32) -> usize {
        ((x * y_size + y) * z_size + z) as usize
    }
    fn get_index(&self, x: u32, y: u32, z: u32) -> usize {
        Self::get_index_from_size(x, y, z, self.y_size, self.z_size)
    }

    // static BitSetDiscreteVoxelShape join(DiscreteVoxelShape var0, DiscreteVoxelShape var1, IndexMerger var2, IndexMerger var3, IndexMerger var4, BooleanOp var5) {
    //     BitSetDiscreteVoxelShape var6 = new BitSetDiscreteVoxelShape(var2.size() - 1, var3.size() - 1, var4.size() - 1);
    //     int[] var7 = new int[]{2147483647, 2147483647, 2147483647, -2147483648, -2147483648, -2147483648};
    //     var2.forMergedIndexes((var7x, var8, var9) -> {
    //        boolean[] var10 = new boolean[]{false};
    //        var3.forMergedIndexes((var10x, var11, var12) -> {
    //           boolean[] var13 = new boolean[]{false};
    //           var4.forMergedIndexes((var12x, var13x, var14) -> {
    //              if (var5.apply(var0.isFullWide(var7x, var10x, var12x), var1.isFullWide(var8, var11, var13x))) {
    //                 var6.storage.set(var6.getIndex(var9, var12, var14));
    //                 var7[2] = Math.min(var7[2], var14);
    //                 var7[5] = Math.max(var7[5], var14);
    //                 var13[0] = true;
    //              }

    //              return true;
    //           });
    //           if (var13[0]) {
    //              var7[1] = Math.min(var7[1], var12);
    //              var7[4] = Math.max(var7[4], var12);
    //              var10[0] = true;
    //           }

    //           return true;
    //        });
    //        if (var10[0]) {
    //           var7[0] = Math.min(var7[0], var9);
    //           var7[3] = Math.max(var7[3], var9);
    //        }

    //        return true;
    //     });
    //     var6.xMin = var7[0];
    //     var6.yMin = var7[1];
    //     var6.zMin = var7[2];
    //     var6.xMax = var7[3] + 1;
    //     var6.yMax = var7[4] + 1;
    //     var6.zMax = var7[5] + 1;
    //     return var6;
    //  }
    pub fn join(
        var0: &DiscreteVoxelShape,
        var1: &DiscreteVoxelShape,
        var2: &IndexMerger,
        var3: &IndexMerger,
        var4: &IndexMerger,
        var5: impl Fn(bool, bool) -> bool,
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
        var2.for_merged_indexes(|var7x: i32, var8: i32, var9: i32| {
            let mut var10 = [false];
            var3.for_merged_indexes(|var10x: i32, var11: i32, var12: i32| {
                let mut var13 = [false];
                var4.for_merged_indexes(|var12x: i32, var13x: i32, var14: i32| {
                    if var5(
                        var0.is_full_wide(
                            var7x.try_into().unwrap(),
                            var10x.try_into().unwrap(),
                            var12x.try_into().unwrap(),
                        ),
                        var1.is_full_wide(
                            var8.try_into().unwrap(),
                            var11.try_into().unwrap(),
                            var13x.try_into().unwrap(),
                        ),
                    ) {
                        var6.storage.set(var6.get_index(
                            var9.try_into().unwrap(),
                            var12.try_into().unwrap(),
                            var14.try_into().unwrap(),
                        ));
                        var7[2] = std::cmp::min(var7[2], var14);
                        var7[5] = std::cmp::max(var7[5], var14);
                        var13[0] = true;
                    }

                    true
                });
                if var13[0] {
                    var7[1] = std::cmp::min(var7[1], var12);
                    var7[4] = std::cmp::max(var7[4], var12);
                    var10[0] = true;
                }

                true
            });
            if var10[0] {
                var7[0] = std::cmp::min(var7[0], var9);
                var7[3] = std::cmp::max(var7[3], var9);
            }

            true
        });
        //     var6.xMin = var7[0];
        //     var6.yMin = var7[1];
        //     var6.zMin = var7[2];
        //     var6.xMax = var7[3] + 1;
        //     var6.yMax = var7[4] + 1;
        //     var6.zMax = var7[5] + 1;
        //     return var6;
        var6.x_min = var7[0];
        var6.y_min = var7[1];
        var6.z_min = var7[2];
        var6.x_max = var7[3] + 1;
        var6.y_max = var7[4] + 1;
        var6.z_max = var7[5] + 1;
        var6
    }

    // protected static void forAllBoxes(DiscreteVoxelShape var0, DiscreteVoxelShape.IntLineConsumer var1, boolean var2) {
    //     BitSetDiscreteVoxelShape var3 = new BitSetDiscreteVoxelShape(var0);

    //     for(int var4 = 0; var4 < var3.ySize; ++var4) {
    //         for(int var5 = 0; var5 < var3.xSize; ++var5) {
    //             int var6 = -1;

    //             for(int var7 = 0; var7 <= var3.zSize; ++var7) {
    //                 if (var3.isFullWide(var5, var4, var7)) {
    //                     if (var2) {
    //                     if (var6 == -1) {
    //                         var6 = var7;
    //                     }
    //                     } else {
    //                     var1.consume(var5, var4, var7, var5 + 1, var4 + 1, var7 + 1);
    //                     }
    //                 } else if (var6 != -1) {
    //                     int var8 = var5;
    //                     int var9 = var4;
    //                     var3.clearZStrip(var6, var7, var5, var4);

    //                     while(var3.isZStripFull(var6, var7, var8 + 1, var4)) {
    //                     var3.clearZStrip(var6, var7, var8 + 1, var4);
    //                     ++var8;
    //                     }

    //                     while(var3.isXZRectangleFull(var5, var8 + 1, var6, var7, var9 + 1)) {
    //                     for(int var10 = var5; var10 <= var8; ++var10) {
    //                         var3.clearZStrip(var6, var7, var10, var9 + 1);
    //                     }

    //                     ++var9;
    //                     }

    //                     var1.consume(var5, var4, var6, var8 + 1, var9 + 1, var7);
    //                     var6 = -1;
    //                 }
    //             }
    //         }
    //     }
    // }
    pub fn for_all_boxes(
        var0: &DiscreteVoxelShape,
        mut consumer: impl IntLineConsumer,
        var2: bool,
    ) {
        let mut var3 = BitSetDiscreteVoxelShape::from(var0);
        for var4 in 0..var3.y_size {
            for var5 in 0..var3.x_size {
                let mut var6 = None;
                for var7 in 0..=var3.z_size {
                    if var3.is_full_wide(var5, var4, var7) {
                        if var2 {
                            if var6.is_none() {
                                var6 = Some(var7);
                            }
                        } else {
                            consumer(var5, var4, var7, var5 + 1, var4 + 1, var7 + 1);
                        }
                    } else if var6.is_some() {
                        let mut var8 = var5;
                        let mut var9 = var4;
                        var3.clear_z_strip(var6.unwrap(), var7, var5, var4);
                        while var3.is_z_strip_full(var6.unwrap(), var7, var8 + 1, var4) {
                            var3.clear_z_strip(var6.unwrap(), var7, var8 + 1, var4);
                            var8 += 1;
                        }
                        while var3.is_xz_rectangle_full(
                            var5,
                            var8 + 1,
                            var6.unwrap(),
                            var7,
                            var9 + 1,
                        ) {
                            for var10 in var5..=var8 {
                                var3.clear_z_strip(var6.unwrap(), var7, var10, var9 + 1);
                            }
                            var9 += 1;
                        }
                        consumer(var5, var4, var6.unwrap(), var8 + 1, var9 + 1, var7);
                        var6 = None;
                    }
                }
            }
        }
    }

    // private boolean isZStripFull(int var1, int var2, int var3, int var4) {
    //     if (var3 < this.xSize && var4 < this.ySize) {
    //        return this.storage.nextClearBit(this.getIndex(var3, var4, var1)) >= this.getIndex(var3, var4, var2);
    //     } else {
    //        return false;
    //     }
    //  }
    fn is_z_strip_full(&self, var1: u32, var2: u32, var3: u32, var4: u32) -> bool {
        if var3 < self.x_size && var4 < self.y_size {
            self.storage
                .next_clear_bit(self.get_index(var3, var4, var1))
                >= self.get_index(var3, var4, var2)
        } else {
            false
        }
    }

    // private boolean isXZRectangleFull(int var1, int var2, int var3, int var4, int var5) {
    //     for(int var6 = var1; var6 < var2; ++var6) {
    //        if (!this.isZStripFull(var3, var4, var6, var5)) {
    //           return false;
    //        }
    //     }

    //     return true;
    //  }
    fn is_xz_rectangle_full(&self, var1: u32, var2: u32, var3: u32, var4: u32, var5: u32) -> bool {
        for var6 in var1..var2 {
            if !self.is_z_strip_full(var3, var4, var6, var5) {
                return false;
            }
        }
        true
    }

    // private void clearZStrip(int var1, int var2, int var3, int var4) {
    //     this.storage.clear(this.getIndex(var3, var4, var1), this.getIndex(var3, var4, var2));
    // }
    fn clear_z_strip(&mut self, var1: u32, var2: u32, var3: u32, var4: u32) {
        self.storage.clear(
            self.get_index(var3, var4, var1),
            self.get_index(var3, var4, var2),
        );
    }
}

impl BitSetDiscreteVoxelShape {
    fn size(&self, axis: Axis) -> u32 {
        axis.choose(self.x_size, self.y_size, self.z_size)
    }

    fn first_full_x(&self) -> u32 {
        self.x_min.try_into().unwrap()
    }
    fn first_full_y(&self) -> u32 {
        self.y_min.try_into().unwrap()
    }
    fn first_full_z(&self) -> u32 {
        self.z_min.try_into().unwrap()
    }

    fn last_full_x(&self) -> u32 {
        self.x_max.try_into().unwrap()
    }
    fn last_full_y(&self) -> u32 {
        self.y_max.try_into().unwrap()
    }
    fn last_full_z(&self) -> u32 {
        self.z_max.try_into().unwrap()
    }

    fn is_full(&self, x: u32, y: u32, z: u32) -> bool {
        self.storage.index(self.get_index(x, y, z))
    }

    fn is_full_wide(&self, x: u32, y: u32, z: u32) -> bool {
        (x < self.size(Axis::X) && y < self.size(Axis::Y) && z < self.size(Axis::Z))
            && (self.is_full(x, y, z))
    }
}

// public BitSetDiscreteVoxelShape(DiscreteVoxelShape var1) {
//     super(var1.xSize, var1.ySize, var1.zSize);
//     if (var1 instanceof BitSetDiscreteVoxelShape) {
//        this.storage = (BitSet)((BitSetDiscreteVoxelShape)var1).storage.clone();
//     } else {
//        this.storage = new BitSet(this.xSize * this.ySize * this.zSize);

//        for(int var2 = 0; var2 < this.xSize; ++var2) {
//           for(int var3 = 0; var3 < this.ySize; ++var3) {
//              for(int var4 = 0; var4 < this.zSize; ++var4) {
//                 if (var1.isFull(var2, var3, var4)) {
//                    this.storage.set(this.getIndex(var2, var3, var4));
//                 }
//              }
//           }
//        }
//     }

//     this.xMin = var1.firstFull(Direction.Axis.X);
//     this.yMin = var1.firstFull(Direction.Axis.Y);
//     this.zMin = var1.firstFull(Direction.Axis.Z);
//     this.xMax = var1.lastFull(Direction.Axis.X);
//     this.yMax = var1.lastFull(Direction.Axis.Y);
//     this.zMax = var1.lastFull(Direction.Axis.Z);
// }

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
                            // ((x * self.y_size + y) * self.z_size + z) as usize
                            storage
                                .set(Self::get_index_from_size(x, y, z, y_size, z_size) as usize);
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
            x_min: shape.first_full_x().try_into().unwrap(),
            y_min: shape.first_full_y().try_into().unwrap(),
            z_min: shape.first_full_z().try_into().unwrap(),
            x_max: shape.last_full_x().try_into().unwrap(),
            y_max: shape.last_full_y().try_into().unwrap(),
            z_max: shape.last_full_z().try_into().unwrap(),
        }
    }
}
