use std::{convert::TryInto, ops::Index};

use super::CubePointRange;
use azalea_core::{gcd, lcm};

pub trait IndexMerger {
    // DoubleList getList();

    // boolean forMergedIndexes(IndexMerger.IndexConsumer var1);

    // int size();

    // public interface IndexConsumer {
    //    boolean merge(int var1, int var2, int var3);
    // }
    fn get_list(&self) -> Vec<f64>;
    fn for_merged_indexes(&self, consumer: impl IndexConsumer) -> bool
    where
        Self: Sized;
    fn size(&self) -> usize;
}

trait IndexConsumer = FnMut(i32, i32, i32) -> bool;

pub struct IdenticalMerger {
    pub coords: Vec<f64>,
}
impl IndexMerger for IdenticalMerger {
    fn get_list(&self) -> Vec<f64> {
        self.coords.clone()
    }
    fn for_merged_indexes(&self, consumer: impl IndexConsumer) -> bool {
        for var3 in 0..(self.coords.len() - 1) {
            if !consumer(var3 as i32, var3 as i32, var3 as i32) {
                return false;
            }
        }
        true
    }
    fn size(&self) -> usize {
        self.coords.len()
    }
}

// public final class DiscreteCubeMerger implements IndexMerger {
//    private final CubePointRange result;
//    private final int firstDiv;
//    private final int secondDiv;
//    DiscreteCubeMerger(int var1, int var2) {
//       super();
//       this.result = new CubePointRange((int)Shapes.lcm(var1, var2));
//       int var3 = IntMath.gcd(var1, var2);
//       this.firstDiv = var1 / var3;
//       this.secondDiv = var2 / var3;
//    }
//    public boolean forMergedIndexes(IndexMerger.IndexConsumer var1) {
//       int var2 = this.result.size() - 1;
//       for(int var3 = 0; var3 < var2; ++var3) {
//          if (!var1.merge(var3 / this.secondDiv, var3 / this.firstDiv, var3)) {
//             return false;
//          }
//       }
//       return true;
//    }
//    public int size() {
//       return this.result.size();
//    }
//    public DoubleList getList() {
//       return this.result;
//    }
// }
pub struct DiscreteCubeMerger {
    pub result: CubePointRange,
    pub first_div: u32,
    pub second_div: u32,
}
impl DiscreteCubeMerger {
    //    DiscreteCubeMerger(int var1, int var2) {
    //       super();
    //       this.result = new CubePointRange((int)Shapes.lcm(var1, var2));
    //       int var3 = IntMath.gcd(var1, var2);
    //       this.firstDiv = var1 / var3;
    //       this.secondDiv = var2 / var3;
    //    }
    pub fn new(a: u32, b: u32) -> DiscreteCubeMerger {
        let result = CubePointRange {
            parts: (u32::try_from(lcm(a, b)).expect("lcm should be able to fit in a u32"))
                .try_into()
                .expect("lcm should not be 0"),
        };
        let gcd = gcd(a, b);
        let first_div = a / gcd;
        let second_div = b / gcd;
        DiscreteCubeMerger {
            result,
            first_div,
            second_div,
        }
    }
}
impl IndexMerger for DiscreteCubeMerger {
    //    public boolean forMergedIndexes(IndexMerger.IndexConsumer var1) {
    //       int var2 = this.result.size() - 1;
    //       for(int var3 = 0; var3 < var2; ++var3) {
    //          if (!var1.merge(var3 / this.secondDiv, var3 / this.firstDiv, var3)) {
    //             return false;
    //          }
    //       }
    //       return true;
    //    }
    fn for_merged_indexes(&self, consumer: impl IndexConsumer) -> bool {
        let var2 = self.result.size() - 1;
        for var3 in 0..var2 {
            if !consumer(
                (var3 / self.second_div).try_into().unwrap(),
                (var3 / self.first_div).try_into().unwrap(),
                var3.try_into().unwrap(),
            ) {
                return false;
            }
        }
        true
    }
    //    public int size() {
    //       return this.result.size();
    //    }
    fn size(&self) -> usize {
        self.result.size().try_into().unwrap()
    }
    //    public DoubleList getList() {
    //       return this.result;
    //    }
    fn get_list(&self) -> Vec<f64> {
        self.result.iter()
    }
}

