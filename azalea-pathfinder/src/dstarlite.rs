//! An implementation of D* Lite: second version (optimized version) as
//! described in https://www.cs.cmu.edu/~maxim/files/dlite_tro05.pdf
//!
//! Future optimization attempt ideas:
//! - Store the `g` and `rhs` in their own HashMap instead of being in Vertex
//! - Use a different priority queue (e.g. fibonacci heap)
//! - Use FxHash instead of the default hasher
//! - Have a `cost(a: Vertex, b: Vertex)` function instead of having the cost be stored in `Edge`

use priority_queue::PriorityQueue;
use std::{
    borrow::Cow,
    collections::HashMap,
    hash::{Hash, Hasher},
    ops::{Add, Deref},
};

#[derive(Default)]
pub struct VertexScore<W: Default> {
    pub g: W,
    pub rhs: W,
}

pub struct DStarLite<
    'a,
    N: Eq + Hash + Clone,
    W: Ord + Default + Copy,
    HeuristicFn: Fn(&N, &N) -> W,
    SuccessorsFn: Fn(&N) -> Vec<EdgeTo<N, W>>,
    PredcessorsFn: Fn(&N) -> Vec<EdgeTo<N, W>>,
> {
    /// Rough estimate of how close we are to the goal. Lower = closer.
    pub heuristic: HeuristicFn,
    /// Get the nodes that can be reached from the current one
    pub successors: SuccessorsFn,
    /// Get the nodes that would direct us to the current node
    pub predecessors: PredcessorsFn,

    pub start: Cow<'a, N>,
    start_last: Cow<'a, N>,

    goal: N,

    queue: PriorityQueue<N, Priority<W>>,
    k_m: W,
    vertex_scores: HashMap<N, VertexScore<W>>,
    /// This is just here so we can reference it. It should never be modified.
    zero_score: VertexScore<W>,

    /// A list of edges and costs that we'll be updating next time.
    pub updated_edge_costs: Vec<(Edge<'a, N, W>, W)>,
}

pub struct Edge<'a, N: Eq + Hash + Clone, W: Ord + Copy> {
    pub predecessor: Cow<'a, N>,
    pub successor: Cow<'a, N>,
    pub cost: W,
}

pub struct EdgeTo<N: Eq + Hash + Clone, W: Ord + Copy> {
    pub target: N,
    pub cost: W,
}

// rust does lexicographic ordering by default when we derive Ord
#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct Priority<W>(W, W)
where
    W: Ord;

pub struct NoPathError;

