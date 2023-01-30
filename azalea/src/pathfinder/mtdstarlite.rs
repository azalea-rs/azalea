//! An implementation of Moving Target D* Lite as described in
//! <http://idm-lab.org/bib/abstracts/papers/aamas10a.pdf>
//!
//! Future optimization attempt ideas:
//! - Use a different priority queue (e.g. fibonacci heap)
//! - Use `FxHash` instead of the default hasher
//! - Have `par` be a raw pointer
//! - Try borrowing vs copying the Node in several places (like `state_mut`)
//! - Store edge costs in their own map

use priority_queue::DoublePriorityQueue;
use std::{collections::HashMap, fmt::Debug, hash::Hash, ops::Add};

/// Nodes are coordinates.
pub struct MTDStarLite<
    N: Eq + Hash + Copy + Debug,
    W: PartialOrd + Default + Copy + num_traits::Bounded + Debug,
    HeuristicFn: Fn(&N) -> W,
    SuccessorsFn: Fn(&N) -> Vec<Edge<N, W>>,
    PredecessorsFn: Fn(&N) -> Vec<Edge<N, W>>,
    SuccessFn: Fn(&N) -> bool,
> {
    /// Returns a rough estimate of how close we are to the goal. Lower =
    /// closer.
    pub heuristic: HeuristicFn,
    /// Returns the nodes that can be reached from the given node.
    pub successors: SuccessorsFn,
    /// Returns the nodes that would direct us to the given node. If the graph
    /// isn't directed (i.e. you can always return to the previous node), this
    /// can be the same as `successors`.
    pub predecessors: PredecessorsFn,
    /// Returns true if the given node is at the goal.
    /// A simple implementation is to check if the given node is equal to the
    /// goal.
    pub success: SuccessFn,

    start: N,
    goal: N,

    old_start: N,
    old_goal: N,

    k_m: W,
    open: DoublePriorityQueue<N, Priority<W>>,
    node_states: HashMap<N, NodeState<N, W>>,
    updated_edge_costs: Vec<ChangedEdge<N, W>>,

    /// This only exists so it can be referenced by `state()` when there's no
    /// state.
    default_state: NodeState<N, W>,
}

