use std::fmt::Formatter;
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use azalea_block::{BlockState, FluidState};
use azalea_core::position::{BlockPos, ChunkPos};
use azalea_core::registry_holder::RegistryHolder;
use bevy_ecs::{component::Component, entity::Entity};
use derive_more::{Deref, DerefMut};
use nohash_hasher::IntMap;

use crate::{ChunkStorage, PartialChunkStorage};

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
