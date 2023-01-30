use std::sync::Arc;

use azalea_ecs::{
    component::Component,
    ecs::Ecs,
    entity::Entity,
    query::{ROQueryItem, ReadOnlyWorldQuery, WorldQuery},
};
use parking_lot::Mutex;

use crate::Client;

impl Client {
    /// A convenience function for getting components of our player's entity.
    pub fn query<'w, Q: WorldQuery>(&self, ecs: &'w mut Ecs) -> <Q as WorldQuery>::Item<'w> {
        ecs.query::<Q>()
            .get_mut(ecs, self.entity)
            .expect("Our client is missing a required component.")
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
    /// let entity = bot.entity_by::<With<Player>, (&GameProfileComponent,)>(
    ///     |profile: &&GameProfileComponent| profile.name == sender,
    /// );
    /// if let Some(entity) = entity {
    ///     let position = bot.entity_component::<Position>(entity);
    ///     // ...
    /// }
    /// ```
    pub fn entity_by<F: ReadOnlyWorldQuery, Q: ReadOnlyWorldQuery>(
        &mut self,
        predicate: impl EntityPredicate<Q, F>,
    ) -> Option<Entity> {
        predicate.find(self.ecs.clone())
    }

    /// Get a component from an entity. Note that this will return an owned type
    /// (i.e. not a reference) so it may be expensive for larger types.
    pub fn entity_component<Q: Component + Clone>(&mut self, entity: Entity) -> Q {
        let mut ecs = self.ecs.lock();
        let mut q = ecs.query::<&Q>();
        let components = q
            .get(&ecs, entity)
            .expect("Entity components must be present in Client::entity)components.");
        components.clone()
    }
}

pub trait EntityPredicate<Q: ReadOnlyWorldQuery, Filter: ReadOnlyWorldQuery> {
    fn find(&self, ecs_lock: Arc<Mutex<Ecs>>) -> Option<Entity>;
}
impl<F, Q, Filter> EntityPredicate<(Q,), Filter> for F
where
    F: Fn(&ROQueryItem<Q>) -> bool,
    Q: ReadOnlyWorldQuery,
    Filter: ReadOnlyWorldQuery,
{
    fn find(&self, ecs_lock: Arc<Mutex<Ecs>>) -> Option<Entity> {
        let mut ecs = ecs_lock.lock();
        let mut query = ecs.query_filtered::<(Entity, Q), Filter>();
        let entity = query.iter(&ecs).find(|(_, q)| (self)(q)).map(|(e, _)| e);

        entity
    }
}

// impl<'a, F, Q1, Q2> EntityPredicate<'a, (Q1, Q2)> for F
// where
//     F: Fn(&<Q1 as WorldQuery>::Item<'_>, &<Q2 as WorldQuery>::Item<'_>) ->
// bool,     Q1: ReadOnlyWorldQuery,
//     Q2: ReadOnlyWorldQuery,
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
