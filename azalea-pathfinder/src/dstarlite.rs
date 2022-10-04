//! An implementation of D* Lite (optimized version) from
//! http://idm-lab.org/bib/abstracts/papers/aaai02b.pdf

use std::collections::BinaryHeap;

#[derive(Default, Ord)]
pub struct Vertex<Weight>
where
    Weight: Default + Ord,
{
    pub g: Weight,
    pub rhs: Weight,
}

type Heuristic<Weight> = fn(start: &Vertex<Weight>, current: &Vertex<Weight>) -> Weight;

pub struct DStarLite<Weight>
where
    Weight: Default + Ord,
{
    pub start: Vertex<Weight>,
    pub goal: Vertex<Weight>,

    pub queue: BinaryHeap<Vertex<Weight>>,

    pub k_m: Weight,
    pub heuristic: Heuristic<Weight>,
}

impl<Weight> DStarLite<Weight> {
    fn calculate_key(&self, s: &Vertex<Weight>) -> (Weight, Weight) {
        // return [min(g(s), rhs(s)) + h(s_start, s) + k_m, min(g(s), rhs(s))]
        (
            s.g.min(s.rhs) + self.heuristic(self.start, s) + self.k_m,
            s.g.min(s.rhs),
        )
    }

    fn initialize(&self) {
        let priority_queue = BinaryHeap::new();
        let k_m = 0;
        self.goal.rhs = 0;
        priority_queue.push(self.goal, (self.heuristic(&self.start, self.goal), 0));
    }

    fn update_vertex(&self, u: Vertex, priority_queue: BinaryHeap<Vertex>) {
        // if(g(u)) != rhs(u) AND u is in U) U.Update(u, calculate_key(u))
        // if u.g != u.rhs && priority_queue.contains(u) {
        // 	priority_queue.update(u, calculate_key(s_start, u, h));
    }
}
