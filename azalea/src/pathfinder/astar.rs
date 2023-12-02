use std::{
    cmp::Reverse,
    fmt::Debug,
    hash::Hash,
    time::{Duration, Instant},
};

use priority_queue::PriorityQueue;
use rustc_hash::FxHashMap;
use tracing::{debug, trace, warn};

pub struct Path<P, M>
where
    P: Eq + Hash + Copy + Debug,
{
    pub movements: Vec<Movement<P, M>>,
    pub partial: bool,
}

// used for better results when timing out
// see https://github.com/cabaletta/baritone/blob/1.19.4/src/main/java/baritone/pathing/calc/AbstractNodeCostSearch.java#L68
const COEFFICIENTS: [f32; 7] = [1.5, 2., 2.5, 3., 4., 5., 10.];

const MIN_IMPROVEMENT: f32 = 0.01;

pub fn a_star<P, M, HeuristicFn, SuccessorsFn, SuccessFn>(
    start: P,
    heuristic: HeuristicFn,
    mut successors: SuccessorsFn,
    success: SuccessFn,
    timeout: Duration,
) -> Path<P, M>
where
    P: Eq + Hash + Copy + Debug,
    HeuristicFn: Fn(P) -> f32,
    SuccessorsFn: FnMut(P) -> Vec<Edge<P, M>>,
    SuccessFn: Fn(P) -> bool,
{
    let start_time = Instant::now();

    let mut open_set = PriorityQueue::new();
    open_set.push(start, Reverse(Weight(0.)));
    let mut nodes: FxHashMap<P, Node<P, M>> = FxHashMap::default();
    nodes.insert(
        start,
        Node {
            position: start,
            movement_data: None,
            came_from: None,
            g_score: f32::default(),
            f_score: f32::MAX,
        },
    );

    let mut best_paths: [P; 7] = [start; 7];
    let mut best_path_scores: [f32; 7] = [heuristic(start); 7];

    let mut num_nodes = 0;

    while let Some((current_node, _)) = open_set.pop() {
        num_nodes += 1;
        if success(current_node) {
            debug!("Nodes considered: {num_nodes}");
            return Path {
                movements: reconstruct_path(nodes, current_node),
                partial: false,
            };
        }

        let current_g_score = nodes
            .get(&current_node)
            .map(|n| n.g_score)
            .unwrap_or(f32::MAX);

        for neighbor in successors(current_node) {
            let tentative_g_score = current_g_score + neighbor.cost;
            let neighbor_g_score = nodes
                .get(&neighbor.movement.target)
                .map(|n| n.g_score)
                .unwrap_or(f32::MAX);
            if tentative_g_score - neighbor_g_score < MIN_IMPROVEMENT {
                let heuristic = heuristic(neighbor.movement.target);
                let f_score = tentative_g_score + heuristic;
                nodes.insert(
                    neighbor.movement.target,
                    Node {
                        position: neighbor.movement.target,
                        movement_data: Some(neighbor.movement.data),
                        came_from: Some(current_node),
                        g_score: tentative_g_score,
                        f_score,
                    },
                );
                open_set.push(neighbor.movement.target, Reverse(Weight(f_score)));

                for (coefficient_i, &coefficient) in COEFFICIENTS.iter().enumerate() {
                    let node_score = heuristic + tentative_g_score / coefficient;
                    if best_path_scores[coefficient_i] - node_score > MIN_IMPROVEMENT {
                        best_paths[coefficient_i] = neighbor.movement.target;
                        best_path_scores[coefficient_i] = node_score;
                    }
                }
            }
        }

        // check for timeout every ~1ms
        if num_nodes % 1000 == 0 && start_time.elapsed() > timeout {
            // timeout, just return the best path we have so far
            trace!("A* couldn't find a path in time, returning best path");
            break;
        }
    }

    let best_path = determine_best_path(&best_paths, &start);

    Path {
        movements: reconstruct_path(nodes, best_path),
        partial: true,
    }
}

fn determine_best_path<P>(best_paths: &[P; 7], start: &P) -> P
where
    P: Eq + Hash + Copy + Debug,
{
    // this basically makes sure we don't create a path that's really short

    for node in best_paths.iter() {
        if node != start {
            return *node;
        }
    }
    warn!("No best node found, returning first node");
    best_paths[0]
}

fn reconstruct_path<P, M>(mut nodes: FxHashMap<P, Node<P, M>>, current: P) -> Vec<Movement<P, M>>
where
    P: Eq + Hash + Copy + Debug,
{
    let mut path = Vec::new();
    let mut current = current;
    while let Some(node) = nodes.remove(&current) {
        if let Some(came_from) = node.came_from {
            current = came_from;
        } else {
            break;
        }
        path.push(Movement {
            target: node.position,
            data: node.movement_data.unwrap(),
        });
    }
    path.reverse();
    path
}

pub struct Node<P, M> {
    pub position: P,
    pub movement_data: Option<M>,
    pub came_from: Option<P>,
    pub g_score: f32,
    pub f_score: f32,
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
pub struct Weight(f32);
impl Ord for Weight {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0
            .partial_cmp(&other.0)
            .unwrap_or(std::cmp::Ordering::Equal)
    }
}
impl Eq for Weight {}
impl PartialOrd for Weight {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
