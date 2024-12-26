use std::{
    cmp::{self},
    collections::BinaryHeap,
    fmt::Debug,
    hash::{BuildHasherDefault, Hash},
    time::{Duration, Instant},
};

use indexmap::IndexMap;
use num_format::ToFormattedString;
use rustc_hash::FxHasher;
use tracing::{debug, trace, warn};

pub struct Path<P, M>
where
    P: Eq + Hash + Copy + Debug,
{
    pub movements: Vec<Movement<P, M>>,
    pub is_partial: bool,
}

// used for better results when timing out
// see https://github.com/cabaletta/baritone/blob/1.19.4/src/main/java/baritone/pathing/calc/AbstractNodeCostSearch.java#L68
const COEFFICIENTS: [f32; 7] = [1.5, 2., 2.5, 3., 4., 5., 10.];

const MIN_IMPROVEMENT: f32 = 0.01;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PathfinderTimeout {
    /// Time out after a certain duration has passed. This is a good default so
    /// you don't waste too much time calculating a path if you're on a slow
    /// computer.
    Time(Duration),
    /// Time out after this many nodes have been considered.
    ///
    /// This is useful as an alternative to a time limit if you're doing
    /// something like running tests where you want consistent results.
    Nodes(usize),
}

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

    let mut open_set = BinaryHeap::<WeightedNode>::new();
    open_set.push(WeightedNode {
        g_score: 0.,
        f_score: 0.,
        index: 0,
    });
    let mut nodes: FxIndexMap<P, Node<M>> = IndexMap::default();
    nodes.insert(
        start,
        Node {
            movement_data: None,
            came_from: usize::MAX,
            g_score: 0.,
        },
    );

    let mut best_paths: [usize; 7] = [0; 7];
    let mut best_path_scores: [f32; 7] = [heuristic(start); 7];

    let mut num_nodes = 0;

    while let Some(WeightedNode { index, g_score, .. }) = open_set.pop() {
        num_nodes += 1;

        let (&node, node_data) = nodes.get_index(index).unwrap();
        if success(node) {
            debug!("Nodes considered: {num_nodes}");

            return Path {
                movements: reconstruct_path(nodes, index),
                is_partial: false,
            };
        }

        if g_score > node_data.g_score {
            continue;
        }

        for neighbor in successors(node) {
            let tentative_g_score = g_score + neighbor.cost;
            // let neighbor_heuristic = heuristic(neighbor.movement.target);
            let neighbor_heuristic;
            let neighbor_index;

            // skip neighbors that don't result in a big enough improvement
            if tentative_g_score - g_score < MIN_IMPROVEMENT {
                continue;
            }

            match nodes.entry(neighbor.movement.target) {
                indexmap::map::Entry::Occupied(mut e) => {
                    if e.get().g_score > tentative_g_score {
                        neighbor_heuristic = heuristic(*e.key());
                        neighbor_index = e.index();
                        e.insert(Node {
                            movement_data: Some(neighbor.movement.data),
                            came_from: index,
                            g_score: tentative_g_score,
                        });
                    } else {
                        continue;
                    }
                }
                indexmap::map::Entry::Vacant(e) => {
                    neighbor_heuristic = heuristic(*e.key());
                    neighbor_index = e.index();
                    e.insert(Node {
                        movement_data: Some(neighbor.movement.data),
                        came_from: index,
                        g_score: tentative_g_score,
                    });
                }
            }

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
        if num_nodes % 10000 == 0 {
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

    let best_path = determine_best_path(best_paths, 0);

    debug!(
        "A* ran at {} nodes per second",
        ((num_nodes as f64 / start_time.elapsed().as_secs_f64()) as u64)
            .to_formatted_string(&num_format::Locale::en)
    );

    Path {
        movements: reconstruct_path(nodes, best_path),
        is_partial: true,
    }
}

fn determine_best_path(best_paths: [usize; 7], start: usize) -> usize {
    // this basically makes sure we don't create a path that's really short

    for node in best_paths {
        if node != start {
            return node;
        }
    }
    warn!("No best node found, returning first node");
    best_paths[0]
}

fn reconstruct_path<P, M>(
    mut nodes: FxIndexMap<P, Node<M>>,
    mut current_index: usize,
) -> Vec<Movement<P, M>>
where
    P: Eq + Hash + Copy + Debug,
{
    let mut path = Vec::new();
    while let Some((&node_position, node)) = nodes.get_index_mut(current_index) {
        if node.came_from == usize::MAX {
            break;
        }

        current_index = node.came_from;

        path.push(Movement {
            target: node_position,
            data: node.movement_data.take().unwrap(),
        });
    }
    path.reverse();
    path
}

pub struct Node<M> {
    pub movement_data: Option<M>,
    pub came_from: usize,
    pub g_score: f32,
}

pub struct Edge<P: Hash + Copy, M> {
    pub movement: Movement<P, M>,
    pub cost: f32,
}

pub struct Movement<P: Hash + Copy, M> {
    pub target: P,
    pub data: M,
}

impl<P: Hash + Copy + Debug, M: Debug> Debug for Movement<P, M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

#[derive(PartialEq)]
pub struct WeightedNode {
    index: usize,
    g_score: f32,
    f_score: f32,
}

impl Ord for WeightedNode {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        // intentionally inverted to make the BinaryHeap a min-heap
        match other
            .f_score
            .partial_cmp(&self.f_score)
            .unwrap_or(cmp::Ordering::Equal)
        {
            cmp::Ordering::Equal => self
                .g_score
                .partial_cmp(&other.g_score)
                .unwrap_or(cmp::Ordering::Equal),
            s => s,
        }
    }
}
impl Eq for WeightedNode {}
impl PartialOrd for WeightedNode {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}
