use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Debug},
};

use azalea_block::{BlockState, fluid_state::FluidState};
use azalea_core::{
    position::{BlockPos, ChunkPos},
    registry_holder::RegistryHolder,
};
use azalea_registry::data::Biome;
use bevy_ecs::entity::Entity;
use nohash_hasher::IntMap;

use crate::{ChunkStorage, PartialChunkStorage};

/// A reference to a slice of the world, as seen by an individual client.
///
/// A `PartialWorld` will typically be owned by a client, and it holds strong
/// references to chunks and entities in a [`World`]s.
///
/// In other words, this type holds the chunks and entities that are within
/// render distance for a client. The same chunk/entity may be referenced by
/// more than one `PartialWorld`.
pub struct PartialWorld {
    pub chunks: PartialChunkStorage,
    /// Some metadata about entities, like what entities are in certain chunks.
    /// This does not contain the entity data itself, that's in the ECS.
    pub entity_infos: PartialEntityInfos,
}

impl PartialWorld {
    pub fn new(chunk_radius: u32, owner_entity: Option<Entity>) -> Self {
        PartialWorld {
            chunks: PartialChunkStorage::new(chunk_radius),
            entity_infos: PartialEntityInfos::new(owner_entity),
        }
    }

    /// Clears the internal references to chunks in the [`PartialWorld`] and
    /// resets the view center.
    pub fn reset(&mut self) {
        self.chunks = PartialChunkStorage::new(self.chunks.chunk_radius);
    }
}

#[deprecated = "moved to `azalea_core::entity_id::MinecraftEntityId`."]
pub type MinecraftEntityId = azalea_core::entity_id::MinecraftEntityId;

/// Keep track of certain metadatas that are only relevant for this partial
/// world.
#[derive(Debug, Default)]
pub struct PartialEntityInfos {
    /// The entity ID of the player that owns this partial world.
    ///
    /// This will make `RelativeEntityUpdate` pretend this entity doesn't exist
    /// so it doesn't get modified from outside sources.
    pub owner_entity: Option<Entity>,
    /// A counter for each entity that tracks how many updates we've observed
    /// for it.
    ///
    /// This is used for shared worlds (i.e. swarms), to make sure we don't
    /// update entities twice on accident.
    pub updates_received: IntMap<azalea_core::entity_id::MinecraftEntityId, u32>,
    // ^ note: using MinecraftEntityId for entity ids is acceptable here since
    // there's no chance of collisions
}

impl PartialEntityInfos {
    pub fn new(owner_entity: Option<Entity>) -> Self {
        Self {
            owner_entity,
            updates_received: IntMap::default(),
        }
    }
}

/// A Minecraft world, sometimes referred to as a dimension.
///
/// This is the most commonly used type to interact with the world that a client
/// is in. In here, all chunks are stored as weak pointers, which means that
/// clients in different areas of the same world can share the same `World`
/// instance.
///
/// Also see [`PartialWorld`].
///
/// Note that this is distinct from Bevy's `World` type, which contains all of
/// the data for every client. When referring to the Bevy `World` within Azalea,
/// the term "ECS" is generally used instead.
#[derive(Debug, Default)]
#[doc(alias("instance", "dimension", "level"))]
pub struct World {
    pub chunks: ChunkStorage,

    /// An index of all the entities we know are in the chunks of the world
    pub entities_by_chunk: HashMap<ChunkPos, HashSet<Entity>>,

    /// An index of Minecraft entity IDs to Azalea ECS entities.
    ///
    /// You should avoid using this (particularly if you're using swarms) and
    /// instead use `azalea_entity::EntityIdIndex`, since some servers may
    /// give different entity IDs for the same entities to different
    /// players.
    pub entity_by_id: IntMap<azalea_core::entity_id::MinecraftEntityId, Entity>,

    pub registries: RegistryHolder,
}

#[deprecated = "renamed to `World`."]
pub type Instance = World;

impl World {
    /// Get the block at the given position, or `None` if it's outside of the
    /// world that we have loaded.
    pub fn get_block_state(&self, pos: BlockPos) -> Option<BlockState> {
        self.chunks.get_block_state(pos)
    }

    /// Similar to [`Self::get_block_state`], but returns data about the fluid
    /// at the position, including for waterlogged blocks.
    pub fn get_fluid_state(&self, pos: BlockPos) -> Option<FluidState> {
        self.chunks.get_block_state(pos).map(FluidState::from)
    }

    /// Get the biome at the given position.
    ///
    /// You can then use `Client::with_resolved_registry` to get the name and
    /// data from the biome.
    ///
    /// Note that biomes are internally stored as 4x4x4 blocks, so if you're
    /// writing code that searches for a specific biome it'll probably be more
    /// efficient to avoid scanning every single block.
    pub fn get_biome(&self, pos: BlockPos) -> Option<Biome> {
        self.chunks.get_biome(pos)
    }

    pub fn set_block_state(&self, pos: BlockPos, state: BlockState) -> Option<BlockState> {
        self.chunks.set_block_state(pos, state)
    }
}

impl Debug for PartialWorld {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PartialWorld")
            .field("chunks", &self.chunks)
            .field("entity_infos", &self.entity_infos)
            .finish()
    }
}

impl Default for PartialWorld {
    /// Creates a completely self-contained [`PartialWorld`]. This is only for
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

impl From<ChunkStorage> for World {
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
