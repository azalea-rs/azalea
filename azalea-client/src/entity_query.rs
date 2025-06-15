use std::{any, sync::Arc};

use azalea_core::position::Vec3;
use azalea_entity::Position;
use azalea_world::InstanceName;
use bevy_ecs::{
    component::Component,
    entity::Entity,
    query::{QueryData, QueryFilter, ROQueryItem},
    world::World,
};
use parking_lot::Mutex;

use crate::Client;

impl Client {
    /// A convenience function for getting components of our player's entity.
    ///
    /// # Examples
    /// ```
    /// # use azalea_world::InstanceName;
    /// # fn example(mut client: azalea_client::Client) {
    /// let is_logged_in = client
    ///     .query::<Option<&InstanceName>>(&mut client.ecs.lock())
    ///     .is_some();
    /// # }
    /// ```
    pub fn query<'w, D: QueryData>(&self, ecs: &'w mut World) -> D::Item<'w> {
        ecs.query::<D>()
            .get_mut(ecs, self.entity)
            .unwrap_or_else(|_| {
                panic!(
                    "Our client is missing a required component {:?}",
                    any::type_name::<D>()
                )
            })
    }

    /// Return a lightweight [`Entity`] for an arbitrary entity that matches the
    /// given predicate function that is in the same [`Instance`] as the
    /// client.
    ///
    /// You can then use [`Self::entity_component`] to get components from this
    /// entity.
    ///
    /// Also see [`Self::entities_by`] which will return all entities that match
    /// the predicate and sorts them by distance (unlike `entity_by`).
    ///
    /// # Example
    /// ```
    /// use azalea_client::{Client, player::GameProfileComponent};
    /// use azalea_entity::{Position, metadata::Player};
    /// use bevy_ecs::query::With;
    ///
    /// # fn example(mut bot: Client, sender_name: String) {
    /// let entity = bot.entity_by::<With<Player>, (&GameProfileComponent,)>(
    ///     |(profile,): &(&GameProfileComponent,)| profile.name == sender_name,
    /// );
    /// if let Some(entity) = entity {
    ///     let position = bot.entity_component::<Position>(entity);
    ///     // ...
    /// }
    /// # }
    /// ```
    ///
    /// [`Entity`]: bevy_ecs::entity::Entity
    /// [`Instance`]: azalea_world::Instance
    pub fn entity_by<F: QueryFilter, Q: QueryData>(
        &self,
        predicate: impl EntityPredicate<Q, F>,
    ) -> Option<Entity> {
        let instance_name = self.get_component::<InstanceName>()?;
        predicate.find_any(self.ecs.clone(), &instance_name)
    }

    /// Similar to [`Self::entity_by`] but returns a `Vec<Entity>` of all
    /// entities in our instance that match the predicate.
    ///
    /// Unlike `entity_by`, the result is sorted by distance to our client's
    /// position, so the closest entity is first.
    pub fn entities_by<F: QueryFilter, Q: QueryData>(
        &self,
        predicate: impl EntityPredicate<Q, F>,
    ) -> Vec<Entity> {
        let Some(instance_name) = self.get_component::<InstanceName>() else {
            return vec![];
        };
        let Some(position) = self.get_component::<Position>() else {
            return vec![];
        };
        predicate.find_all_sorted(self.ecs.clone(), &instance_name, (&position).into())
    }

    /// Get a component from an entity. Note that this will return an owned type
    /// (i.e. not a reference) so it may be expensive for larger types.
    ///
    /// If you're trying to get a component for this client, use
    /// [`Self::component`].
    pub fn entity_component<Q: Component + Clone>(&self, entity: Entity) -> Q {
        let mut ecs = self.ecs.lock();
        let mut q = ecs.query::<&Q>();
        let components = q.get(&ecs, entity).unwrap_or_else(|_| {
            panic!(
                "Entity is missing a required component {:?}",
                any::type_name::<Q>()
            )
        });
        components.clone()
    }

    /// Get a component from an entity, if it exists. This is similar to
    /// [`Self::entity_component`] but returns an `Option` instead of panicking
    /// if the component isn't present.
    pub fn get_entity_component<Q: Component + Clone>(&self, entity: Entity) -> Option<Q> {
        let mut ecs = self.ecs.lock();
        let mut q = ecs.query::<&Q>();
        let components = q.get(&ecs, entity).ok();
        components.cloned()
    }
}

pub trait EntityPredicate<Q: QueryData, Filter: QueryFilter> {
    fn find_any(&self, ecs_lock: Arc<Mutex<World>>, instance_name: &InstanceName)
    -> Option<Entity>;
    fn find_all_sorted(
        &self,
        ecs_lock: Arc<Mutex<World>>,
        instance_name: &InstanceName,
        nearest_to: Vec3,
    ) -> Vec<Entity>;
}
impl<F, Q: QueryData, Filter: QueryFilter> EntityPredicate<Q, Filter> for F
where
    F: Fn(&ROQueryItem<Q>) -> bool,
{
    fn find_any(
        &self,
        ecs_lock: Arc<Mutex<World>>,
        instance_name: &InstanceName,
    ) -> Option<Entity> {
        let mut ecs = ecs_lock.lock();
        let mut query = ecs.query_filtered::<(Entity, &InstanceName, Q), Filter>();
        query
            .iter(&ecs)
            .find(|(_, e_instance_name, q)| *e_instance_name == instance_name && (self)(q))
            .map(|(e, _, _)| e)
    }

    fn find_all_sorted(
        &self,
        ecs_lock: Arc<Mutex<World>>,
        instance_name: &InstanceName,
        nearest_to: Vec3,
    ) -> Vec<Entity> {
        let mut ecs = ecs_lock.lock();
        let mut query = ecs.query_filtered::<(Entity, &InstanceName, &Position, Q), Filter>();
        let mut entities = query
            .iter(&ecs)
            .filter(|(_, e_instance_name, _, q)| *e_instance_name == instance_name && (self)(q))
            .map(|(e, _, position, _)| (e, Vec3::from(position)))
            .collect::<Vec<(Entity, Vec3)>>();

        entities.sort_by_cached_key(|(_, position)| {
            // to_bits is fine here as long as the number is positive
            position.distance_squared_to(nearest_to).to_bits()
        });

        entities
            .into_iter()
            .map(|(e, _)| e)
            .collect::<Vec<Entity>>()
    }
}
