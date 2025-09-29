use std::{any, sync::Arc};

use azalea_core::position::Vec3;
use azalea_entity::Position;
use azalea_world::InstanceName;
use bevy_ecs::{
    component::Component,
    entity::Entity,
    query::{QueryData, QueryFilter, QueryItem, ROQueryItem},
    world::World,
};
use parking_lot::Mutex;

use crate::Client;

impl Client {
    /// A convenience function for getting components from our client's entity.
    ///
    /// # Examples
    /// ```
    /// # use azalea_world::InstanceName;
    /// # fn example(mut client: azalea_client::Client) {
    /// let is_logged_in = client.query_self::<Option<&InstanceName>, _>(|ins| ins.is_some());
    /// # }
    /// ```
    ///
    /// # Panics
    ///
    /// This will panic if the component doesn't exist on the client.
    pub fn query_self<D: QueryData, R>(&self, f: impl FnOnce(QueryItem<D>) -> R) -> R {
        let mut ecs = self.ecs.lock();
        let mut qs = ecs.query::<D>();
        let res = qs.get_mut(&mut ecs, self.entity).unwrap_or_else(|_| {
            panic!(
                "Our client is missing a required component {:?}",
                any::type_name::<D>()
            )
        });
        f(res)
    }

    /// Quickly returns a lightweight [`Entity`] for an arbitrary entity that
    /// matches the given predicate function that is in the same
    /// [`Instance`] as the client.
    ///
    /// You can then use [`Self::entity_component`] to get components from this
    /// entity.
    ///
    /// If you want to find the nearest entity, consider using
    /// [`Self::nearest_entity_by`] instead. If you want to find all entities
    /// that match the predicate, use [`Self::nearest_entities_by`].
    ///
    /// # Example
    /// ```
    /// use azalea_client::{Client, player::GameProfileComponent};
    /// use azalea_entity::{Position, metadata::Player};
    /// use bevy_ecs::query::With;
    ///
    /// # fn example(mut bot: Client, sender_name: String) {
    /// let entity = bot.any_entity_by::<With<Player>, (&GameProfileComponent,)>(
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
    pub fn any_entity_by<F: QueryFilter, Q: QueryData>(
        &self,
        predicate: impl EntityPredicate<Q, F>,
    ) -> Option<Entity> {
        let instance_name = self.get_component::<InstanceName>()?;
        predicate.find_any(self.ecs.clone(), &instance_name)
    }

    /// Return a lightweight [`Entity`] for the nearest entity that matches the
    /// given predicate function.
    ///
    /// You can then use [`Self::entity_component`] to get components from this
    /// entity.
    ///
    /// If you don't need the entity to be the nearest one, it may be more
    /// efficient to use [`Self::any_entity_by`] instead. You can also use
    /// [`Self::nearest_entities_by`] to get all nearby entities.
    ///
    /// ```
    /// use azalea_entity::{LocalEntity, Position, metadata::Player};
    /// use bevy_ecs::query::{With, Without};
    ///
    /// # fn example(mut bot: azalea_client::Client, sender_name: String) {
    /// // get the position of the nearest player
    /// if let Some(nearest_player) =
    ///     bot.nearest_entity_by::<(With<Player>, Without<LocalEntity>), ()>(|_: &()| true)
    /// {
    ///     let nearest_player_pos = *bot.entity_component::<Position>(nearest_player);
    ///     bot.chat(format!("You are at {nearest_player_pos}"));
    /// }
    /// # }
    /// ```
    ///
    /// [`Entity`]: bevy_ecs::entity::Entity
    pub fn nearest_entity_by<F: QueryFilter, Q: QueryData>(
        &self,
        predicate: impl EntityPredicate<Q, F>,
    ) -> Option<Entity> {
        self.nearest_entities_by(predicate).first().copied()
    }

    /// Similar to [`Self::nearest_entity_by`] but returns a `Vec<Entity>` of
    /// all entities in our instance that match the predicate.
    ///
    /// The first entity is the nearest one.
    ///
    /// ```
    /// # use azalea_entity::{LocalEntity, Position, metadata::Player};
    /// # use bevy_ecs::query::{With, Without};
    /// # fn example(mut bot: azalea_client::Client, sender_name: String) {
    /// let nearby_players =
    ///     bot.nearest_entities_by::<(With<Player>, Without<LocalEntity>), ()>(|_: &()| true);
    /// # }
    /// ```
    pub fn nearest_entities_by<F: QueryFilter, Q: QueryData>(
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
