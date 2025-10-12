use azalea_core::aabb::Aabb;
use azalea_entity::{
    Physics,
    metadata::{AbstractBoat, Shulker},
};
use azalea_world::Instance;
use bevy_ecs::{
    component::Component,
    entity::Entity,
    query::{Changed, Or, With},
    system::{Commands, Query},
};
use tracing::error;

use super::VoxelShape;

/// This query matches on entities that we can collide with (boats and
/// shulkers).
///
/// If you want to use this in a more complex query, use
/// [`CollidableEntityFilter`] as a filter instead.
pub type CollidableEntityQuery<'world, 'state> = Query<'world, 'state, (), CollidableEntityFilter>;
/// This filter matches on entities that we can collide with (boats and
/// shulkers).
///
/// Use [`CollidableEntityQuery`] if you want an empty query that matches with
/// this.
pub type CollidableEntityFilter = Or<(With<AbstractBoat>, With<Shulker>)>;

/// A component that mirrors the Physics::bounding_box of every entity, but is
/// updated before client-side physics is done.
#[derive(Component)]
pub struct LastBoundingBox(pub Aabb);

pub type AabbQuery<'world, 'state, 'a> = Query<'world, 'state, &'a LastBoundingBox>;

/// Update the [`LastBoundingBox`] for every entity.
pub fn update_last_bounding_box(
    mut commands: Commands,
    mut query: Query<(Entity, Option<&mut LastBoundingBox>, &Physics), Changed<Physics>>,
) {
    for (entity, mut last_bounding_box, physics) in &mut query {
        if let Some(last_bounding_box) = last_bounding_box.as_mut() {
            last_bounding_box.0 = physics.bounding_box;
        } else {
            commands
                .entity(entity)
                .insert(LastBoundingBox(physics.bounding_box));
        }
    }
}

pub fn get_entity_collisions(
    world: &Instance,
    aabb: &Aabb,
    source_entity: Option<Entity>,
    aabb_query: &AabbQuery,
    collidable_entity_query: &CollidableEntityQuery,
) -> Vec<VoxelShape> {
    if aabb.size() < 1.0E-7 {
        return vec![];
    }

    let collision_predicate = |entity| collidable_entity_query.get(entity).is_ok();

    let collidable_entities = get_entities(
        world,
        source_entity,
        &aabb.inflate_all(1.0E-7),
        &collision_predicate,
        aabb_query,
    );

    collidable_entities
        .into_iter()
        .map(|(_entity, aabb)| VoxelShape::from(aabb))
        .collect()
}

/// Return all entities that are colliding with the given bounding box and match
/// the given predicate.
///
/// `source_entity` is the entity that the bounding box belongs to, and won't be
/// one of the returned entities.
pub fn get_entities(
    world: &Instance,
    source_entity: Option<Entity>,
    aabb: &Aabb,
    predicate: &dyn Fn(Entity) -> bool,
    aabb_query: &AabbQuery,
) -> Vec<(Entity, Aabb)> {
    let mut matches = Vec::new();

    super::world_collisions::for_entities_in_chunks_colliding_with(
        world,
        aabb,
        |_chunk_pos, entities_in_chunk| {
            // now check if the entity itself collides
            for &candidate in entities_in_chunk {
                if Some(candidate) != source_entity && predicate(candidate) {
                    let Ok(candidate_aabb) = aabb_query.get(candidate) else {
                        error!(
                            "Entity {candidate} (found from for_entities_in_chunks_colliding_with) is missing required components."
                        );
                        continue;
                    };
                    let candidate_aabb = &candidate_aabb.0;

                    if aabb.intersects_aabb(candidate_aabb) {
                        matches.push((candidate, candidate_aabb.to_owned()));
                    }
                }
            }
        },
    );

    matches
}
