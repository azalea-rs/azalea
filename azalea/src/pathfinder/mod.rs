mod moves;
mod mtdstarlite;

use crate::bot::{JumpEvent, LookAtEvent};
use crate::{SprintDirection, WalkDirection};

use azalea_client::{StartSprintEvent, StartWalkEvent};
use azalea_core::{BlockPos, CardinalDirection};
use azalea_ecs::app::{App, Plugin};
use azalea_ecs::AppTickExt;
use azalea_ecs::{
    component::Component,
    entity::Entity,
    event::EventReader,
    event::EventWriter,
    schedule::SystemSet,
    system::{Query, Res},
};
use azalea_world::{
    entity::{Physics, Position, WorldName},
    WorldContainer,
};
use log::{debug, error};
use mtdstarlite::Edge;
pub use mtdstarlite::MTDStarLite;
use std::collections::VecDeque;

#[derive(Clone, Default)]
pub struct PathfinderPlugin;
impl Plugin for PathfinderPlugin {
    fn build(&self, app: &mut App) {
        app.add_tick_system_set(SystemSet::new().with_system(tick_execute_path))
            .add_system(goto_listener);
    }
}

/// A component that makes this entity able to pathfind.
#[derive(Component)]
pub struct Pathfinder {
    pub path: VecDeque<Node>,
}

pub trait PathfinderClientExt {
    fn goto(&self, goal: impl Goal + Send + Sync + 'static);
}

impl PathfinderClientExt for azalea_client::Client {
    fn goto(&self, goal: impl Goal + Send + Sync + 'static) {
        self.ecs.lock().send_event(GotoEvent {
            entity: self.entity,
            goal: Box::new(goal),
        });
    }
}
pub struct GotoEvent {
    pub entity: Entity,
    pub goal: Box<dyn Goal + Send + Sync>,
}
fn goto_listener(
    mut events: EventReader<GotoEvent>,
    mut query: Query<(&Position, &WorldName, &mut Pathfinder)>,
    world_container: Res<WorldContainer>,
) {
    for event in events.iter() {
        let (position, world_name, mut pathfinder) = query
            .get_mut(event.entity)
            .expect("Called goto on an entity that can't pathfind");
        let start = Node {
            pos: BlockPos::from(position),
            vertical_vel: VerticalVel::None,
        };
        let end = event.goal.goal_node();
        debug!("start: {start:?}, end: {end:?}");

        let possible_moves: Vec<&dyn moves::Move> = vec![
            &moves::ForwardMove(CardinalDirection::North),
            &moves::ForwardMove(CardinalDirection::East),
            &moves::ForwardMove(CardinalDirection::South),
            &moves::ForwardMove(CardinalDirection::West),
            //
            &moves::AscendMove(CardinalDirection::North),
            &moves::AscendMove(CardinalDirection::East),
            &moves::AscendMove(CardinalDirection::South),
            &moves::AscendMove(CardinalDirection::West),
            //
            &moves::DescendMove(CardinalDirection::North),
            &moves::DescendMove(CardinalDirection::East),
            &moves::DescendMove(CardinalDirection::South),
            &moves::DescendMove(CardinalDirection::West),
            //
            &moves::DiagonalMove(CardinalDirection::North),
            &moves::DiagonalMove(CardinalDirection::East),
            &moves::DiagonalMove(CardinalDirection::South),
            &moves::DiagonalMove(CardinalDirection::West),
        ];

        let successors = |node: &Node| {
            let mut edges = Vec::new();

            let world_lock = world_container
                .get(world_name)
                .expect("Entity tried to pathfind but the entity isn't in a valid world");
            let world = world_lock.read();
            for possible_move in &possible_moves {
                edges.push(Edge {
                    target: possible_move.next_node(node),
                    cost: possible_move.cost(&world, node),
                });
            }
            edges
        };

        let mut pf = MTDStarLite::new(
            start,
            end,
            |n| event.goal.heuristic(n),
            successors,
            successors,
            |n| event.goal.success(n),
        );

        let start = std::time::Instant::now();
        let p = pf.find_path();
        let end = std::time::Instant::now();
        debug!("path: {p:?}");
        debug!("time: {:?}", end - start);

        // convert the Option<Vec<Node>> to a VecDeque<Node>
        if let Some(p) = p {
            pathfinder.path = p.into_iter().collect();
        } else {
            error!("no path found");
        }
    }
}

fn tick_execute_path(
    mut query: Query<(Entity, &mut Pathfinder, &Position, &Physics)>,
    mut look_at_events: EventWriter<LookAtEvent>,
    mut sprint_events: EventWriter<StartSprintEvent>,
    mut walk_events: EventWriter<StartWalkEvent>,
    mut jump_events: EventWriter<JumpEvent>,
) {
    for (entity, mut pathfinder, position, physics) in &mut query {
        loop {
            let Some(target) = pathfinder.path.front() else {
                return;
            };
            let center = target.pos.center();
            // println!("going to {center:?} (at {pos:?})", pos = bot.entity().pos());
            look_at_events.send(LookAtEvent {
                entity,
                position: center,
            });
            sprint_events.send(StartSprintEvent {
                entity,
                direction: SprintDirection::Forward,
            });
            // check if we should jump
            if target.pos.y > position.y.floor() as i32 {
                jump_events.send(JumpEvent(entity));
            }

            if target.is_reached(position, physics) {
                // println!("ok target {target:?} reached");
                pathfinder.path.pop_front();
                if pathfinder.path.is_empty() {
                    walk_events.send(StartWalkEvent {
                        entity,
                        direction: WalkDirection::None,
                    });
                }
                // tick again, maybe we already reached the next node!
            } else {
                break;
            }
        }
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
    #[must_use]
    pub fn is_reached(&self, position: &Position, physics: &Physics) -> bool {
        // println!(
        //     "entity.delta.y: {} {:?}=={:?}, self.vertical_vel={:?}",
        //     entity.delta.y,
        //     BlockPos::from(entity.pos()),
        //     self.pos,
        //     self.vertical_vel
        // );
        BlockPos::from(position) == self.pos
            && match self.vertical_vel {
                VerticalVel::NoneMidair => physics.delta.y > -0.1 && physics.delta.y < 0.1,
                VerticalVel::None => physics.on_ground,
                VerticalVel::FallingLittle => physics.delta.y < -0.1,
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
