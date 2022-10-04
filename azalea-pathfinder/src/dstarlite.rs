//! An implementation of D* Lite (optimized version) as described in
//! http://idm-lab.org/bib/abstracts/papers/aaai02b.pdf
//!
//! Note that the words "vertex" and "key" from the paper are replaced with
//! "node" and "priority" in this code because I think these terms better
//! represent their purpose and are more commonly used when describing similar
//! algorithms.

use priority_queue::PriorityQueue;
use std::collections::BinaryHeap;

#[derive(Default, Eq, PartialEq, Hash)]
pub struct Node<W>
where
    W: Default + Ord,
{
    pub g: W,
    pub rhs: W,
}

type Heuristic<W>
where
    W: Default + Ord,
= fn(start: &Node<W>, current: &Node<W>) -> W;

pub struct DStarLite<W>
where
    W: Default + Ord,
{
    pub start: Node<W>,
    pub goal: Node<W>,

    pub queue: PriorityQueue<Node<W>, Priority<W>>,

    pub k_m: W,
    pub heuristic: Heuristic<W>,
}

// rust does lexicographic ordering by default when we derive Ord
#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct Priority<W>(W, W)
where
    W: Default + Ord;

impl<W> DStarLite<W>
where
    W: Default + Ord,
{
    fn calculate_key(&self, s: &Node<W>) -> Priority<W> {
        // return [min(g(s), rhs(s)) + h(s_start, s) + k_m, min(g(s), rhs(s))]
        Priority(
            s.g.min(s.rhs) + self.heuristic(self.start, s) + self.k_m,
            s.g.min(s.rhs),
        )
    }

    fn initialize(&self) {
        let priority_queue = BinaryHeap::new();
        let k_m = 0;
        self.goal.rhs = 0;
        priority_queue.push(
            self.goal,
            Priority(self.heuristic(&self.start, self.goal), 0),
        );
    }

    fn update_vertex(&self, u: Node<W>, priority_queue: PriorityQueue<Node<W>, Priority<W>>) {
        // if(g(u)) != rhs(u) AND u is in U) U.Update(u, calculate_key(u))
        if u.g != u.rhs && priority_queue.contains(u) {
            priority_queue.update(u, self.calculate_key(s_start, u, h));
        }
    }
}
