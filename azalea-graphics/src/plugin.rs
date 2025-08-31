use std::{io::Cursor, num::NonZero, sync::Arc};

use azalea::{
    app::{App, AppExit, Plugin, Update}, block_update::handle_block_update_event, chunks::{handle_receive_chunk_event, ReceiveChunkEvent}, core::{
        position::ChunkPos,
        registry_holder::{BiomeData, RegistryHolder},
    }, ecs::{
        component::Component, entity::Entity, event::{EventReader, EventWriter}, query::Added, schedule::IntoScheduleConfigs, system::{Commands, Query, Res}
    }, local_player::InstanceHolder, prelude::*, registry::{Biome, DataRegistry}, world::Chunk
};
use crossbeam::channel::TryRecvError;
use log::error;
use parking_lot::RwLock;
use simdnbt::Deserialize;

use crate::renderer::{RendererEvent, RendererHandle, mesher::LocalChunk};

/// A cache of parsed biome data indexed by biome registry index
#[derive(Component, Clone, Debug)]
pub struct BiomeCache {
    pub biomes: Vec<BiomeData>,
}

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
            Update,
            forward_chunk_updates
                .after(handle_receive_chunk_event)
                .after(handle_block_update_event),
        );
        app.add_systems(Update, (create_biome_cache, poll_renderer_events));
    }
}

fn forward_chunk_updates(
    mut events: EventReader<ReceiveChunkEvent>,
    renderer: Res<RendererResource>,
    mut query: Query<(&InstanceHolder, &BiomeCache)>,
) {
    for event in events.read() {
        let pos = ChunkPos::new(event.packet.x, event.packet.z);

        let Ok((local_player, biome_cache)) = query.get_mut(event.entity) else {
            error!("Entity {:?} missing InstanceHolder or BiomeCache", event.entity);
            continue;
        };
        let instance = local_player.instance.read();

        let lookup_chunk =
            |pos: ChunkPos| -> Option<Arc<RwLock<Chunk>>> { instance.chunks.get(&pos) };

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
            renderer.handle.send_chunk(
                pos,
                local_chunk,
                biome_cache.clone(),
            )
        } else {
            error!("no chunk at {:?}", pos);
        }
    }
}

fn create_biome_cache(
    mut commands: Commands,
    query: Query<(Entity, &InstanceHolder), Added<InstanceHolder>>,
) {
    for (entity, instance_holder) in query.iter() {
        let instance = instance_holder.instance.read();
        let registries = &instance.registries;

        let biome_cache = create_biome_cache_from_registries(registries);

        commands.entity(entity).insert(biome_cache);
    }
}

fn create_biome_cache_from_registries(registries: &RegistryHolder) -> BiomeCache {
    let mut biomes = Vec::new();

    if let Some(biome_registry) = registries.map.get(&azalea::ResourceLocation::new(Biome::NAME)) {
        for (_key, value) in biome_registry {
            let mut nbt_bytes = Vec::new();
            value.write(&mut nbt_bytes);
            
            let nbt_borrow_compound = match simdnbt::borrow::read_compound(&mut Cursor::new(&nbt_bytes)) {
                Ok(compound) => compound,
                Err(e) => {
                    error!("Failed to read NBT compound for biome: {}", e);
                    continue;
                }
            };
            
            let biome_data = match BiomeData::from_compound((&nbt_borrow_compound).into()) {
                Ok(value) => value,
                Err(e) => {
                    error!("Failed to parse BiomeData: {}", e);
                    continue;
                }
            };

            biomes.push(biome_data);
        }
    }

    BiomeCache { biomes }
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
