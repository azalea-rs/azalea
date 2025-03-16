// default List<VoxelShape> getEntityCollisions(@Nullable Entity entity, AABB
// aabb) {     if (aabb.getSize() < 1.0E-7) {
//        return List.of();
//     } else {
//        Predicate var3 = entity == null ? EntitySelector.CAN_BE_COLLIDED_WITH
//              : EntitySelector.NO_SPECTATORS.and(entity::canCollideWith);
//        List var4 = this.getEntities(entity, aabb.inflate(1.0E-7), var3);
//        if (var4.isEmpty()) {
//           return List.of();
//        } else {
//           Builder var5 = ImmutableList.builderWithExpectedSize(var4.size());

//           for (Entity var7 : var4) {
//              var5.add(Shapes.create(var7.getBoundingBox()));
//           }

//           return var5.build();
//        }
//     }
//  }

use azalea_core::aabb::AABB;
use azalea_entity::{
    LocalEntity, Physics,
    metadata::{AbstractBoat, Shulker},
};
use azalea_world::Instance;
use bevy_ecs::{
    entity::Entity,
    query::{Or, With, Without},
    system::Query,
};
use tracing::error;

use super::VoxelShape;

/// This query matches on entities that we can collide with. That is, boats and
/// shulkers.
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

pub type PhysicsQuery<'world, 'state, 'a> =
    Query<'world, 'state, &'a Physics, Without<LocalEntity>>;

pub fn get_entity_collisions(
    world: &Instance,
    aabb: &AABB,
    source_entity: Option<Entity>,
    physics_query: &PhysicsQuery,
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
        physics_query,
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
    aabb: &AABB,
    predicate: &dyn Fn(Entity) -> bool,
    physics_query: &PhysicsQuery,
) -> Vec<(Entity, AABB)> {
    let mut matches = Vec::new();

    super::world_collisions::for_entities_in_chunks_colliding_with(
        world,
        aabb,
        |_chunk_pos, entities_in_chunk| {
            // now check if the entity itself collides
            for &candidate in entities_in_chunk {
                if Some(candidate) != source_entity && predicate(candidate) {
                    let Ok(physics) = physics_query.get(candidate) else {
                        error!(
                            "Entity {candidate} (found from for_entities_in_chunks_colliding_with) is missing required components."
                        );
                        continue;
                    };

                    let candidate_aabb = physics.bounding_box;
                    if aabb.intersects_aabb(&candidate_aabb) {
                        matches.push((candidate, physics.bounding_box));
                    }
                }
            }
        },
    );

    matches
}