impl<
        N: Eq + Hash + Copy + Debug,
        W: PartialOrd + Add<Output = W> + Default + Copy + num_traits::Bounded + Debug,
        HeuristicFn: Fn(&N) -> W,
        SuccessorsFn: Fn(&N) -> Vec<Edge<N, W>>,
        PredecessorsFn: Fn(&N) -> Vec<Edge<N, W>>,
        SuccessFn: Fn(&N) -> bool,
    > MTDStarLite<N, W, HeuristicFn, SuccessorsFn, PredecessorsFn, SuccessFn>
{
    fn calculate_key(&self, n: &N) -> Priority<W> {
        let s = self.state(n);
        let min_score = if s.g < s.rhs { s.g } else { s.rhs };
        Priority(
            if min_score == W::max_value() {
                min_score
            } else {
                min_score + (self.heuristic)(n) + self.k_m
            },
            min_score,
        )
    }

    pub fn new(
        start: N,
        goal: N,
        heuristic: HeuristicFn,
        successors: SuccessorsFn,
        predecessors: PredecessorsFn,
        success: SuccessFn,
    ) -> Self {
        let open = DoublePriorityQueue::default();
        let k_m = W::default();

        let known_nodes = vec![start, goal];

        let mut pf = MTDStarLite {
            heuristic,
            successors,
            predecessors,
            success,

            start,
            goal,

            old_start: start,
            old_goal: goal,

            k_m,
            open,
            node_states: HashMap::new(),
            updated_edge_costs: Vec::new(),

            default_state: NodeState::default(),
        };

        for n in &known_nodes {
            *pf.state_mut(n) = NodeState::default();
        }
        pf.state_mut(&start).rhs = W::default();
        pf.open.push(start, pf.calculate_key(&start));

        pf
    }

    fn update_state(&mut self, n: &N) {
        let u = self.state_mut(n);
        if u.g != u.rhs {
            if self.open.get(n).is_some() {
                self.open.change_priority(n, self.calculate_key(n));
            } else {
                self.open.push(*n, self.calculate_key(n));
            }
        } else if self.open.get(n).is_some() {
            self.open.remove(n);
        }
    }

    fn compute_cost_minimal_path(&mut self) {
        while {
            if let Some((_, top_key)) = self.open.peek_min() {
                (top_key < &self.calculate_key(&self.goal)) || {
                    let goal_state = self.state(&self.goal);
                    goal_state.rhs > goal_state.g
                }
            } else {
                false
            }
        } {
            let (u_node, k_old) = self.open.pop_min().unwrap();
            let k_new = self.calculate_key(&u_node);
            if k_old < k_new {
                self.open.change_priority(&u_node, k_new);
                continue;
            }
            let u = self.state_mut(&u_node);
            if u.g > u.rhs {
                u.g = u.rhs;
                self.open.remove(&u_node);
                for edge in (self.successors)(&u_node) {
                    let s_node = edge.target;
                    let s = self.state(&s_node);
                    let u = self.state(&u_node);
                    if s_node != self.start && (s.rhs > u.g + edge.cost) {
                        let s_rhs = u.g + edge.cost;
                        let s = self.state_mut(&s_node);
                        s.par = Some(u_node);
                        s.rhs = s_rhs;
                        self.update_state(&s_node);
                    }
                }
            } else {
                u.g = W::max_value();
                let u_edge = Edge {
                    target: u_node,
                    cost: W::default(),
                };
                for edge in (self.successors)(&u_node)
                    .iter()
                    .chain([&u_edge].into_iter())
                {
                    let s_node = edge.target;
                    let s = self.state(&s_node);
                    if s_node != self.start && s.par == Some(u_node) {
                        let mut min_pred = u_node;
                        let mut min_score = W::max_value();

                        for edge in (self.predecessors)(&s_node) {
                            let s = self.state(&edge.target);
                            let score = s.g + edge.cost;
                            if score < min_score {
                                min_score = score;
                                min_pred = edge.target;
                            }
                        }

                        let s = self.state_mut(&s_node);
                        s.rhs = min_score;
                        if s.rhs == W::max_value() {
                            s.par = None;
                        } else {
                            s.par = Some(min_pred);
                        }
                    }
                    self.update_state(&s_node);
                }
            }
        }
    }

    pub fn find_path(&mut self) -> Option<Vec<N>> {
        if (self.success)(&self.start) {
            return None;
        }

        //
        self.k_m = self.k_m + (self.heuristic)(&self.old_goal);

        if self.old_start != self.start {
            self.optimized_deletion();
        }

        while let Some(edge) = self.updated_edge_costs.pop() {
            let (u_node, v_node) = (edge.predecessor, edge.successor);
            // update the edge cost c(u, v);
            if edge.old_cost > edge.cost {
                let u_g = self.state(&u_node).g;
                if v_node != self.start && self.state(&v_node).rhs > u_g + edge.cost {
                    let v = self.state_mut(&v_node);
                    v.par = Some(u_node);
                    v.rhs = u_g + edge.cost;
                }
            } else if v_node != self.start && self.state(&v_node).par == Some(u_node) {
                let mut min_pred = u_node;
                let mut min_score = W::max_value();

                for edge in (self.predecessors)(&v_node) {
                    let s = self.state(&edge.target);
                    let score = s.g + edge.cost;
                    if score < min_score {
                        min_score = score;
                        min_pred = edge.target;
                    }
                }

                let v = self.state_mut(&v_node);
                v.rhs = min_score;
                if v.rhs == W::max_value() {
                    v.par = None;
                } else {
                    v.par = Some(min_pred);
                }
                self.update_state(&v_node);
            }
        }
        //

        self.old_start = self.start;
        self.old_goal = self.goal;

        self.compute_cost_minimal_path();
        if self.state(&self.goal).rhs == W::max_value() {
            // no path exists
            return None;
        }

        let mut reverse_path = vec![self.goal];

        // identify a path from sstart to sgoal using the parent pointers
        let mut target = self.state(&self.goal).par;
        while !(Some(self.start) == target) {
            let Some(this_target) = target else {
                break;
            };
            // hunter follows path from start to goal;
            reverse_path.push(this_target);
            target = self.state(&this_target).par;
        }

        // if hunter caught target {
        //     return None;
        // }

        let path: Vec<N> = reverse_path.into_iter().rev().collect();

        Some(path)
    }

    fn optimized_deletion(&mut self) {
        let start = self.start;
        self.state_mut(&start).par = None;

        let mut min_pred = self.old_start;
        let mut min_score = W::max_value();

        for edge in (self.predecessors)(&self.old_start) {
            let s = self.state(&edge.target);
            let score = s.g + edge.cost;
            if score < min_score {
                min_score = score;
                min_pred = edge.target;
            }
        }

        let old_start = self.old_start;
        let s = self.state_mut(&old_start);
        s.rhs = min_score;
        if s.rhs == W::max_value() {
            s.par = None;
        } else {
            s.par = Some(min_pred);
        }
        self.update_state(&old_start);
    }

    fn state(&self, n: &N) -> &NodeState<N, W> {
        self.node_states.get(n).unwrap_or(&self.default_state)
    }

    fn state_mut(&mut self, n: &N) -> &mut NodeState<N, W> {
        self.node_states.entry(*n).or_default()
    }
}

