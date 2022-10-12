#![feature(let_chains)]

mod moves;
mod mtdstarlite;

use async_trait::async_trait;
use azalea::{Client, Event};
use azalea_core::BlockPos;
use mtdstarlite::Edge;
pub use mtdstarlite::MTDStarLite;
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct Plugin {
    pub state: Arc<Mutex<State>>,
}

#[derive(Default)]
pub struct State {
    // pathfinder: Option<MTDStarLite<Node, f32>>,
}

#[async_trait]
impl azalea::Plugin for Plugin {
    async fn handle(self: Arc<Self>, bot: Client, event: Arc<Event>) {
        // match *
    }
}

pub trait Trait {
    fn goto(&self, goal: impl Goal);
}

impl Trait for azalea_client::Client {
    fn goto(&self, goal: impl Goal) {
        let start = Node {
            pos: BlockPos::from(self.entity().pos()),
        };
        let end = goal.goal_node();

        let successors = |node: &Node| {
            let mut edges = Vec::new();
            for possible_move in [
                moves::NorthMove {},
                moves::SouthMove {},
                moves::EastMove {},
                moves::WestMove {},
            ]
            .iter()
            {
                if possible_move.can_execute(&self.dimension(), &node.pos) {
                    edges.push(Edge {
                        node: Node {
                            pos: node.pos + possible_move.offset(),
                        },
                        cost: 1.0,
                    });
                }
            }
            edges
        };

        let mut pf = MTDStarLite::new(
            start,
            end,
            |n| goal.heuristic(n),
            successors,
            successors,
            |n| goal.success(n),
        );
        let p = pf.find_path();
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct Node {
    pub pos: BlockPos,
}

pub trait Goal {
    fn heuristic(&self, n: &Node) -> f32;
    fn success(&self, n: &Node) -> bool;
    // TODO: this should be removed and mtdstarlite should stop depending on
    // being given a goal node
    fn goal_node(&self) -> Node;
}
