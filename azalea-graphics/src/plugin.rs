use std::{num::NonZero, sync::Arc};

use azalea::{
    app::{App, AppExit, Plugin}, block_update::handle_block_update_event, chunks::{handle_receive_chunk_event, ReceiveChunkEvent}, core::position::ChunkPos, ecs::{
        event::{EventReader, EventWriter},
        schedule::IntoScheduleConfigs,
        system::{Query, Res},
    }, local_player::InstanceHolder, prelude::*, world::Chunk
};
use crossbeam::channel::TryRecvError;
use log::error;
use parking_lot::RwLock;

use crate::renderer::{RendererCommand, RendererEvent, RendererHandle, mesher::LocalChunk};

#[derive(Resource, Clone)]
pub struct RendererResource {
    pub handle: RendererHandle,
}

pub struct RendererPlugin {
    pub handle: RendererHandle,
}

impl Plugin for RendererPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RendererResource {
            handle: self.handle.clone(),
        });
        app.add_systems(
            GameTick,
            forward_chunk_updates.after(handle_receive_chunk_event).after(handle_block_update_event),
        );
        app.add_systems(GameTick, poll_renderer_events);
    }
}

fn forward_chunk_updates(
    mut events: EventReader<ReceiveChunkEvent>,
    renderer: Res<RendererResource>,
    mut query: Query<&InstanceHolder>,
) {
    for event in events.read() {
        let pos = ChunkPos::new(event.packet.x, event.packet.z);

        let local_player = query.get_mut(event.entity).unwrap();
        let instance = local_player.instance.read();
        let partial_instance = local_player.partial_instance.read();

        let lookup_chunk = |pos: ChunkPos| -> Option<Arc<RwLock<Chunk>>> {
            partial_instance
                .chunks
                .limited_get(&pos)
                .cloned()
                .or_else(|| instance.chunks.get(&pos))
        };

        if let Some(center) = lookup_chunk(pos) {
            let neighbors = [
                lookup_chunk(ChunkPos::new(pos.x, pos.z - 1)), // North
                lookup_chunk(ChunkPos::new(pos.x, pos.z + 1)), // South
                lookup_chunk(ChunkPos::new(pos.x + 1, pos.z)), // East
                lookup_chunk(ChunkPos::new(pos.x - 1, pos.z)), // West
                lookup_chunk(ChunkPos::new(pos.x + 1, pos.z - 1)), // NE
                lookup_chunk(ChunkPos::new(pos.x - 1, pos.z - 1)), // NW
                lookup_chunk(ChunkPos::new(pos.x + 1, pos.z + 1)), // SE
                lookup_chunk(ChunkPos::new(pos.x - 1, pos.z + 1)), // SW
            ];

            let local_chunk = LocalChunk { center, neighbors };

            renderer
                .handle
                .tx
                .send(RendererCommand::ChunkUpdate(pos, local_chunk))
                .unwrap();
        } else {
            error!("no chunk at {:?}", pos);
        }
    }
}

fn poll_renderer_events(renderer: Res<RendererResource>, mut writer: EventWriter<AppExit>) {
    match renderer.handle.rx.try_recv() {
        Ok(RendererEvent::Closed) => {
            writer.write(AppExit::Success);
        }
        Err(TryRecvError::Empty) => {}
        Err(TryRecvError::Disconnected) => {
            writer.write(AppExit::Error(NonZero::new(1).unwrap()));
        }
    }
}
