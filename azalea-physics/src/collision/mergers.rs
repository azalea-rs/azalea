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
        self.coords.clone()
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