#[derive(PartialEq, Debug)]
pub struct Priority<W>(W, W)
where
    W: PartialOrd + Debug;

impl<W: PartialOrd + Debug> PartialOrd for Priority<W> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.0 < other.0 {
            Some(std::cmp::Ordering::Less)
        } else if self.0 > other.0 {
            Some(std::cmp::Ordering::Greater)
        } else if self.1 < other.1 {
            Some(std::cmp::Ordering::Less)
        } else if self.1 > other.1 {
            Some(std::cmp::Ordering::Greater)
        } else {
            Some(std::cmp::Ordering::Equal)
        }
    }
}
impl<W: PartialOrd + Debug> Ord for Priority<W> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other)
            .expect("Partial compare should not fail for Priority")
    }
}
impl<W: PartialOrd + Debug> Eq for Priority<W> {}

#[derive(Debug)]
pub struct NodeState<N: Eq + Hash + Copy + Debug, W: Default + num_traits::Bounded + Debug> {
    pub g: W,
    pub rhs: W,
    // future possible optimization: try making this a pointer
    pub par: Option<N>,
}

impl<N: Eq + Hash + Copy + Debug, W: Default + num_traits::Bounded + Debug> Default
    for NodeState<N, W>
{
    fn default() -> Self {
        NodeState {
            g: W::max_value(),
            rhs: W::max_value(),
            par: None,
        }
    }
}

pub struct Edge<N: Eq + Hash + Copy, W: PartialOrd + Copy> {
    pub target: N,
    pub cost: W,
}

pub struct ChangedEdge<N: Eq + Hash + Clone, W: PartialOrd + Copy> {
    pub predecessor: N,
    pub successor: N,
    pub old_cost: W,
    pub cost: W,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mtdstarlite() {
        let maze = [
            [0, 1, 0, 0, 0],
            [0, 1, 0, 1, 0],
            [0, 0, 0, 1, 0],
            [0, 1, 0, 1, 0],
            [0, 0, 1, 0, 0],
        ];
        let width = maze[0].len();
        let height = maze.len();

        let goal = (4, 4);

        let heuristic = |n: &(usize, usize)| -> usize {
            ((n.0 as isize - goal.0 as isize).abs() + (n.1 as isize - goal.1 as isize).abs())
                as usize
        };
        let successors = |n: &(usize, usize)| -> Vec<Edge<(usize, usize), usize>> {
            let mut successors = Vec::with_capacity(4);
            let (x, y) = *n;

            if x > 0 && maze[y][x - 1] == 0 {
                successors.push(Edge {
                    target: ((x - 1, y)),
                    cost: 1,
                });
            }
            if x < width - 1 && maze[y][x + 1] == 0 {
                successors.push(Edge {
                    target: ((x + 1, y)),
                    cost: 1,
                });
            }
            if y > 0 && maze[y - 1][x] == 0 {
                successors.push(Edge {
                    target: ((x, y - 1)),
                    cost: 1,
                });
            }
            if y < height - 1 && maze[y + 1][x] == 0 {
                successors.push(Edge {
                    target: ((x, y + 1)),
                    cost: 1,
                });
            }

            successors
        };
        let predecessors =
            |n: &(usize, usize)| -> Vec<Edge<(usize, usize), usize>> { successors(n) };

        let mut pf = MTDStarLite::new((0, 0), goal, heuristic, successors, predecessors, |n| {
            n == &goal
        });
        let path = pf.find_path().unwrap();
        assert_eq!(
            path,
            vec![
                (0, 1),
                (0, 2),
                (1, 2),
                (2, 2),
                (2, 1),
                (2, 0),
                (3, 0),
                (4, 0),
                (4, 1),
                (4, 2),
                (4, 3),
                (4, 4),
            ]
        );
    }
}
