use std::convert::TryInto;

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
    fn for_merged_indexes(&self, consumer: &IndexConsumer) -> bool;
    fn size(&self) -> usize;
}

type IndexConsumer = dyn FnOnce(i32, i32, i32) -> bool;

pub struct IdenticalMerger {
    pub coords: Vec<f64>,
}
impl IndexMerger for IdenticalMerger {
    fn get_list(&self) -> Vec<f64> {
        self.coords
    }
    fn for_merged_indexes(&self, consumer: &IndexConsumer) -> bool {
        let mut var2 = self.coords.len() - 1;
        for var3 in 0..var2 {
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
        let mut result = CubePointRange {
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
    fn for_merged_indexes(&self, consumer: &IndexConsumer) -> bool {
        let mut var2 = self.result.size() - 1;
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
