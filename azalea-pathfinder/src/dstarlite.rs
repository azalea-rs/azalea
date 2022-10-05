//! An implementation of D* Lite: second version (optimized version) as
//! described in https://www.cs.cmu.edu/~maxim/files/dlite_tro05.pdf

use priority_queue::PriorityQueue;
use std::{
    hash::{Hash, Hasher},
    ops::Add,
};

#[derive(Eq, PartialEq, Clone)]
pub struct Vertex<N: Eq + Hash + Clone, W: Ord + Copy> {
    pub g: W,
    pub rhs: W,
    pub node: N,
}

impl<N: Eq + Hash + Clone, W: Ord + Copy> Hash for Vertex<N, W> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.node.hash(state);
    }
}

pub struct DStarLite<N: Eq + Hash + Clone, W: Ord + Copy> {
    pub heuristic: fn(start: &Vertex<N, W>, current: &Vertex<N, W>) -> W,
    pub successors: fn(current: &Vertex<N, W>) -> Vec<Vertex<N, W>>,

    pub start: Vertex<N, W>,
    pub goal: Vertex<N, W>,

    pub queue: PriorityQueue<Vertex<N, W>, Priority<W>>,

    pub k_m: W,
}

// rust does lexicographic ordering by default when we derive Ord
#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct Priority<W>(W, W)
where
    W: Ord;

impl<N, W> DStarLite<N, W>
where
    N: Eq + Hash + Clone,
    W: Ord + Add<Output = W> + Default + Copy,
{
    fn calculate_key(&self, s: &Vertex<N, W>) -> Priority<W> {
        // return [min(g(s), rhs(s)) + h(s_start, s) + k_m, min(g(s), rhs(s))]
        Priority(
            Ord::min(s.g, s.rhs) + (self.heuristic)(&self.start, s) + self.k_m,
            Ord::min(s.g, s.rhs),
        )
    }

    fn initialize(&mut self) {
        self.queue = Default::default();
        self.k_m = W::default();
        self.goal.rhs = W::default();
        self.queue.push(
            self.goal.clone(),
            Priority((self.heuristic)(&self.start, &self.goal), W::default()),
        );
    }

    fn update_vertex(&mut self, u: Vertex<N, W>) {
        // if(g(u)) != rhs(u) AND u is in U) U.Update(u, calculate_key(u))
        if u.g != u.rhs && self.queue.get(&u).is_some() {
            self.queue.change_priority(&u, self.calculate_key(&u));
        } else if u.g != u.rhs && self.queue.get(&u).is_none() {
            let key = self.calculate_key(&u);
            self.queue.push(u, key);
        } else if u.g == u.rhs && self.queue.get(&u).is_some() {
            self.queue.remove(&u);
        }
    }

    fn compute_shortest_path(&mut self) {
        while self.queue.peek().unwrap().1 < &self.calculate_key(&self.start)
            || self.start.rhs > self.start.g
        {
            let (u, k_old) = self.queue.pop().unwrap();
            let k_new = self.calculate_key(u);
            if k_old < k_new {
                self.queue.change_priority(&u, k_new);
            } else if u.g > u.rhs {
                u.g = u.rhs;
                self.queue.remove(&u);
                // for all s in Pred(u)
                // rhs(s) = min(rhs(s), c(s, u) + g(u))
                // update_vertex(s)
                for s in self.get_predecessors(&u) {
                    s.rhs = Ord::min(s.rhs, self.get_cost(&s, &u) + u.g);
                    self.update_vertex(s);
                }
            } else {
                let g_old = u.g;
                u.g = W::max_value();
                // for all s in Pred(u) + {u}
                //   if (rhs(s) = c(s, u) + g_old)
                //     if (s != s_goal) rhs(s) = min s' in Succ(s) (c(s, s') + g(s'))
                //   update_vertex(s)
                for s in [self.get_predecessors(&u), [u]].concat() {
                    if s.rhs == self.get_cost(&s, &u) + g_old {
                        if s != self.goal {
                            s.rhs = self
                                .get_successors(&s)
                                .iter()
                                .map(|s_prime| self.get_cost(&s, s_prime) + s_prime.g)
                                .min()
                                .unwrap();
                        }
                    }
                    self.update_vertex(s);
                }
            }
        }
    }
}
