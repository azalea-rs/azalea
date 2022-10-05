//! An implementation of D* Lite: second version (optimized version) as
//! described in https://www.cs.cmu.edu/~maxim/files/dlite_tro05.pdf

use priority_queue::PriorityQueue;
use std::{
    borrow::Cow,
    hash::{Hash, Hasher},
    ops::{Add, Deref},
};

#[derive(Eq, PartialEq, Clone)]
pub struct Vertex<N: Eq + Hash + Clone, W: Ord + Copy> {
    pub node: N,
    pub g: W,
    pub rhs: W,
}

impl<N: Eq + Hash + Clone, W: Ord + Copy + Default> Vertex<N, W> {
    pub fn new(node: N) -> Self {
        Self {
            node,
            rhs: W::default(),
            g: W::default(),
        }
    }
}

impl<N: Eq + Hash + Clone, W: Ord + Copy> Hash for Vertex<N, W> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.node.hash(state);
    }
}

pub type HeuristicFn<N, W> = fn(start: &Vertex<N, W>, current: &Vertex<N, W>) -> W;

pub type EdgesFn<N, W> = fn(current: &Vertex<N, W>) -> Vec<Edge<N, W>>;

pub struct DStarLite<'a, N: Eq + Hash + Clone, W: Ord + Copy> {
    /// Rough estimate of how close we are to the goal. Lower = closer.
    pub heuristic: HeuristicFn<N, W>,
    /// Get the nodes that can be reached from the current one
    pub successors: EdgesFn<N, W>,
    /// Get the nodes that would direct us to the current node
    pub predecessors: EdgesFn<N, W>,

    pub start: Cow<'a, Vertex<N, W>>,
    pub goal: Vertex<N, W>,

    pub queue: PriorityQueue<Vertex<N, W>, Priority<W>>,

    pub k_m: W,
}

pub struct Edge<'a, N: Eq + Hash + Clone, W: Ord + Copy> {
    pub predecessor: Cow<'a, Vertex<N, W>>,
    pub successor: Cow<'a, Vertex<N, W>>,
    pub cost: W,
}

// rust does lexicographic ordering by default when we derive Ord
#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct Priority<W>(W, W)
where
    W: Ord;

pub trait Max {
    const MAX: Self;
}

impl<N, W> DStarLite<'_, N, W>
where
    N: Eq + Hash + Clone,
    W: Ord + Add<Output = W> + Default + Copy + Max,
{
    fn calculate_key(&self, s: &Vertex<N, W>) -> Priority<W> {
        // return [min(g(s), rhs(s)) + h(s_start, s) + k_m, min(g(s), rhs(s))]
        Priority(
            Ord::min(s.g, s.rhs) + (self.heuristic)(&self.start, s) + self.k_m,
            Ord::min(s.g, s.rhs),
        )
    }

    pub fn new(
        start: N,
        goal: N,
        heuristic: HeuristicFn<N, W>,
        successors: EdgesFn<N, W>,
        predecessors: EdgesFn<N, W>,
    ) -> Self {
        let start_vertex = Vertex::new(start);
        let goal_vertex = Vertex::new(goal);

        let mut queue = PriorityQueue::with_capacity(1);
        // Vertex<N, W>, Priority<W>
        queue.push(
            goal_vertex.clone(),
            Priority(heuristic(&start_vertex, &goal_vertex), W::default()),
        );
        Self {
            start: Cow::Owned(start_vertex),
            goal: goal_vertex,

            heuristic,
            successors,
            predecessors,

            queue,
            k_m: W::default(),
            // self.goal.rhs = W::default(),
        }
    }

    pub fn update_vertex(&mut self, u: &Vertex<N, W>) {
        // if(g(u)) != rhs(u) AND u is in U) U.Update(u, calculate_key(u))
        if u.g != u.rhs && self.queue.get(&u).is_some() {
            self.queue.change_priority(&u, self.calculate_key(&u));
        } else if u.g != u.rhs && self.queue.get(&u).is_none() {
            let key = self.calculate_key(&u);
            self.queue.push(u.clone(), key);
        } else if u.g == u.rhs && self.queue.get(&u).is_some() {
            self.queue.remove(&u);
        }
    }

    pub fn compute_shortest_path(&mut self) {
        while self.queue.peek().unwrap().1 < &self.calculate_key(&self.start)
            || self.start.rhs > self.start.g
        {
            let (mut u, k_old) = self.queue.pop().unwrap();
            let k_new = self.calculate_key(&u);
            if k_old < k_new {
                self.queue.change_priority(&u, k_new);
            } else if u.g > u.rhs {
                u.g = u.rhs;
                self.queue.remove(&u);
                // for all s in Pred(u)
                // rhs(s) = min(rhs(s), c(s, u) + g(u))
                // update_vertex(s)
                for mut edge in (self.predecessors)(&u) {
                    edge.predecessor.to_mut().rhs = Ord::min(edge.predecessor.rhs, edge.cost + u.g);
                    self.update_vertex(&edge.predecessor);
                }
            } else {
                let g_old = u.g;
                u.g = W::MAX;
                // for all s in Pred(u) + {u}
                //   if (rhs(s) = c(s, u) + g_old)
                //     if (s != s_goal) rhs(s) = min s' in Succ(s) (c(s, s') + g(s'))
                //   update_vertex(s)
                for mut predecessor_edge in ((self.predecessors)(&u)).into_iter().chain(
                    [Edge {
                        predecessor: Cow::Borrowed(&u),
                        successor: Cow::Borrowed(&u),
                        cost: W::default(),
                    }]
                    .into_iter(),
                ) {
                    if predecessor_edge.predecessor.rhs == predecessor_edge.cost + g_old {
                        if &self.goal != predecessor_edge.predecessor.deref() {
                            predecessor_edge.predecessor.to_mut().rhs =
                                (self.successors)(&predecessor_edge.predecessor)
                                    .iter()
                                    .map(|successor_edge| {
                                        successor_edge.cost + successor_edge.successor.g
                                    })
                                    .min()
                                    .unwrap();
                        }
                    }
                    self.update_vertex(&predecessor_edge.predecessor);
                }
            }
        }
    }

    // fn main(&mut self) {
    //     let s_last = &self.start;
    //     self.compute_shortest_path();
    //     while self.start.deref() != &self.goal {
    //         self.start = (self.successors)(&self.start)
    //             .into_iter()
    //             .min_by(|a, b| a.cost.cmp(&b.cost))
    //             .expect("No possible successors")
    //             .successor;
    //         if self.start.rhs == W::MAX {
    //             // no path
    //         }
    //     }
    // }
}
