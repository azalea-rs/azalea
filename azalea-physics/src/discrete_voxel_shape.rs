use azalea_core::BitSet;

// TODO: every impl of DiscreteVoxelShape could be turned into a single enum as an optimization

pub trait DiscreteVoxelShape {
    // public boolean isFullWide(int var1, int var2, int var3) {
    // 	if (var1 >= 0 && var2 >= 0 && var3 >= 0) {
    // 	   return var1 < this.xSize && var2 < this.ySize && var3 < this.zSize ? this.isFull(var1, var2, var3) : false;
    // 	} else {
    // 	   return false;
    // 	}
    //  }
    // pub fn is_full_wide(&self, x: i32, y: i32, z: i32) -> bool {
    //     // TODO: can this be u32 instead of i32?
    //     if x >= 0 && y >= 0 && z >= 0 {
    //         if x < self.x_size as i32 && y < self.y_size as i32 && z < self.z_size as i32 {
    //             return self.is_full(x, y, z);
    //         }
    //     }
    //     return false;
    // }
    fn x_size(&self) -> u32;
    fn y_size(&self) -> u32;
    fn z_size(&self) -> u32;

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

    // i don't know how to do this properly
    fn clone(&self) -> Box<dyn DiscreteVoxelShape>;
}

#[derive(Default, Clone)]
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
}

impl DiscreteVoxelShape for BitSetDiscreteVoxelShape {
    fn x_size(&self) -> u32 {
        self.x_size
    }
    fn y_size(&self) -> u32 {
        self.y_size
    }
    fn z_size(&self) -> u32 {
        self.z_size
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
}
