pub mod mine_area;

use azalea_client::{mining::Mining, InstanceHolder};
use azalea_entity::Position;

use crate::{
    auto_tool::StartMiningBlockWithAutoToolEvent,
    ecs::prelude::*,
    pathfinder::{self, ExecutingPath, GotoEvent, Pathfinder},
    LookAtEvent,
};

#[derive(Component, Clone, Debug)]
pub enum Process {
    MineArea(mine_area::MineArea),
}

impl From<mine_area::MineArea> for Process {
    fn from(mine_area: mine_area::MineArea) -> Self {
        Self::MineArea(mine_area)
    }
}

#[derive(Event)]
pub struct SetActiveProcessEvent {
    pub entity: Entity,
    pub process: Process,
}

pub fn set_active_pathfinder_process_listener(
    mut commands: Commands,
    mut events: EventReader<SetActiveProcessEvent>,
    mut stop_pathfinding_events: EventWriter<pathfinder::StopPathfindingEvent>,
) {
    for event in events.read() {
        stop_pathfinding_events.send(pathfinder::StopPathfindingEvent {
            entity: event.entity,
            force: false,
        });
        commands.entity(event.entity).insert(event.process.clone());
    }
}

pub struct ProcessSystemComponents<'a> {
    pub entity: Entity,
    pub position: &'a Position,
    pub instance_holder: &'a InstanceHolder,
    pub pathfinder: &'a Pathfinder,
    pub mining: Option<&'a Mining>,
    pub executing_path: Option<&'a ExecutingPath>,
}

#[allow(clippy::type_complexity)]
pub fn process_tick(
    mut commands: Commands,
    query: Query<(
        Entity,
        &Process,
        &Position,
        &InstanceHolder,
        &Pathfinder,
        Option<&Mining>,
        Option<&ExecutingPath>,
    )>,
    mut goto_events: EventWriter<GotoEvent>,
    mut look_at_events: EventWriter<LookAtEvent>,
    mut start_mining_block_events: EventWriter<StartMiningBlockWithAutoToolEvent>,
) {
    for (entity, process, position, instance_holder, pathfinder, mining, executing_path) in &query {
        let components = ProcessSystemComponents {
            entity,
            position,
            instance_holder,
            pathfinder,
            mining,
            executing_path,
        };
        match process {
            Process::MineArea(mine_area) => {
                mine_area::mine_area(
                    mine_area,
                    &mut commands,
                    components,
                    &mut goto_events,
                    &mut look_at_events,
                    &mut start_mining_block_events,
                );
            }
        }
    }
}
