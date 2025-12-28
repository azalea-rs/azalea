use std::{any, sync::Arc};

use azalea_core::position::Vec3;
use azalea_entity::Position;
use azalea_world::InstanceName;
use bevy_ecs::{
    component::Component,
    entity::Entity,
    query::{QueryData, QueryEntityError, QueryFilter, QueryItem, ROQueryItem},
    world::World,
};
use parking_lot::{MappedRwLockReadGuard, RwLock, RwLockReadGuard};

use crate::Client;

impl Client {
    /// Get a component from the client.
    ///
    /// This allows you to access certain data stored about the client entity
    /// that isn't accessible in a simpler way.
    ///
    /// This returns a reference to the component wrapped by a read guard. This
    /// makes the component cheap to access, but means that the ECS cannot be
    /// mutated while it's in scope. In some cases, it may be simpler for you to
    /// immediately clone the component after accessing it.
    ///
    /// If the component isn't guaranteed to be present, consider using
    /// [`Self::get_component`] instead.
    ///
    /// To do more complex queries or to mutate data, see [`Self::query_self`].
    ///
    /// To access data about other entities, you can use
    /// [`Self::entity_component`] (and its other related functions).
    ///
    /// You may also use [`Self::ecs`] directly if you need more control over
    /// when the ECS is locked.
    ///
    /// # Panics
    ///
    /// This will panic if the component doesn't exist on the client. Use
    /// [`Self::get_component`] to avoid this.
    ///
    /// # Examples
    ///
    /// ```
    /// # use azalea_world::InstanceName;
    /// # fn example(client: &azalea::Client) {
    /// let world_name = client.component::<InstanceName>();
    /// # }
    pub fn component<T: Component>(&self) -> MappedRwLockReadGuard<'_, T> {
        self.get_component::<T>().unwrap_or_else(|| {
            panic!(
                "Our client is missing a required component: {:?}",
                any::type_name::<&T>()
            )
        })
    }

    /// Get a component on this client, or `None` if it doesn't exist.
    ///
    /// If the component is guaranteed to be present, consider using
    /// [`Self::component`]. Also see that function for more details.
    pub fn get_component<T: Component>(&self) -> Option<MappedRwLockReadGuard<'_, T>> {
        self.get_entity_component::<T>(self.entity)
    }

    /// Query the ECS for data from our client entity.
    ///
    /// To query another entity, you can use [`Self::query_entity`].
    ///
    /// You can use this to mutate data on the client.
    ///
    /// # Examples
    ///
    /// ```
    /// # use azalea_entity::Position;
    /// # fn example(mut client: azalea::Client) {
    /// // teleport one block up
    /// client.query_self::<&mut Position, _>(|mut pos| pos.y += 1.0);
    /// # }
    /// ```
    ///
    /// # Panics
    ///
    /// This will panic if the client is missing a component required by the
    /// query.
    pub fn query_self<D: QueryData, R>(&self, f: impl FnOnce(QueryItem<D>) -> R) -> R {
        let mut ecs = self.ecs.write();
        let mut qs = ecs.query::<D>();
        let res = qs.get_mut(&mut ecs, self.entity).unwrap_or_else(|_| {
            panic!(
                "`Client::query_self` failed when querying for {:?}",
                any::type_name::<D>()
            )
        });
        f(res)
    }

    /// Query the ECS for data from an entity.
    ///
    /// Note that it is often simpler to use [`Self::entity_component`].
    ///
    /// To query the client, you should use [`Self::query_self`].
    ///
    /// You can also use this to mutate data on an entity.
    ///
    /// # Panics
    ///
    /// This will panic if the entity doesn't exist or if the query isn't valid
    /// for the entity. For a non-panicking version, you may use
    /// [`Self::try_query_entity`].
    pub fn query_entity<D: QueryData, R>(
        &self,
        entity: Entity,
        f: impl FnOnce(QueryItem<D>) -> R,
    ) -> R {
        self.try_query_entity(entity, f).unwrap_or_else(|_| {
            panic!(
                "Entity is missing a required component {:?}",
                any::type_name::<D>()
            )
        })
    }

    /// A convenience function for getting components from any entity, or None
    /// if the query fails.
    ///
    /// If you're sure that the entity exists and that the query will succeed,
    /// you can use [`Self::query_entity`].
    pub fn try_query_entity<D: QueryData, R>(
        &self,
        entity: Entity,
        f: impl FnOnce(QueryItem<D>) -> R,
    ) -> Result<R, QueryEntityError> {
        let mut ecs = self.ecs.write();
        let mut qs = ecs.query::<D>();
        qs.get_mut(&mut ecs, entity).map(f)
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
    /// use azalea::{
    ///     Client,
    ///     entity::{Position, metadata::Player},
    ///     player::GameProfileComponent,
    /// };
    /// use bevy_ecs::query::With;
    ///
    /// # fn example(mut bot: Client, sender_name: String) {
    /// let entity = bot.any_entity_by::<&GameProfileComponent, With<Player>>(
    ///     |profile: &GameProfileComponent| profile.name == sender_name,
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
    pub fn any_entity_by<Q: QueryData, F: QueryFilter>(
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
    /// # fn example(mut bot: azalea::Client, sender_name: String) {
    /// // get the position of the nearest player
    /// if let Some(nearest_player) =
    ///     bot.nearest_entity_by::<(), (With<Player>, Without<LocalEntity>)>(|_: ()| true)
    /// {
    ///     let nearest_player_pos = **bot.entity_component::<Position>(nearest_player);
    ///     bot.chat(format!("You are at {nearest_player_pos}"));
    /// }
    /// # }
    /// ```
    ///
    /// [`Entity`]: bevy_ecs::entity::Entity
    pub fn nearest_entity_by<Q: QueryData, F: QueryFilter>(
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
    /// # fn example(mut bot: azalea::Client, sender_name: String) {
    /// let nearby_players =
    ///     bot.nearest_entities_by::<(), (With<Player>, Without<LocalEntity>)>(|_: ()| true);
    /// # }
    /// ```
    pub fn nearest_entities_by<Q: QueryData, F: QueryFilter>(
        &self,
        predicate: impl EntityPredicate<Q, F>,
    ) -> Vec<Entity> {
        let Some(instance_name) = self.get_component::<InstanceName>() else {
            return vec![];
        };
        let Some(position) = self.get_component::<Position>() else {
            return vec![];
        };
        let (instance_name, position) = (instance_name.clone(), *position);
        predicate.find_all_sorted(self.ecs.clone(), &instance_name, (&position).into())
    }

    /// Get a component from an entity.
    ///
    /// This allows you to access data stored about entities that isn't
    /// accessible in a simpler way.
    ///
    /// This returns a reference to the component wrapped by a read guard. This
    /// makes the component cheap to access, but means that the ECS cannot be
    /// mutated while it's in scope. In some cases, it may be simpler for you to
    /// immediately clone the component after accessing it.
    ///
    /// If you're trying to get a component for this client, you should use
    /// [`Self::component`] instead.
    ///
    /// To do more complex queries or to mutate data, see
    /// [`Self::query_entity`].
    ///
    /// # Panics
    ///
    /// This will panic if the component doesn't exist on the entity. Use
    /// [`Self::get_entity_component`] to avoid this.
    pub fn entity_component<T: Component>(&self, entity: Entity) -> MappedRwLockReadGuard<'_, T> {
        self.get_entity_component::<T>(entity).unwrap_or_else(|| {
            panic!(
                "Entity {entity} is missing a required component: {:?}",
                any::type_name::<&T>()
            )
        })
    }

    /// Get a component from an entity, if it exists.
    ///
    /// This is similar to [`Self::entity_component`] but returns an `Option`
    /// instead of panicking if the component isn't present.
    pub fn get_entity_component<T: Component>(
        &self,
        entity: Entity,
    ) -> Option<MappedRwLockReadGuard<'_, T>> {
        let ecs = self.ecs.read();
        RwLockReadGuard::try_map(ecs, |ecs: &World| ecs.get(entity)).ok()
    }
}

