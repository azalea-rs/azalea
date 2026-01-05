use std::{
    cmp::{self, Reverse},
    collections::BinaryHeap,
    fmt::{self, Debug},
    hash::{BuildHasherDefault, Hash},
    time::{Duration, Instant},
};

use indexmap::IndexMap;
use num_format::ToFormattedString;
use radix_heap::RadixHeapMap;
use rustc_hash::FxHasher;
use tracing::{debug, trace, warn};

pub struct Path<P, M>
where
    P: Eq + Hash + Copy + Debug,
{
    pub movements: Vec<Movement<P, M>>,
    pub is_partial: bool,
    /// The A* cost for executing the path.
    ///
    /// For Azalea's pathfinder, this is generally the estimated amount of time
    /// that it takes to complete the path, in ticks.
    pub cost: f32,
}

// used for better results when timing out
// see https://github.com/cabaletta/baritone/blob/1.19.4/src/main/java/baritone/pathing/calc/AbstractNodeCostSearch.java#L68
const COEFFICIENTS: [f32; 7] = [1.5, 2., 2.5, 3., 4., 5., 10.];

const MIN_IMPROVEMENT: f32 = 0.01;

type FxIndexMap<K, V> = IndexMap<K, V, BuildHasherDefault<FxHasher>>;

// Sources:
// - https://en.wikipedia.org/wiki/A*_search_algorithm
// - https://github.com/evenfurther/pathfinding/blob/main/src/directed/astar.rs
// - https://github.com/cabaletta/baritone/blob/1.19.4/src/main/java/baritone/pathing/calc/AbstractNodeCostSearch.java
pub fn a_star<P, M, HeuristicFn, SuccessorsFn, SuccessFn>(
    start: P,
    heuristic: HeuristicFn,
    mut successors: SuccessorsFn,
    success: SuccessFn,
    min_timeout: PathfinderTimeout,
    max_timeout: PathfinderTimeout,
) -> Path<P, M>
where
    P: Eq + Hash + Copy + Debug,
    HeuristicFn: Fn(P) -> f32,
    SuccessorsFn: FnMut(P) -> Vec<Edge<P, M>>,
    SuccessFn: Fn(P) -> bool,
{
    let start_time = Instant::now();

    let mut open_set = PathfinderHeap::new();
    open_set.push(WeightedNode {
        g_score: 0.,
        f_score: 0.,
        index: 0,
    });
    let mut nodes: FxIndexMap<P, Node> = IndexMap::default();
    nodes.insert(
        start,
        Node {
            came_from: u32::MAX,
            g_score: 0.,
        },
    );

    let mut best_paths: [u32; 7] = [0; 7];
    let mut best_path_scores: [f32; 7] = [heuristic(start); 7];

    let mut num_nodes = 0_usize;
    let mut num_movements = 0;

    while let Some(WeightedNode { index, g_score, .. }) = open_set.pop() {
        let (&node, node_data) = nodes.get_index(index as usize).unwrap();
        if g_score > node_data.g_score {
            continue;
        }

        num_nodes += 1;

        if success(node) {
            let best_path = index;
            log_perf_info(start_time, num_nodes, num_movements);

            return Path {
                movements: reconstruct_path(nodes, best_path, successors),
                is_partial: false,
                cost: g_score,
            };
        }

        for neighbor in successors(node) {
            let tentative_g_score = g_score + neighbor.cost;
            // let neighbor_heuristic = heuristic(neighbor.movement.target);
            let neighbor_heuristic;
            let neighbor_index;

            num_movements += 1;

            match nodes.entry(neighbor.movement.target) {
                indexmap::map::Entry::Occupied(mut e) => {
                    if e.get().g_score > tentative_g_score {
                        neighbor_heuristic = heuristic(*e.key());
                        neighbor_index = e.index() as u32;
                        e.insert(Node {
                            came_from: index,
                            g_score: tentative_g_score,
                        });
                    } else {
                        continue;
                    }
                }
                indexmap::map::Entry::Vacant(e) => {
                    neighbor_heuristic = heuristic(*e.key());
                    neighbor_index = e.index() as u32;
                    e.insert(Node {
                        came_from: index,
                        g_score: tentative_g_score,
                    });
                }
            }

            // we don't update the existing node, which means that the same node might be
            // present in the open_set multiple times. this is fine because at the start of
            // the loop we check `g_score > node_data.g_score`.
            open_set.push(WeightedNode {
                index: neighbor_index,
                g_score: tentative_g_score,
                f_score: tentative_g_score + neighbor_heuristic,
            });

            for (coefficient_i, &coefficient) in COEFFICIENTS.iter().enumerate() {
                let node_score = neighbor_heuristic + tentative_g_score / coefficient;
                if best_path_scores[coefficient_i] - node_score > MIN_IMPROVEMENT {
                    best_paths[coefficient_i] = neighbor_index;
                    best_path_scores[coefficient_i] = node_score;
                }
            }
        }

        // check for timeout every ~10ms
        if num_nodes.is_multiple_of(10_000) {
            let min_timeout_reached = match min_timeout {
                PathfinderTimeout::Time(max_duration) => start_time.elapsed() >= max_duration,
                PathfinderTimeout::Nodes(max_nodes) => num_nodes >= max_nodes,
            };

            if min_timeout_reached {
                // means we have a non-empty path
                if best_paths[6] != 0 {
                    break;
                }

                if min_timeout_reached {
                    let max_timeout_reached = match max_timeout {
                        PathfinderTimeout::Time(max_duration) => {
                            start_time.elapsed() >= max_duration
                        }
                        PathfinderTimeout::Nodes(max_nodes) => num_nodes >= max_nodes,
                    };

                    if max_timeout_reached {
                        // timeout, we're gonna be returning an empty path :(
                        trace!("A* couldn't find a path in time, returning best path");
                        break;
                    }
                }
            }
        }
    }

    let best_path_idx = determine_best_path_idx(best_paths, 0);
    log_perf_info(start_time, num_nodes, num_movements);
    Path {
        movements: reconstruct_path(nodes, best_paths[best_path_idx], successors),
        is_partial: true,
        cost: best_path_scores[best_path_idx],
    }
}

