use azalea_core::{Axis, AxisCycle, BitSet};

use super::mergers::IndexMerger;

// TODO: every impl of DiscreteVoxelShape could be turned into a single enum as an optimization

pub trait DiscreteVoxelShape: Send + Sync {
    fn size(&self, axis: Axis) -> u32;

    fn first_full_x(&self) -> u32;
    fn first_full_y(&self) -> u32;
    fn first_full_z(&self) -> u32;

    fn last_full_x(&self) -> u32;
    fn last_full_y(&self) -> u32;
    fn last_full_z(&self) -> u32;

    fn is_empty(&self) -> bool {
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

    fn is_full_wide(&self, x: u32, y: u32, z: u32) -> bool {
        (x < self.size(Axis::X) && y < self.size(Axis::Y) && z < self.size(Axis::Z))
            && (self.is_full(x, y, z))
    }
    fn is_full_wide_axis_cycle(&self, axis_cycle: AxisCycle, x: u32, y: u32, z: u32) -> bool {
        self.is_full_wide(
            axis_cycle.cycle_xyz(x, y, z, Axis::X),
            axis_cycle.cycle_xyz(x, y, z, Axis::Y),
            axis_cycle.cycle_xyz(x, y, z, Axis::Z),
        )
    }

    fn is_full(&self, x: u32, y: u32, z: u32) -> bool;

    // i don't know how to do this properly
    fn clone(&self) -> Box<dyn DiscreteVoxelShape>;
}

#[derive(Default, Clone, Eq, PartialEq)]
pub struct BitSetDiscreteVoxelShape {
    x_size: u32,
    y_size: u32,
    z_size: u32,

    storage: BitSet,
    x_min: u32,
    y_min: u32,
    z_min: u32,
    x_max: u32,
    y_max: u32,
    z_max: u32,
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
            x_min,
            y_min,
            z_min,
            x_max: 0,
            y_max: 0,
            z_max: 0,
        }
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
            self.x_min = std::cmp::min(self.x_min, x);
            self.y_min = std::cmp::min(self.y_min, y);
            self.z_min = std::cmp::min(self.z_min, z);
            self.x_max = std::cmp::max(self.x_max, x + 1);
            self.y_max = std::cmp::max(self.y_max, y + 1);
            self.z_max = std::cmp::max(self.z_max, z + 1);
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
    fn get_index(&self, x: u32, y: u32, z: u32) -> usize {
        ((x * self.y_size + y) * self.z_size + z) as usize
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
        var0: &dyn DiscreteVoxelShape,
        var1: &dyn DiscreteVoxelShape,
        var2: &dyn IndexMerger,
        var3: &dyn IndexMerger,
        var4: &dyn IndexMerger,
        var5: impl FnOnce(bool, bool) -> bool,
    ) -> Self {
        let mut var6 = BitSetDiscreteVoxelShape::new(var2.size() - 1, var3.size() - 1, var4.size() - 1);
        let mut var7 = [2147483647, 2147483647, 2147483647, -2147483648, -2147483648, -2147483648];
        var2.for_merged_indexes(|var7x, var8, var9| {
            let mut var10 = [false];
            var3.for_merged_indexes(|var10x, var11, var12| {
                let mut var13 = [false];
                var4.for_merged_indexes(|var12x, var13x, var14| {
                    if var5.apply(var0.is_full_wide(var7x, var10x, var12x), var1.is_full_wide(var8, var11, var13x)) {
                        var6.storage.set(var6.get_index(var9, var12, var14));
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
        var6.x_min = var7[0];
        var6.y_min = var7[1];
        var6.z_min = var7[2];
        var6.x_max = var7[3] +
}

impl DiscreteVoxelShape for BitSetDiscreteVoxelShape {
    fn size(&self, axis: Axis) -> u32 {
        axis.choose(self.x_size, self.y_size, self.z_size)
    }

    fn first_full_x(&self) -> u32 {
        self.x_min
    }
    fn first_full_y(&self) -> u32 {
        self.y_min
    }
    fn first_full_z(&self) -> u32 {
        self.z_min
    }

    fn last_full_x(&self) -> u32 {
        self.x_max
    }
    fn last_full_y(&self) -> u32 {
        self.y_max
    }
    fn last_full_z(&self) -> u32 {
        self.z_max
    }

    fn clone(&self) -> Box<dyn DiscreteVoxelShape> {
        Box::new(Clone::clone(self))
    }

    fn is_full(&self, x: u32, y: u32, z: u32) -> bool {
        self.storage.index(self.get_index(x, y, z))
    }
}