impl<
        'a,
        N: Eq + Hash + Clone,
        W: Ord + Add<Output = W> + Default + Copy + num_traits::bounds::Bounded,
        HeuristicFn: Fn(&N, &N) -> W,
        SuccessorsFn: Fn(&N) -> Vec<EdgeTo<N, W>>,
        PredecessorsFn: Fn(&N) -> Vec<EdgeTo<N, W>>,
    > DStarLite<'a, N, W, HeuristicFn, SuccessorsFn, PredecessorsFn>
{
    fn score(&self, node: &N) -> &VertexScore<W> {
        self.vertex_scores.get(node).unwrap_or(&self.zero_score)
    }
    fn score_mut(&mut self, node: &N) -> &mut VertexScore<W> {
        self.vertex_scores.entry(node.clone()).or_default()
    }

    fn calculate_key(&self, s: &N) -> Priority<W> {
        let s_score = self.score(s);
        // return [min(g(s), rhs(s)) + h(s_start, s) + k_m, min(g(s), rhs(s))]
        Priority(
            Ord::min(s_score.g, s_score.rhs) + (self.heuristic)(&self.start, s) + self.k_m,
            Ord::min(s_score.g, s_score.rhs),
        )
    }

    pub fn new(
        start: N,
        goal: N,
        heuristic: HeuristicFn,
        successors: SuccessorsFn,
        predecessors: PredecessorsFn,
    ) -> Self {
        let mut queue = PriorityQueue::with_capacity(1);
        // Vertex<N, W>, Priority<W>
        queue.push(
            goal.clone(),
            Priority(heuristic(&start, &goal), W::default()),
        );
        let mut s = Self {
            start: Cow::Owned(start.clone()),
            start_last: Cow::Owned(start),

            goal,

            heuristic,
            successors,
            predecessors,
            zero_score: VertexScore::default(),

            queue,
            k_m: W::default(),
            vertex_scores: HashMap::new(),

            updated_edge_costs: Vec::new(),
        };
        s.compute_shortest_path();
        s
    }

    pub fn update_vertex(&mut self, u: &N) {
        let VertexScore { g, rhs } = self.score(u);
        // if(g(u)) != rhs(u) AND u is in U) U.Update(u, calculate_key(u))
        if g != rhs && self.queue.get(&u).is_some() {
            self.queue.change_priority(&u, self.calculate_key(&u));
        } else if g != rhs && self.queue.get(&u).is_none() {
            let key = self.calculate_key(&u);
            self.queue.push(u.clone(), key);
        } else if g == rhs && self.queue.get(&u).is_some() {
            self.queue.remove(&u);
        }
    }

    fn compute_shortest_path(&mut self) {
        while {
            let VertexScore {
                g: start_g,
                rhs: start_rhs,
            } = self.score(&self.start);
            self.queue.peek().unwrap().1 < &self.calculate_key(&self.start) || start_rhs > start_g
        } {
            let (u, k_old) = self.queue.pop().unwrap();
            let k_new = self.calculate_key(&u);
            if k_old < k_new {
                self.queue.change_priority(&u, k_new);
                continue;
            }
            let u_score = self.score_mut(&u);
            if u_score.g > u_score.rhs {
                u_score.g = u_score.rhs;
                self.queue.remove(&u);
                // for all s in Pred(u)
                // rhs(s) = min(rhs(s), c(s, u) + g(u))
                // update_vertex(s)
                for edge in (self.predecessors)(&u) {
                    let u_g_score = self.score(&u).g;
                    let target_score = self.score_mut(&edge.target);
                    target_score.rhs = Ord::min(target_score.rhs, edge.cost + u_g_score);
                    self.update_vertex(&edge.target);
                }
            } else {
                let g_old = u_score.g;
                u_score.g = W::max_value();
                // for all s in Pred(u) + {u}
                //   if (rhs(s) = c(s, u) + g_old)
                //     if (s != s_goal) rhs(s) = min s' in Succ(s) (c(s, s') + g(s'))
                //   update_vertex(s)
                for pred_edge in ((self.predecessors)(&u)).into_iter().chain(
                    [EdgeTo {
                        target: u,
                        cost: W::default(),
                    }]
                    .into_iter(),
                ) {
                    if self.score(&pred_edge.target).rhs == pred_edge.cost + g_old {
                        if self.goal != pred_edge.target {
                            let successors = (self.successors)(&pred_edge.target);
                            self.score_mut(&pred_edge.target).rhs = successors
                                .iter()
                                .map(|successor_edge| {
                                    successor_edge.cost + self.score(&successor_edge.target).g
                                })
                                .min()
                                .unwrap();
                        }
                    }
                    self.update_vertex(&pred_edge.target);
                }
            }
        }
    }

    pub fn update_from_updated_edges(&mut self) {
        self.k_m = self.k_m + (self.heuristic)(&self.start, &self.start_last);
        self.start_last = self.start.to_owned();

        while let Some((mut edge, new_cost)) = self.updated_edge_costs.pop() {
            let old_cost = edge.cost;
            edge.cost = new_cost;
            let target_score = self.score_mut(&edge.successor);
            if old_cost > new_cost {
                target_score.rhs = Ord::min(target_score.rhs, edge.cost + target_score.g);
            } else if target_score.rhs == old_cost + target_score.g {
                let g_score = target_score.g;
                if edge.successor.deref() != &self.goal {
                    let successors = (self.successors)(&edge.successor);
                    self.score_mut(&edge.successor).rhs =
                        successors.iter().map(|s| s.cost + g_score).min().unwrap();
                }
            }
            self.update_vertex(&edge.successor);
        }
    }

    /// Return the next vertex to visit and set our current position to be there.
    pub fn next(&mut self) -> Result<Option<&N>, NoPathError> {
        while self.start.deref() != &self.goal {
            let start_score = self.score(&self.start);
            if start_score.rhs == W::max_value() {
                return Err(NoPathError);
            }

            *self.start.to_mut() = (self.successors)(&self.start)
                .into_iter()
                .min_by(|a, b| a.cost.cmp(&self.score(&b.target).g))
                .expect("No possible successors")
                .target;
            return Ok(Some(self.start.as_ref()));
        }

        Ok(None)
    }

    // /// Change our current position.
    // pub fn set_start(&mut self, s: Vertex<N, W>) {
    //     *self.start.to_mut() = s;
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dstarlite() {
        let maze = [
            [0, 1, 0, 0, 0],
            [0, 1, 0, 1, 0],
            [0, 0, 0, 1, 0],
            [0, 1, 0, 1, 0],
            [0, 0, 1, 0, 0],
        ];
        let width = maze[0].len();
        let height = maze.len();

        fn heuristic(a: &(usize, usize), b: &(usize, usize)) -> usize {
            ((a.0 as isize - b.0 as isize).abs() + (a.1 as isize - b.1 as isize).abs()) as usize
        }
        let successors = |a: &(usize, usize)| -> Vec<EdgeTo<(usize, usize), usize>> {
            let mut successors = Vec::with_capacity(4);
            let (x, y) = *a;

            if x > 0 && maze[y][x - 1] == 0 {
                successors.push(EdgeTo {
                    target: ((x - 1, y)),
                    cost: 1,
                });
            }
            if x < width - 1 && maze[y][x + 1] == 0 {
                successors.push(EdgeTo {
                    target: ((x + 1, y)),
                    cost: 1,
                });
            }
            if y > 0 && maze[y - 1][x] == 0 {
                successors.push(EdgeTo {
                    target: ((x, y - 1)),
                    cost: 1,
                });
            }
            if y < height - 1 && maze[y + 1][x] == 0 {
                successors.push(EdgeTo {
                    target: ((x, y + 1)),
                    cost: 1,
                });
            }

            successors
        };
        let predecessors = |a: &(usize, usize)| -> Vec<EdgeTo<(usize, usize), usize>> {
            let mut predecessors = Vec::with_capacity(4);
            let (x, y) = *a;

            if x > 0 && maze[y][x - 1] == 0 {
                predecessors.push(EdgeTo {
                    target: ((x - 1, y)),
                    cost: 1,
                });
            }
            if x < width - 1 && maze[y][x + 1] == 0 {
                predecessors.push(EdgeTo {
                    target: ((x + 1, y)),
                    cost: 1,
                });
            }
            if y > 0 && maze[y - 1][x] == 0 {
                predecessors.push(EdgeTo {
                    target: ((x, y - 1)),
                    cost: 1,
                });
            }
            if y < height - 1 && maze[y + 1][x] == 0 {
                predecessors.push(EdgeTo {
                    target: ((x, y + 1)),
                    cost: 1,
                });
            }

            predecessors
        };

        let mut dstar = DStarLite::new((0, 0), (4, 4), heuristic, successors, predecessors);
        println!("getting move");
        while let Ok(Some(pos)) = dstar.next() {
            println!("{:?}", pos);
        }
        panic!()
    }
}
