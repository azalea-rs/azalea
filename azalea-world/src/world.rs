use std::fmt::{self, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::io::{self, Cursor};
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use azalea_block::fluid_state::FluidState;
use azalea_block::BlockState;
use azalea_buf::{AzaleaRead, AzaleaReadVar, AzaleaWrite, AzaleaWriteVar, BufReadError};
use azalea_core::position::{BlockPos, ChunkPos};
use azalea_core::registry_holder::RegistryHolder;
use bevy_ecs::{component::Component, entity::Entity};
use derive_more::{Deref, DerefMut};
use nohash_hasher::IntMap;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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

/// An entity ID used by Minecraft.
///
/// These IDs are picked by the server. Some server softwares (like Bungeecord)
/// may pick entity IDs per-player, so you should avoid relying on them for
/// identifying IDs (especially if you're using a shared world -- i.e. a swarm).
///
/// You might find [`Entity`] more useful, since that's an ID decided by us that
/// is likely to be correct across shared worlds. You could also use the
/// `EntityUuid` from `azalea_entity`, that one is unlikely to change even
/// across server restarts.
///
/// This serializes as a i32. Usually it's a VarInt in the protocol, but not
/// always. If you do need it to serialize as a VarInt, make sure to use use the
/// `#[var]` attribute.
///
/// [`Entity`]: bevy_ecs::entity::Entity
#[derive(Component, Copy, Clone, Debug, Default, PartialEq, Eq, Deref, DerefMut)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct MinecraftEntityId(pub i32);

impl Hash for MinecraftEntityId {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.write_i32(self.0);
    }
}
impl nohash_hasher::IsEnabled for MinecraftEntityId {}

// we can't have the default be #[var] because mojang doesn't use varints for
// entities sometimes :(
impl AzaleaRead for MinecraftEntityId {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        i32::azalea_read(buf).map(MinecraftEntityId)
    }
}
impl AzaleaWrite for MinecraftEntityId {
    fn azalea_write(&self, buf: &mut impl io::Write) -> Result<(), io::Error> {
        i32::azalea_write(&self.0, buf)
    }
}
impl AzaleaReadVar for MinecraftEntityId {
    fn azalea_read_var(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        i32::azalea_read_var(buf).map(MinecraftEntityId)
    }
}
impl AzaleaWriteVar for MinecraftEntityId {
    fn azalea_write_var(&self, buf: &mut impl io::Write) -> Result<(), io::Error> {
        i32::azalea_write_var(&self.0, buf)
    }
}
impl Display for MinecraftEntityId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "eid({})", self.0)
    }
}

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

    /// An index of Minecraft entity IDs to Azalea ECS entities.
    ///
    /// You should avoid using this (particularly if you're using swarms) and
    /// instead use `azalea_entity::EntityIdIndex`, since some servers may
    /// give different entity IDs for the same entities to different
    /// players.
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
