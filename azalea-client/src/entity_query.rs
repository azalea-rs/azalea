use azalea_world::entity::Entity;
use bevy_ecs::query::{ReadOnlyWorldQuery, WorldQuery};

use crate::Client;

impl Client {
    /// A convenience function for getting components of our player's entity.
    pub fn query<'w, Q: WorldQuery>(
        &self,
        ecs: &'w mut bevy_ecs::world::World,
    ) -> <Q as WorldQuery>::Item<'w> {
        ecs.query::<Q>()
            .get_mut(ecs, self.entity)
            .expect("Our client is missing a required component.")
    }

    /// Return a lightweight [`Entity`] for the entity that matches the given
    /// predicate function.
    ///
    /// You can then use [`Self::map_entity`] to get components from this
    /// entity.
    pub fn entity_by<'a, F: ReadOnlyWorldQuery, Q: ReadOnlyWorldQuery>(
        &mut self,
        mut predicate: impl EntityPredicate<'a, Q>,
    ) -> Option<Entity> {
        let mut ecs = self.ecs.lock();
        let mut query = ecs.query_filtered::<(Entity, Q), F>();
        let entity = query
            .iter_mut(&mut ecs)
            .find(|(_, q)| predicate.matches(q))
            .map(|(entity, _)| entity);
        entity
    }
}

pub trait EntityPredicate<'a, Q: ReadOnlyWorldQuery> {
    fn matches(&self, components: &<Q as WorldQuery>::Item<'a>) -> bool;
}
impl<'a, F, Q> EntityPredicate<'a, Q> for F
where
    F: Fn(Q) -> bool,
    Q: ReadOnlyWorldQuery,
{
    fn matches(&self, query: &<Q as WorldQuery>::Item<'a>) -> bool {
        (self)(query)
    }
}
// impl<'a, F, Q1, Q2> EntityPredicate<'a, (Q1, Q2)> for F
// where
//     F: Fn(Q1, Q2) -> bool,
//     Q1: WorldQuery<Item<'a> = Q1>,
//     Q2: WorldQuery<Item<'a> = Q2>,
// {
//     fn matches(&self, query: <(Q1, Q2) as WorldQuery>::Item<'_>) -> bool {
//         (self)(query.0, query.1)
//     }
// }
