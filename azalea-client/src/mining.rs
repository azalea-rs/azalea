use azalea_core::BlockPos;
use bevy_app::{App, Plugin};
use bevy_ecs::prelude::*;

use crate::Client;

/// A plugin that allows clients to break blocks in the world.
pub struct MinePlugin;
impl Plugin for MinePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StartMiningBlockEvent>()
            .add_system(handle_start_mining_block_event);
    }
}

impl Client {
    /// Start mining a block.
    pub fn start_mining_block(&self, position: BlockPos) {
        self.ecs.lock().send_event(StartMiningBlockEvent {
            entity: self.entity,
            position,
        });
    }
}

pub struct StartMiningBlockEvent {
    pub entity: Entity,
    pub position: BlockPos,
}

fn handle_start_mining_block_event(mut events: EventReader<StartMiningBlockEvent>) {
    for event in events.iter() {
        //
    }
}
