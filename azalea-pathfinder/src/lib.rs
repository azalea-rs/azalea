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
            vertical_vel: VerticalVel::None,
        };
        let end = goal.goal_node();
        println!("start: {:?}, end: {:?}", start, end);

        let successors = |node: &Node| {
            let mut edges = Vec::new();
            let possible_moves: Vec<&dyn moves::Move> = vec![
                &moves::NorthMove,
                &moves::SouthMove,
                &moves::EastMove,
                &moves::WestMove,
                &moves::JumpUpMove,
                &moves::FallNorthMove,
                &moves::FallSouthMove,
                &moves::FallEastMove,
                &moves::FallWestMove,
                &moves::LandMove,
            ];
            let dimension = self.dimension.read();
            for possible_move in possible_moves.iter() {
                edges.push(Edge {
                    target: possible_move.next_node(&node),
                    cost: if possible_move.can_execute(&dimension, node) {
                        possible_move.cost()
                    } else {
                        f32::INFINITY
                    },
                });
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
        println!("path: {:?}", p);

        let state = self.plugins.get::<Plugin>().unwrap().state.clone();
        // convert the Option<Vec<Node>> to a VecDeque<Node>
        *state.path.lock() = p.expect("no path").into_iter().collect();
    }
}

fn tick_execute_path(bot: &mut Client, path: &mut VecDeque<Node>) {
    let target = if let Some(target) = path.front() {
        target
    } else {
        return;
    };
    let center = target.pos.center();
    // println!("going to {center:?} (at {pos:?})", pos = bot.entity().pos());
    bot.look_at(&center);
    bot.walk(WalkDirection::Forward);
    bot.set_sprinting(true);
    // check if we should jump
    if target.pos.y > bot.entity().pos().y.floor() as i32 {
        bot.jump();
    }

    if target.is_reached(&bot.entity()) {
        println!("ok target {target:?} reached");
        path.pop_front();
        if path.is_empty() {
            bot.walk(WalkDirection::None);
            bot.set_sprinting(false);
        }
        // tick again, maybe we already reached the next node!
        tick_execute_path(bot, path);
    }
}

/// Information about our vertical velocity
#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum VerticalVel {
    None,
    /// No vertical velocity, but we're not on the ground
    NoneMidair,
    // less than 3 blocks (no fall damage)
    FallingLittle,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct Node {
    pub pos: BlockPos,
    pub vertical_vel: VerticalVel,
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
        println!(
            "entity.delta.y: {} {:?}=={:?}, self.vertical_vel={:?}",
            entity.delta.y,
            BlockPos::from(entity.pos()),
            self.pos,
            self.vertical_vel
        );
        BlockPos::from(entity.pos()) == self.pos
            && match self.vertical_vel {
                VerticalVel::NoneMidair => entity.delta.y > -0.1 && entity.delta.y < 0.1,
                VerticalVel::None => entity.on_ground,
                VerticalVel::FallingLittle => entity.delta.y < -0.1,
            }
    }
}

pub struct BlockPosGoal {
    pub pos: BlockPos,
}
impl Goal for BlockPosGoal {
    fn heuristic(&self, n: &Node) -> f32 {
        let dx = (self.pos.x - n.pos.x) as f32;
        let dy = (self.pos.y - n.pos.y) as f32;
        let dz = (self.pos.z - n.pos.z) as f32;
        dx * dx + dy * dy + dz * dz
    }
    fn success(&self, n: &Node) -> bool {
        n.pos == self.pos
    }
    fn goal_node(&self) -> Node {
        Node {
            pos: self.pos,
            vertical_vel: VerticalVel::None,
        }
    }
}

impl From<BlockPos> for BlockPosGoal {
    fn from(pos: BlockPos) -> Self {
        Self { pos }
    }
}