fn log_perf_info(start_time: Instant, num_nodes: usize, num_movements: usize) {
    let elapsed = start_time.elapsed();
    let elapsed_seconds = elapsed.as_secs_f64();
    let nodes_per_second = (num_nodes as f64 / elapsed_seconds) as u64;
    let num_movements_per_second = (num_movements as f64 / elapsed_seconds) as u64;
    debug!(
        "Considered {} nodes in {elapsed:?}",
        num_nodes.to_formatted_string(&num_format::Locale::en)
    );
    debug!(
        "A* ran at {} nodes per second and {} movements per second",
        nodes_per_second.to_formatted_string(&num_format::Locale::en),
        num_movements_per_second.to_formatted_string(&num_format::Locale::en),
    );
}

fn determine_best_path_idx(best_paths: [u32; 7], start: u32) -> usize {
    // this basically makes sure we don't create a path that's really short

    for (i, &node) in best_paths.iter().enumerate() {
        if node != start {
            return i;
        }
    }
    warn!("No best node found, returning first node");
    0
}

fn reconstruct_path<P, M, SuccessorsFn>(
    nodes: FxIndexMap<P, Node>,
    mut current_index: u32,
    mut successors: SuccessorsFn,
) -> Vec<Movement<P, M>>
where
    P: Eq + Hash + Copy + Debug,
    SuccessorsFn: FnMut(P) -> Vec<Edge<P, M>>,
{
    let mut path = Vec::new();
    while let Some((&node_position, node)) = nodes.get_index(current_index as usize) {
        if node.came_from == u32::MAX {
            break;
        }
        let came_from_position = *nodes.get_index(node.came_from as usize).unwrap().0;

        // find the movement data for this successor, we have to do this again because
        // we don't include the movement data in the Node (as an optimization)
        let mut best_successor = None;
        let mut best_successor_cost = f32::INFINITY;
        for successor in successors(came_from_position) {
            if successor.movement.target == node_position && successor.cost < best_successor_cost {
                best_successor_cost = successor.cost;
                best_successor = Some(successor);
            }
        }
        let Some(found_successor) = best_successor else {
            warn!(
                "a successor stopped being possible while reconstructing the path, returning empty path"
            );
            return vec![];
        };

        path.push(Movement {
            target: node_position,
            data: found_successor.movement.data,
        });

        current_index = node.came_from;
    }
    path.reverse();
    path
}

pub struct Node {
    pub came_from: u32,
    pub g_score: f32,
}

#[derive(Clone, Debug)]
pub struct Edge<P: Hash + Copy, M> {
    pub movement: Movement<P, M>,
    pub cost: f32,
}

pub struct Movement<P: Hash + Copy, M> {
    pub target: P,
    pub data: M,
}

impl<P: Hash + Copy + Debug, M: Debug> Debug for Movement<P, M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Movement")
            .field("target", &self.target)
            .field("data", &self.data)
            .finish()
    }
}
impl<P: Hash + Copy + Clone, M: Clone> Clone for Movement<P, M> {
    fn clone(&self) -> Self {
        Self {
            target: self.target,
            data: self.data.clone(),
        }
    }
}

#[derive(Default)]
struct PathfinderHeap {
    binary_heap: BinaryHeap<WeightedNode>,
    /// Key is f_score.to_bits(), value is (g_score, index)
    ///
    /// As long as the f_score is positive, comparing it as bits is fine. Also,
    /// it has to be `Reverse`d to make it a min-heap.
    radix_heap: RadixHeapMap<Reverse<u32>, (f32, u32)>,
}
impl PathfinderHeap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, item: WeightedNode) {
        if let Some(top) = self.radix_heap.top() {
            // this can happen when the heuristic isn't optimal, so just fall back to a
            // binary heap in those cases
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

#[derive(PartialEq)]
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

/// A timeout that the pathfinder will consider when calculating a path.
///
/// See [`PathfinderOpts::min_timeout`] and [`PathfinderOpts::max_timeout`] if
/// you want to modify this.
///
/// [`PathfinderOpts::min_timeout`]: super::goto_event::PathfinderOpts::min_timeout
/// [`PathfinderOpts::max_timeout`]: super::goto_event::PathfinderOpts::max_timeout
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PathfinderTimeout {
    /// Time out after a certain duration has passed.
    ///
    /// This is a good default so you don't waste too much time calculating a
    /// path if you're on a slow computer.
    Time(Duration),
    /// Time out after this many nodes have been considered.
    ///
    /// This is useful as an alternative to a time limit if you're doing
    /// something like running tests where you want consistent results.
    Nodes(usize),
}
impl Default for PathfinderTimeout {
    fn default() -> Self {
        Self::Time(Duration::from_secs(1))
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
