use azalea_block::BlockState;
use azalea_core::position::BlockPos;
use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::*;

use crate::{
    chunks::handle_receive_chunk_event, interact::BlockStatePredictionHandler,
    local_player::InstanceHolder,
};

pub struct BlockUpdatePlugin;
impl Plugin for BlockUpdatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            // has to be after ReceiveChunkEvent is handled so if we get chunk+blockupdate in one
            // Update then the block update actually gets applied
            handle_block_update_event.after(handle_receive_chunk_event),
        );
    }
}

/// A component that holds the list of block updates that need to be handled.
///
/// This is updated by `read_packets` (in `PreUpdate`) and handled/cleared by
/// [`handle_block_update_event`] (`Update`).
///
/// This is a component instead of an ECS event for performance reasons.
#[derive(Clone, Component, Debug, Default)]
pub struct QueuedServerBlockUpdates {
    pub list: Vec<(BlockPos, BlockState)>,
}

pub fn handle_block_update_event(
    mut query: Query<(
        &mut QueuedServerBlockUpdates,
        &InstanceHolder,
        &mut BlockStatePredictionHandler,
    )>,
) {
    for (mut queued, instance_holder, mut prediction_handler) in query.iter_mut() {
        let world = instance_holder.instance.read();
        for (pos, block_state) in queued.list.drain(..) {
            if !prediction_handler.update_known_server_state(pos, block_state) {
                world.chunks.set_block_state(pos, block_state);
            }
        }
    }
}
