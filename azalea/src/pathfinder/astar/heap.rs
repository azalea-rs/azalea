use std::{
    cmp::{self, Reverse},
    collections::BinaryHeap,
};

use radix_heap::RadixHeapMap;

#[derive(Default)]
pub struct PathfinderHeap {
    /// Key is f_score.to_bits(), value is (g_score, index)
    ///
    /// As long as the f_score is positive, comparing it as bits is fine. Also,
    /// it has to be `Reverse`d to make it a min-heap.
    radix_heap: RadixHeapMap<Reverse<u32>, (f32, u32)>,
    // fallback
    binary_heap: BinaryHeap<WeightedNode>,
}
impl PathfinderHeap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, item: WeightedNode) {
        if let Some(top) = self.radix_heap.top() {
            // this can happen when the heuristic wasn't an underestimate, so just fall back
            // to a binary heap in those cases
            if item.f_score < f32::from_bits(top.0) {
                self.binary_heap.push(item);
                return;
            }
        }
        self.radix_heap
            .push(Reverse(item.f_score.to_bits()), (item.g_score, item.index))
    }
    pub fn pop(&mut self) -> Option<WeightedNode> {
        self.binary_heap.pop().or_else(|| {
            self.radix_heap
                .pop()
                .map(|(f_score, (g_score, index))| WeightedNode {
                    f_score: f32::from_bits(f_score.0),
                    g_score,
                    index,
                })
        })
    }
}

#[derive(PartialEq, Debug)]
#[repr(C)]
pub struct WeightedNode {
    /// Sum of the g_score and heuristic
    pub f_score: f32,
    /// The actual cost to get to this node
    pub g_score: f32,
    pub index: u32,
}

impl Ord for WeightedNode {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        // intentionally inverted to make the BinaryHeap a min-heap
        match other.f_score.total_cmp(&self.f_score) {
            cmp::Ordering::Equal => self.g_score.total_cmp(&other.g_score),
            s => s,
        }
    }
}
impl Eq for WeightedNode {}
impl PartialOrd for WeightedNode {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn weighted_node(f: f32, g: f32) -> WeightedNode {
        WeightedNode {
            f_score: f,
            g_score: g,
            index: 0,
        }
    }

    #[test]
    fn test_weighted_node_eq() {
        let a = weighted_node(0., 0.);
        let b = weighted_node(0., 0.);
        assert!(a == b);
    }
    #[test]
    fn test_weighted_node_le() {
        let a = weighted_node(1., 0.);
        let b = weighted_node(0., 0.);
        assert_eq!(a.cmp(&b), cmp::Ordering::Less);
        assert!(a.le(&b));
    }
    #[test]
    fn test_weighted_node_le_g() {
        let a = weighted_node(0., 1.);
        let b = weighted_node(0., 0.);
        assert_eq!(a.cmp(&b), cmp::Ordering::Greater);
        assert!(!a.le(&b));
    }
}
