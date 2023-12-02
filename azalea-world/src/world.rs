use crate::{iterators::ChunkIterator, palette::Palette, ChunkStorage, PartialChunkStorage};
use azalea_block::{BlockState, BlockStates, FluidState};
use azalea_core::position::{BlockPos, ChunkPos};
use azalea_core::registry_holder::RegistryHolder;
use bevy_ecs::{component::Component, entity::Entity};
use derive_more::{Deref, DerefMut};
use nohash_hasher::IntMap;
use std::fmt::Formatter;
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

/// PartialInstances are usually owned by clients, and hold strong references to
/// chunks and entities in [`Instance`]s.
///
/// Basically, they hold the chunks and entities that are within render
/// distance but can still access chunks and entities owned by other
/// `PartialInstance`s that have the same `Instance`.
///
/// This is primarily useful for having multiple clients in the same Instance.
pub struct PartialInstance {
    pub chunks: PartialChunkStorage,
    /// Some metadata about entities, like what entities are in certain chunks.
    /// This does not contain the entity data itself, that's in the ECS.
    pub entity_infos: PartialEntityInfos,
}

impl PartialInstance {
    pub fn new(chunk_radius: u32, owner_entity: Option<Entity>) -> Self {
        PartialInstance {
            chunks: PartialChunkStorage::new(chunk_radius),
            entity_infos: PartialEntityInfos::new(owner_entity),
        }
    }
}

/// An entity ID used by Minecraft. These are not guaranteed to be unique in
/// shared worlds, that's what [`Entity`] is for.
///
/// [`Entity`]: bevy_ecs::entity::Entity
#[derive(Component, Copy, Clone, Debug, PartialEq, Eq, Deref, DerefMut)]
pub struct MinecraftEntityId(pub u32);

impl std::hash::Hash for MinecraftEntityId {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        hasher.write_u32(self.0);
    }
}
impl nohash_hasher::IsEnabled for MinecraftEntityId {}

/// Keep track of certain metadatas that are only relevant for this partial
/// world.
#[derive(Debug, Default)]
pub struct PartialEntityInfos {
    // note: using MinecraftEntityId for entity ids is acceptable here since
    // there's no chance of collisions here
    /// The entity id of the player that owns this partial world. This will
    /// make `RelativeEntityUpdate` pretend this entity doesn't exist so
    /// it doesn't get modified from outside sources.
    pub owner_entity: Option<Entity>,
    /// A counter for each entity that tracks how many updates we've observed
    /// for it.
    ///
    /// This is used for shared worlds (i.e. swarms), to make sure we don't
    /// update entities twice on accident.
    pub updates_received: IntMap<MinecraftEntityId, u32>,
}

impl PartialEntityInfos {
    pub fn new(owner_entity: Option<Entity>) -> Self {
        Self {
            owner_entity,
            updates_received: IntMap::default(),
        }
    }
}

/// A world where the chunks are stored as weak pointers. This is used for
/// shared worlds.
#[derive(Default, Debug)]
pub struct Instance {
    pub chunks: ChunkStorage,

    /// An index of all the entities we know are in the chunks of the world
    pub entities_by_chunk: HashMap<ChunkPos, HashSet<Entity>>,

    /// An index of Minecraft entity IDs to Azalea ECS entities. You should
    /// avoid using this and instead use `azalea_entity::EntityIdIndex`
    pub entity_by_id: IntMap<MinecraftEntityId, Entity>,

    pub registries: RegistryHolder,
}

impl Instance {
    pub fn get_block_state(&self, pos: &BlockPos) -> Option<BlockState> {
        self.chunks.get_block_state(pos)
    }

    pub fn get_fluid_state(&self, pos: &BlockPos) -> Option<FluidState> {
        self.chunks.get_block_state(pos).map(FluidState::from)
    }

    pub fn set_block_state(&self, pos: &BlockPos, state: BlockState) -> Option<BlockState> {
        self.chunks.set_block_state(pos, state)
    }

