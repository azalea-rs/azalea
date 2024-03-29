use std::sync::Arc;

use bevy_ecs::{
    component::Component,
    entity::Entity,
    query::QueryData,
    query::{QueryFilter, ROQueryItem},
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
                    std::any::type_name::<D>()
                )
            })
    }

    /// Return a lightweight [`Entity`] for the entity that matches the given
    /// predicate function.
    ///
    /// You can then use [`Self::entity_component`] to get components from this
    /// entity.
    ///
    /// # Example
    /// Note that this will very likely change in the future.
    /// ```
    /// use azalea_client::{Client, GameProfileComponent};
    /// use bevy_ecs::query::With;
    /// use azalea_entity::{Position, metadata::Player};
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
    pub fn entity_by<F: QueryFilter, Q: QueryData>(
        &mut self,
        predicate: impl EntityPredicate<Q, F>,
    ) -> Option<Entity> {
        predicate.find(self.ecs.clone())
    }

    /// Get a component from an entity. Note that this will return an owned type
    /// (i.e. not a reference) so it may be expensive for larger types.
    ///
    /// If you're trying to get a component for this client, use
    /// [`Self::component`].
    pub fn entity_component<Q: Component + Clone>(&mut self, entity: Entity) -> Q {
        let mut ecs = self.ecs.lock();
        let mut q = ecs.query::<&Q>();
        let components = q.get(&ecs, entity).unwrap_or_else(|_| {
            panic!(
                "Entity is missing a required component {:?}",
                std::any::type_name::<Q>()
            )
        });
        components.clone()
    }

    /// Get a component from an entity, if it exists. This is similar to
    /// [`Self::entity_component`] but returns an `Option` instead of panicking
    /// if the component isn't present.
    pub fn get_entity_component<Q: Component + Clone>(&mut self, entity: Entity) -> Option<Q> {
        let mut ecs = self.ecs.lock();
        let mut q = ecs.query::<&Q>();
        let components = q.get(&ecs, entity).ok();
        components.cloned()
    }
}

pub trait EntityPredicate<Q: QueryData, Filter: QueryFilter> {
    fn find(&self, ecs_lock: Arc<Mutex<World>>) -> Option<Entity>;
}
impl<F, Q, Filter> EntityPredicate<Q, Filter> for F
where
    F: Fn(&ROQueryItem<Q>) -> bool,
    Q: QueryData,
    Filter: QueryFilter,
{
    fn find(&self, ecs_lock: Arc<Mutex<World>>) -> Option<Entity> {
        let mut ecs = ecs_lock.lock();
        let mut query = ecs.query_filtered::<(Entity, Q), Filter>();
        let entity = query.iter(&ecs).find(|(_, q)| (self)(q)).map(|(e, _)| e);

        entity
    }
}

// impl<'a, F, Q1, Q2> EntityPredicate<'a, (Q1, Q2)> for F
// where
//     F: Fn(&<Q1 as WorldQuery>::Item<'_>, &<Q2 as WorldQuery>::Item<'_>) ->
// bool,     Q1: QueryFilter,
//     Q2: QueryFilter,
// {
//     fn find(&self, ecs: &mut Ecs) -> Option<Entity> {
//         // (self)(query)
//         let mut query = ecs.query_filtered::<(Entity, Q1, Q2), ()>();
//         let entity = query
//             .iter(ecs)
//             .find(|(_, q1, q2)| (self)(q1, q2))
//             .map(|(e, _, _)| e);

//         entity
//     }
// }
