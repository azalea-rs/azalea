use std::{cmp::Reverse, collections::HashMap, fmt::Debug, hash::Hash, ops::Add};

use priority_queue::PriorityQueue;

pub fn a_star<N, W, HeuristicFn, SuccessorsFn, SuccessFn>(
    start: N,
    heuristic: HeuristicFn,
    successors: SuccessorsFn,
    success: SuccessFn,
) -> Option<Vec<N>>
where
    N: Eq + Hash + Copy + Debug,
    W: PartialOrd + Default + Copy + num_traits::Bounded + Debug + Add<Output = W>,
    HeuristicFn: Fn(&N) -> W,
    SuccessorsFn: Fn(&N) -> Vec<Edge<N, W>>,
    SuccessFn: Fn(&N) -> bool,
{
    let mut open_set = PriorityQueue::new();
    open_set.push(start, Reverse(Weight(W::default())));
    let mut nodes: HashMap<N, Node<N, W>> = HashMap::new();
    nodes.insert(
        start,
        Node {
            data: start,
            came_from: None,
            g_score: W::default(),
            f_score: W::max_value(),
        },
    );

    while let Some((current_node, _)) = open_set.pop() {
        if success(&current_node) {
            return Some(reconstruct_path(&nodes, current_node));
        }

        let current_g_score = nodes
            .get(&current_node)
            .map(|n| n.g_score)
            .unwrap_or(W::max_value());

        for neighbor in successors(&current_node) {
            let tentative_g_score = current_g_score + neighbor.cost;
            let neighbor_g_score = nodes
                .get(&neighbor.target)
                .map(|n| n.g_score)
                .unwrap_or(W::max_value());
            if tentative_g_score < neighbor_g_score {
                let f_score = tentative_g_score + heuristic(&neighbor.target);
                nodes.insert(
                    neighbor.target,
                    Node {
                        data: neighbor.target,
                        came_from: Some(current_node),
                        g_score: tentative_g_score,
                        f_score,
                    },
                );
                open_set.push(neighbor.target, Reverse(Weight(f_score)));
            }
        }
    }

    None
}

fn reconstruct_path<N, W>(nodes: &HashMap<N, Node<N, W>>, current: N) -> Vec<N>
where
    N: Eq + Hash + Copy + Debug,
    W: PartialOrd + Default + Copy + num_traits::Bounded + Debug,
{
    let mut path = vec![current];
    let mut current = current;
    while let Some(node) = nodes.get(&current) {
        if let Some(came_from) = node.came_from {
            path.push(came_from);
            current = came_from;
        } else {
            break;
        }
    }
    path.reverse();
    path
}

pub struct Node<N, W> {
    pub data: N,
    pub came_from: Option<N>,
    pub g_score: W,
    pub f_score: W,
}

pub struct Edge<N: Eq + Hash + Copy, W: PartialOrd + Copy> {
    pub target: N,
    pub cost: W,
}

#[derive(PartialEq)]
pub struct Weight<W: PartialOrd + Debug>(W);
impl<W: PartialOrd + Debug> Ord for Weight<W> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0
            .partial_cmp(&other.0)
            .unwrap_or(std::cmp::Ordering::Equal)
    }
}
impl<W: PartialOrd + Debug> Eq for Weight<W> {}
impl<W: PartialOrd + Debug> PartialOrd for Weight<W> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