// public class NonOverlappingMerger extends AbstractDoubleList implements IndexMerger {
//     private final DoubleList lower;
//     private final DoubleList upper;
//     private final boolean swap;
pub struct NonOverlappingMerger {
    pub lower: Vec<f64>,
    pub upper: Vec<f64>,
    pub swap: bool,
}

//     protected NonOverlappingMerger(DoubleList var1, DoubleList var2, boolean var3) {
//        super();
//        this.lower = var1;
//        this.upper = var2;
//        this.swap = var3;
//     }
impl NonOverlappingMerger {
    pub fn new(lower: Vec<f64>, upper: Vec<f64>, swap: bool) -> NonOverlappingMerger {
        NonOverlappingMerger { lower, upper, swap }
    }

    //     private boolean forNonSwappedIndexes(IndexMerger.IndexConsumer var1) {
    //        int var2 = this.lower.size();

    //        int var3;
    //        for(var3 = 0; var3 < var2; ++var3) {
    //           if (!var1.merge(var3, -1, var3)) {
    //              return false;
    //           }
    //        }

    //        var3 = this.upper.size() - 1;

    //        for(int var4 = 0; var4 < var3; ++var4) {
    //           if (!var1.merge(var2 - 1, var4, var2 + var4)) {
    //              return false;
    //           }
    //        }

    //        return true;
    //     }
    fn for_non_swapped_indexes(&self, consumer: impl IndexConsumer) -> bool {
        let var2 = self.lower.len();
        for var3 in 0..var2 {
            if !consumer(var3.try_into().unwrap(), -1, var3.try_into().unwrap()) {
                return false;
            }
        }
        let var3 = self.upper.len() - 1;
        for var4 in 0..var3 {
            if !consumer(
                (var2 - 1).try_into().unwrap(),
                var4.try_into().unwrap(),
                (var2 + var4).try_into().unwrap(),
            ) {
                return false;
            }
        }
        true
    }
}

//     public int size() {
//        return this.lower.size() + this.upper.size();
//     }
impl IndexMerger for NonOverlappingMerger {
    fn size(&self) -> usize {
        self.lower.len() + self.upper.len()
    }

    //     public boolean forMergedIndexes(IndexMerger.IndexConsumer var1) {
    //        return this.swap ? this.forNonSwappedIndexes((var1x, var2, var3) -> {
    //           return var1.merge(var2, var1x, var3);
    //        }) : this.forNonSwappedIndexes(var1);
    //     }
    fn for_merged_indexes(&self, mut consumer: impl IndexConsumer) -> bool {
        if self.swap {
            self.for_non_swapped_indexes(move |var1x, var2, var3| consumer(var2, var1x, var3))
        } else {
            self.for_non_swapped_indexes(consumer)
        }
    }

    //     public DoubleList getList() {
    //        return this;
    //     }
    fn get_list(&self) -> Vec<f64> {
        (0..self.size()).map(|i| *self.index(i)).collect()
    }
}
//  }

// //     public double getDouble(int var1) {
// //        return var1 < this.lower.size() ? this.lower.getDouble(var1) : this.upper.getDouble(var1 - this.lower.size());
// //     }
// fn get_double(&self, var1: usize) -> f64 {
//     if var1 < self.lower.len() {
//         self.lower[var1]
//     } else {
//         self.upper[var1 - self.lower.len()]
//     }
// }
impl Index<usize> for NonOverlappingMerger {
    fn index(&self, var1: usize) -> &f64 {
        if var1 < self.lower.len() {
            &self.lower[var1]
        } else {
            &self.upper[var1 - self.lower.len()]
        }
    }

    type Output = f64;
}

// public class IndirectMerger implements IndexMerger {
//     private static final DoubleList EMPTY = DoubleLists.unmodifiable(DoubleArrayList.wrap(new double[]{0.0D}));
//     private final double[] result;
//     private final int[] firstIndices;
//     private final int[] secondIndices;
//     private final int resultLength;
pub struct IndirectMerger {
    pub result: Vec<f64>,
    pub first_indices: Vec<usize>,
    pub second_indices: Vec<usize>,
    pub result_length: usize,
}

//     public IndirectMerger(DoubleList var1, DoubleList var2, boolean var3, boolean var4) {
//        super();
//        double var5 = 0.0D / 0.0;
//        int var7 = var1.size();
//        int var8 = var2.size();
//        int var9 = var7 + var8;
//        this.result = new double[var9];
//        this.firstIndices = new int[var9];
//        this.secondIndices = new int[var9];
//        boolean var10 = !var3;
//        boolean var11 = !var4;
//        int var12 = 0;
//        int var13 = 0;
//        int var14 = 0;