    /// Find the coordinates of a block in the world.
    ///
    /// Note that this is sorted by `x+y+z` and not `x^2+y^2+z^2` for
    /// optimization purposes.
    ///
    /// ```
    /// # fn example(client: &azalea_client::Client) {
    /// client.world().read().find_block(client.position(), &azalea_registry::Block::Chest.into());
    /// # }
    /// ```
    pub fn find_block(
        &self,
        nearest_to: impl Into<BlockPos>,
        block_states: &BlockStates,
    ) -> Option<BlockPos> {
        // iterate over every chunk in a 3d spiral pattern
        // and then check the palette for the block state

        let nearest_to: BlockPos = nearest_to.into();
        let start_chunk: ChunkPos = (&nearest_to).into();
        let mut iter = ChunkIterator::new(start_chunk, 32);

        let mut nearest_found_pos: Option<BlockPos> = None;
        let mut nearest_found_distance = 0;

        // we do `while` instead of `for` so we can access iter later
        while let Some(chunk_pos) = iter.next() {
            let Some(chunk) = self.chunks.get(&chunk_pos) else {
                // if the chunk isn't loaded then we skip it.
                // we don't just return since it *could* cause issues if there's a random
                // unloaded chunk and then more that are loaded.
                // unlikely but still something to consider, and it's not like this slows it
                // down much anyways.
                continue;
            };

            for (section_index, section) in chunk.read().sections.iter().enumerate() {
                let maybe_has_block = match &section.states.palette {
                    Palette::SingleValue(id) => block_states.contains(&BlockState { id: *id }),
                    Palette::Linear(ids) => ids
                        .iter()
                        .any(|&id| block_states.contains(&BlockState { id })),
                    Palette::Hashmap(ids) => ids
                        .iter()
                        .any(|&id| block_states.contains(&BlockState { id })),
                    Palette::Global => true,
                };
                if !maybe_has_block {
                    continue;
                }

                for i in 0..4096 {
                    let block_state = section.states.get_at_index(i);
                    let block_state = BlockState { id: block_state };

                    if block_states.contains(&block_state) {
                        let (section_x, section_y, section_z) = section.states.coords_from_index(i);
                        let (x, y, z) = (
                            chunk_pos.x * 16 + (section_x as i32),
                            self.chunks.min_y + (section_index * 16) as i32 + section_y as i32,
                            chunk_pos.z * 16 + (section_z as i32),
                        );
                        let this_block_pos = BlockPos { x, y, z };
                        let this_block_distance = (nearest_to - this_block_pos).length_manhattan();
                        // only update if it's closer
                        if nearest_found_pos.is_none()
                            || this_block_distance < nearest_found_distance
                        {
                            nearest_found_pos = Some(this_block_pos);
                            nearest_found_distance = this_block_distance;
                        }
                    }
                }
            }

            if let Some(nearest_found_pos) = nearest_found_pos {
                // this is required because find_block searches chunk-by-chunk, which can cause
                // us to find blocks first that aren't actually the closest
                let required_chunk_distance = u32::max(
                    u32::max(
                        (chunk_pos.x - start_chunk.x).unsigned_abs(),
                        (chunk_pos.z - start_chunk.z).unsigned_abs(),
                    ),
                    (nearest_to.y - nearest_found_pos.y)
                        .unsigned_abs()
                        .div_ceil(16),
                );
                let nearest_chunk_distance = iter.layer;

                // if we found the position and there's no chance there's something closer,
                // return it
                if nearest_chunk_distance >= required_chunk_distance {
                    return Some(nearest_found_pos);
                }
            }
        }

        if nearest_found_pos.is_some() {
            nearest_found_pos
        } else {
            None
        }
    }
}

impl Debug for PartialInstance {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PartialInstance")
            .field("chunks", &self.chunks)
            .field("entity_infos", &self.entity_infos)
            .finish()
    }
}

impl Default for PartialInstance {
    /// Creates a completely self-contained `PartialInstance`. This is only for
    /// testing and shouldn't be used in actual code!
    fn default() -> Self {
        let chunk_storage = PartialChunkStorage::default();
        let entity_storage = PartialEntityInfos::default();
        Self {
            chunks: chunk_storage,
            entity_infos: entity_storage,
        }
    }
}

impl From<ChunkStorage> for Instance {
    /// Make an empty world from this `ChunkStorage`. This is meant to be a
    /// convenience function for tests.
    fn from(chunks: ChunkStorage) -> Self {
        Self {
            chunks,
            entities_by_chunk: HashMap::new(),
            entity_by_id: IntMap::default(),
            registries: RegistryHolder::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use azalea_registry::Block;

    use crate::Chunk;

    use super::*;

    #[test]
    fn find_block() {
        let mut instance = Instance::default();

        let chunk_storage = &mut instance.chunks;
        let mut partial_chunk_storage = PartialChunkStorage::default();

        // block at (17, 0, 0) and (0, 18, 0)

        partial_chunk_storage.set(
            &ChunkPos { x: 0, z: 0 },
            Some(Chunk::default()),
            chunk_storage,
        );
        partial_chunk_storage.set(
            &ChunkPos { x: 1, z: 0 },
            Some(Chunk::default()),
            chunk_storage,
        );

        chunk_storage.set_block_state(&BlockPos { x: 17, y: 0, z: 0 }, Block::Stone.into());
        chunk_storage.set_block_state(&BlockPos { x: 0, y: 18, z: 0 }, Block::Stone.into());

        let pos = instance.find_block(BlockPos { x: 0, y: 0, z: 0 }, &Block::Stone.into());
        assert_eq!(pos, Some(BlockPos { x: 17, y: 0, z: 0 }));
    }
}
