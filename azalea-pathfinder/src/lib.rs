mod moves;
mod mtdstarlite;

use async_trait::async_trait;
use azalea::{prelude::*, WalkDirection};
use azalea::{Client, Event};
use azalea_core::BlockPos;
use azalea_world::entity::EntityData;
use mtdstarlite::Edge;
pub use mtdstarlite::MTDStarLite;
use parking_lot::Mutex;
use std::collections::VecDeque;
use std::sync::Arc;

#[derive(Default, Clone)]
pub struct Plugin {
    pub state: State,
}

#[derive(Default, Clone)]
pub struct State {
    // pathfinder: Option<MTDStarLite<Node, f32>>,
    pub path: Arc<Mutex<VecDeque<Node>>>,
}

#[async_trait]
impl azalea::Plugin for Plugin {
    async fn handle(self: Box<Self>, event: Event, mut bot: Client) {
        let mut path = self.state.path.lock();

        if !path.is_empty() {
            tick_execute_path(&mut bot, &mut path);
        }
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
            let possible_moves: Vec<&dyn moves::Move> = vec![
                &moves::NorthMove,
                &moves::SouthMove,
                &moves::EastMove,
                &moves::WestMove,
            ];
            let world = self.world.read();
            for possible_move in possible_moves.iter() {
                if possible_move.can_execute(&world, &node.pos) {
                    edges.push(Edge {
                        target: Node {
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

fn tick_execute_path(bot: &mut Client, path: &mut VecDeque<Node>) {
    let target = if let Some(target) = path.front() {
        target
    } else {
        return;
    };
    let center = target.pos.center();
    bot.look_at(&center);
    bot.walk(WalkDirection::Forward);

    if target.is_reached(&bot.entity()) {
        path.pop_front();
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

impl Node {
    /// Returns whether the entity is at the node and should start going to the
    /// next node.
    pub fn is_reached(&self, entity: &EntityData) -> bool {
        entity.yya == 0. && BlockPos::from(entity.pos()) == self.pos
    }
}
