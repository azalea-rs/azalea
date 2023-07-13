use azalea_core::{BlockPos, Vec3};
use azalea_world::{InstanceContainer, InstanceName, MinecraftEntityId};
use bevy_ecs::prelude::*;
use log::{debug, error, info};

use crate::{EntityInfos, EntityUuid, EyeHeight, FluidOnEyes, LoadedBy, Local, Position};

/// Remove new entities that have the same id as an existing entity, and
/// increase the reference counts.
///
/// This is the reason why spawning entities into the ECS when you get a spawn
/// entity packet is okay. This system will make sure the new entity gets
/// combined into the old one.
#[allow(clippy::type_complexity)]
pub fn deduplicate_entities(
    mut commands: Commands,
    mut query: Query<
        (Entity, &MinecraftEntityId, &InstanceName),
        (Changed<MinecraftEntityId>, Without<Local>),
    >,
    mut loaded_by_query: Query<&mut LoadedBy>,
    instance_container: Res<InstanceContainer>,
) {
    // if this entity already exists, remove it
    for (new_entity, id, world_name) in query.iter_mut() {
        if let Some(world_lock) = instance_container.get(world_name) {
            let world = world_lock.write();
            if let Some(old_entity) = world.entity_by_id.get(id) {
                if old_entity == &new_entity {
                    continue;
                }

                // this entity already exists!!! remove the one we just added but increase
                // the reference count
                let new_loaded_by = loaded_by_query
                    .get(new_entity)
                    .unwrap_or_else(|_| panic!(
                        "Entities should always have the LoadedBy component ({new_entity:?} did not)"
                    ))
                    .clone();
                let old_loaded_by = loaded_by_query.get_mut(*old_entity);
                // merge them if possible
                if let Ok(mut old_loaded_by) = old_loaded_by {
                    old_loaded_by.extend(new_loaded_by.iter());
                }
                commands.entity(new_entity).despawn();
                info!(
                    "Entity with id {id:?} / {new_entity:?} already existed in the world, merging it with {old_entity:?}"
                );
                break;
            }
        } else {
            error!("Entity was inserted into a world that doesn't exist.");
        }
    }
}

// when a local entity is added, if there was already an entity with the same id
// then delete the old entity
#[allow(clippy::type_complexity)]
pub fn deduplicate_local_entities(
    mut commands: Commands,
    mut query: Query<
        (Entity, &MinecraftEntityId, &InstanceName),
        (Changed<MinecraftEntityId>, With<Local>),
    >,
    instance_container: Res<InstanceContainer>,
) {
    // if this entity already exists, remove the old one
    for (new_entity, id, world_name) in query.iter_mut() {
        if let Some(world_lock) = instance_container.get(world_name) {
            let world = world_lock.write();
            if let Some(old_entity) = world.entity_by_id.get(id) {
                if old_entity == &new_entity {
                    // lol
                    continue;
                }

                commands.entity(*old_entity).despawn();
                debug!(
                    "Added local entity {id:?} / {new_entity:?} but already existed in world as {old_entity:?}, despawning {old_entity:?}"
                );
                break;
            }
        } else {
            error!("Entity was inserted into a world that doesn't exist.");
        }
    }
}

pub fn update_uuid_index(
    mut entity_infos: ResMut<EntityInfos>,
    query: Query<(Entity, &EntityUuid, Option<&Local>), Changed<EntityUuid>>,
) {
    for (entity, &uuid, local) in query.iter() {
        // only add it if it doesn't already exist in
        // entity_infos.entity_by_uuid
        if local.is_none() {
            if let Some(old_entity) = entity_infos.entity_by_uuid.get(&uuid) {
                debug!(
                    "Entity with UUID {uuid:?} already existed in the world, not adding to
        index (old ecs id: {old_entity:?} / new ecs id: {entity:?})"
                );
                continue;
            }
        }
        entity_infos.entity_by_uuid.insert(*uuid, entity);
    }
}

// /// Clear all entities in a chunk. This will not clear them from the
// /// shared storage unless there are no other references to them.
// pub fn clear_entities_in_chunk(
//     mut commands: Commands,
//     partial_entity_infos: &mut PartialEntityInfos,
//     chunk: &ChunkPos,
//     instance_container: &WorldContainer,
//     world_name: &WorldName,
//     mut query: Query<(&MinecraftEntityId, &mut ReferenceCount)>,
// ) { let world_lock = instance_container.get(world_name).unwrap(); let world =
//   world_lock.read();

//     if let Some(entities) = world.entities_by_chunk.get(chunk).cloned() {
//         for &entity in &entities {
//             let (id, mut reference_count) = query.get_mut(entity).unwrap();
//             if partial_entity_infos.loaded_entity_ids.remove(id) {
//                 // decrease the reference count
//                 **reference_count -= 1;
//             }
//         }
//     }
// }

/// System to keep the entity_by_id index up-to-date.
pub fn update_entity_by_id_index(
    mut query: Query<
        (Entity, &MinecraftEntityId, &InstanceName, Option<&Local>),
        Changed<MinecraftEntityId>,
    >,
    instance_container: Res<InstanceContainer>,
) {
    for (entity, id, world_name, local) in query.iter_mut() {
        let world_lock = instance_container.get(world_name).unwrap();
        let mut world = world_lock.write();
        if local.is_none() {
            if let Some(old_entity) = world.entity_by_id.get(id) {
                debug!(
                    "Entity with ID {id:?} already existed in the world, not adding to
        index (old ecs id: {old_entity:?} / new ecs id: {entity:?})"
                );
                continue;
            }
        }
        world.entity_by_id.insert(*id, entity);
        debug!("Added {entity:?} to {world_name:?} with {id:?}.");
    }
}

pub fn update_fluid_on_eyes(
    mut query: Query<(&mut FluidOnEyes, &Position, &EyeHeight, &InstanceName)>,
    instance_container: Res<InstanceContainer>,
) {
    for (mut fluid_on_eyes, position, eye_height, instance_name) in query.iter_mut() {
        let Some(instance) = instance_container.get(instance_name) else {
            continue;
        };

        let adjusted_eye_y = position.y + (**eye_height as f64) - 0.1111111119389534;
        let eye_block_pos = BlockPos::from(Vec3::new(position.x, adjusted_eye_y, position.z));
        let fluid_at_eye = instance
            .read()
            .get_fluid_state(&eye_block_pos)
            .unwrap_or_default();
        let fluid_cutoff_y = eye_block_pos.y as f64 + (fluid_at_eye.height as f64 / 16f64);
        if fluid_cutoff_y > adjusted_eye_y {
            **fluid_on_eyes = fluid_at_eye.fluid;
        } else {
            **fluid_on_eyes = azalea_registry::Fluid::Empty;
        }
    }
}