pub trait EntityPredicate<Q: QueryData, Filter: QueryFilter> {
    fn find_any(
        &self,
        ecs_lock: Arc<RwLock<World>>,
        instance_name: &InstanceName,
    ) -> Option<Entity>;
    fn find_all_sorted(
        &self,
        ecs_lock: Arc<RwLock<World>>,
        instance_name: &InstanceName,
        nearest_to: Vec3,
    ) -> Vec<Entity>;
}
impl<F, Q: QueryData, Filter: QueryFilter> EntityPredicate<Q, Filter> for F
where
    F: Fn(ROQueryItem<Q>) -> bool,
    for<'w, 's> <<Q as QueryData>::ReadOnly as QueryData>::Item<'w, 's>: Copy,
{
    fn find_any(
        &self,
        ecs_lock: Arc<RwLock<World>>,
        instance_name: &InstanceName,
    ) -> Option<Entity> {
        let mut ecs = ecs_lock.write();
        let mut query = ecs.query_filtered::<(Entity, &InstanceName, Q), Filter>();
        query
            .iter(&ecs)
            .find(|(_, e_instance_name, q)| *e_instance_name == instance_name && (self)(*q))
            .map(|(e, _, _)| e)
    }

    fn find_all_sorted(
        &self,
        ecs_lock: Arc<RwLock<World>>,
        instance_name: &InstanceName,
        nearest_to: Vec3,
    ) -> Vec<Entity> {
        let mut ecs = ecs_lock.write();
        let mut query = ecs.query_filtered::<(Entity, &InstanceName, &Position, Q), Filter>();
        let mut entities = query
            .iter(&ecs)
            .filter(|(_, e_instance_name, _, q)| *e_instance_name == instance_name && (self)(*q))
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