//        while(true) {
//           boolean var17;
//           while(true) {
//              boolean var15 = var13 >= var7;
//              boolean var16 = var14 >= var8;
//              if (var15 && var16) {
//                 this.resultLength = Math.max(1, var12);
//                 return;
//              }

//              var17 = !var15 && (var16 || var1.getDouble(var13) < var2.getDouble(var14) + 1.0E-7D);
//              if (var17) {
//                 ++var13;
//                 if (!var10 || var14 != 0 && !var16) {
//                    break;
//                 }
//              } else {
//                 ++var14;
//                 if (!var11 || var13 != 0 && !var15) {
//                    break;
//                 }
//              }
//           }

//           int var18 = var13 - 1;
//           int var19 = var14 - 1;
//           double var20 = var17 ? var1.getDouble(var18) : var2.getDouble(var19);
//           if (!(var5 >= var20 - 1.0E-7D)) {
//              this.firstIndices[var12] = var18;
//              this.secondIndices[var12] = var19;
//              this.result[var12] = var20;
//              ++var12;
//              var5 = var20;
//           } else {
//              this.firstIndices[var12 - 1] = var18;
//              this.secondIndices[var12 - 1] = var19;
//           }
//        }
//     }
impl IndirectMerger {
    pub fn new(var1: &Vec<f64>, var2: &Vec<f64>, var3: bool, var4: bool) -> IndirectMerger {
        let mut var5 = f64::NAN;
        let var7 = var1.len();
        let var8 = var2.len();
        let var9 = var7 + var8;
        let mut result = vec![0.0; var9];
        let mut first_indices = vec![0; var9];
        let mut second_indices = vec![0; var9];
        let var10 = !var3;
        let var11 = !var4;
        let mut var12 = 0;
        let mut var13 = 0;
        let mut var14 = 0;

        loop {
            let mut var17: bool;
            loop {
                let var15 = var13 >= var7;
                let var16 = var14 >= var8;
                if var15 && var16 {
                    let result_length = std::cmp::max(1, var12);
                    return IndirectMerger {
                        result,
                        first_indices,
                        second_indices,
                        result_length,
                    };
                }

                var17 = !var15 && (var16 || var1[var13] < var2[var14] + 1.0e-7);
                if var17 {
                    var13 += 1;
                    if !var10 || var14 != 0 && !var16 {
                        break;
                    }
                } else {
                    var14 += 1;
                    if !var11 || var13 != 0 && !var15 {
                        break;
                    }
                }
            }

            let var18 = var13 - 1;
            let var19 = var14 - 1;
            let var20 = if var17 { var1[var18] } else { var2[var19] };
            if !(var5 >= var20 - 1.0e-7) {
                first_indices[var12] = var18;
                second_indices[var12] = var19;
                result[var12] = var20;
                var12 += 1;
                var5 = var20;
            } else {
                first_indices[var12 - 1] = var18;
                second_indices[var12 - 1] = var19;
            }
        }
    }

    //     public boolean forMergedIndexes(IndexMerger.IndexConsumer var1) {
    //        int var2 = this.resultLength - 1;

    //        for(int var3 = 0; var3 < var2; ++var3) {
    //           if (!var1.merge(this.firstIndices[var3], this.secondIndices[var3], var3)) {
    //              return false;
    //           }
    //        }

    //        return true;
    //     }
}

impl IndexMerger for IndirectMerger {
    fn for_merged_indexes(&self, consumer: impl IndexConsumer) -> bool {
        let var2 = self.result_length - 1;

        for var3 in 0..var2 {
            if !consumer(
                self.first_indices[var3].try_into().unwrap(),
                self.second_indices[var3].try_into().unwrap(),
                var3.try_into().unwrap(),
            ) {
                return false;
            }
        }

        true
    }

    //     public int size() {
    //        return this.resultLength;
    //     }
    fn size(&self) -> usize {
        self.result_length
    }

    //     public DoubleList getList() {
    //        return (DoubleList)(this.resultLength <= 1 ? EMPTY : DoubleArrayList.wrap(this.result, this.resultLength));
    //     }
    fn get_list(&self) -> Vec<f64> {
        if self.result_length <= 1 {
            vec![]
        } else {
            self.result[..self.result_length].to_vec()
        }
    }
    //  }
}
