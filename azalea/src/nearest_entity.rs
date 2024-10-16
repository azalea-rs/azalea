use azalea_entity::Position;
use azalea_world::{InstanceName, MinecraftEntityId};
use bevy_ecs::{
    prelude::Entity,
    query::{QueryFilter, With},
    system::{Query, SystemParam},
};

/// This system parameter can be used as a shorthand for quickly finding an
/// entity, (or several) close to a given position.
///
/// This system parameter allows for additional filtering of entities based off
/// of ECS marker components, such as `With<>`, `Without<>`, or `Added<>`, etc.
/// All functions used by this system parameter instance will respect the
/// applied filter.
///
/// ```
/// use azalea::chat::SendChatEvent;
/// use azalea::nearest_entity::EntityFinder;
/// use azalea_entity::metadata::{Player, AbstractMonster};
/// use azalea_entity::LocalEntity;
/// use bevy_ecs::system::Query;
/// use bevy_ecs::prelude::{Entity, EventWriter};
/// use bevy_ecs::query::With;
///
/// /// All bots near aggressive mobs will scream in chat.
/// pub fn bots_near_aggressive_mobs(
///     bots: Query<Entity, (With<LocalEntity>, With<Player>)>,
///     entity_finder: EntityFinder<With<AbstractMonster>>,
///     mut chat_events: EventWriter<SendChatEvent>,
/// ) {
///     for bot_id in bots.iter() {
///         let Some(nearest) = entity_finder.nearest_to_entity(bot_id, 16.0) else {
///             continue;
///         };
///
///         chat_events.send(SendChatEvent {
///             entity: bot_id,
///             content: String::from("Ahhh!"),
///         });
///     }
/// }
/// ```
#[derive(SystemParam)]
pub struct EntityFinder<'w, 's, F = ()>
where
    F: QueryFilter + 'static,
{
    all_entities:
        Query<'w, 's, (&'static Position, &'static InstanceName), With<MinecraftEntityId>>,

    filtered_entities: Query<
        'w,
        's,
        (Entity, &'static InstanceName, &'static Position),
        (With<MinecraftEntityId>, F),
    >,
}

impl<'w, 's, 'a, F> EntityFinder<'w, 's, F>
where
    F: QueryFilter + 'static,
{
    /// Gets the nearest entity to the given position and world instance name.
    /// This method will return `None` if there are no entities within range. If
    /// multiple entities are within range, only the closest one is returned.
    pub fn nearest_to_position(
        &'a self,
        position: &Position,
        instance_name: &InstanceName,
        max_distance: f64,
    ) -> Option<Entity> {
        let mut nearest_entity = None;
        let mut min_distance = max_distance;

        for (target_entity, e_instance, e_pos) in self.filtered_entities.iter() {
            if e_instance != instance_name {
                continue;
            }

            let target_distance = position.distance_to(e_pos);
            if target_distance < min_distance {
                nearest_entity = Some(target_entity);
                min_distance = target_distance;
            }
        }

        nearest_entity
    }

    /// Gets the nearest entity to the given entity. This method will return
    /// `None` if there are no entities within range. If multiple entities are
    /// within range, only the closest one is returned.
    pub fn nearest_to_entity(&'a self, entity: Entity, max_distance: f64) -> Option<Entity> {
        let Ok((position, instance_name)) = self.all_entities.get(entity) else {
            return None;
        };

        let mut nearest_entity = None;
        let mut min_distance = max_distance;

        for (target_entity, e_instance, e_pos) in self.filtered_entities.iter() {
            if entity == target_entity {
                continue;
            };

            if e_instance != instance_name {
                continue;
            }

            let target_distance = position.distance_to(e_pos);
            if target_distance < min_distance {
                nearest_entity = Some(target_entity);
                min_distance = target_distance;
            }
        }

        nearest_entity
    }

    /// This function get an iterator over all nearby entities to the given
    /// position within the given maximum distance. The entities in this
    /// iterator are not returned in any specific order.
    ///
    /// This function returns the Entity ID of nearby entities and their
    /// distance away.
    pub fn nearby_entities_to_position(
        &'a self,
        position: &'a Position,
        instance_name: &'a InstanceName,
        max_distance: f64,
    ) -> impl Iterator<Item = (Entity, f64)> + 'a {
        self.filtered_entities
            .iter()
            .filter_map(move |(target_entity, e_instance, e_pos)| {
                if e_instance != instance_name {
                    return None;
                }

                let distance = position.distance_to(e_pos);
                if distance < max_distance {
                    Some((target_entity, distance))
                } else {
                    None
                }
            })
    }

    /// This function get an iterator over all nearby entities to the given
    /// entity within the given maximum distance. The entities in this iterator
    /// are not returned in any specific order.
    ///
    /// This function returns the Entity ID of nearby entities and their
    /// distance away.
    pub fn nearby_entities_to_entity(
        &'a self,
        entity: Entity,
        max_distance: f64,
    ) -> impl Iterator<Item = (Entity, f64)> + 'a {
        let position;
        let instance_name;
        if let Ok((pos, instance)) = self.all_entities.get(entity) {
            position = *pos;
            instance_name = Some(instance);
        } else {
            position = Position::default();
            instance_name = None;
        };

        self.filtered_entities
            .iter()
            .filter_map(move |(target_entity, e_instance, e_pos)| {
                if entity == target_entity {
                    return None;
                }

                if Some(e_instance) != instance_name {
                    return None;
                }

                let distance = position.distance_to(e_pos);
                if distance < max_distance {
                    Some((target_entity, distance))
                } else {
                    None
                }
            })
    }
}
