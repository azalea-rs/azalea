use azalea_core::BlockPos;
use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::*;

use crate::Client;

/// A plugin that allows clients to break blocks in the world.
pub struct MinePlugin;
impl Plugin for MinePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StartMiningBlockEvent>()
            .add_systems(Update, handle_start_mining_block_event);
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

#[derive(Event)]
pub struct StartMiningBlockEvent {
    pub entity: Entity,
    pub position: BlockPos,
}

fn handle_start_mining_block_event(mut _events: EventReader<StartMiningBlockEvent>) {
    // for event in events.iter() {
    //     //
    // }
}
